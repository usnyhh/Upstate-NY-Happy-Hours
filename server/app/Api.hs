{-# LANGUAGE DataKinds         #-}
{-# LANGUAGE DeriveGeneric     #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TypeOperators     #-}
-- | The HTTP API. This server only exposes data endpoints; it does not
-- serve, build, or otherwise know about the client. The client is served
-- from a different origin, so the app is wrapped in a CORS middleware.
module Api
  ( Api
  , Location (..)
  , api
  , server
  , app
  ) where

import           Data.Aeson                  (FromJSON, ToJSON)
import           Data.Text                   (Text)
import           GHC.Generics                (Generic)
import           Network.Wai.Middleware.Cors (CorsResourcePolicy (..),
                                              cors, simpleCorsResourcePolicy)
import           Servant

-- | Mirrors the @locations@ table. Field names are the JSON keys, so they
-- must match the client's 'Location' type.
data Location = Location
  { locationId :: Int
  , name       :: Text
  , address    :: Text
  } deriving (Show, Eq, Generic)

instance ToJSON Location
instance FromJSON Location

type Api = "api" :> ApiRoutes

type ApiRoutes =
       "hello"     :> Get '[PlainText] Text
  :<|> "locations" :> Get '[JSON] [Location]

api :: Proxy Api
api = Proxy

server :: Server Api
server = hello :<|> locations

hello :: Handler Text
hello = pure "Hello from Servant API!"

-- | Placeholder data until the database is wired up.
locations :: Handler [Location]
locations = pure
  [ Location 1 "Example Tavern" "123 Main St, Albany, NY"
  , Location 2 "Sample wherehouse" "456 State St, Troy, NY"
  ]

app :: Application
app = cors (const (Just corsPolicy)) (serve api server)

-- | Allow any origin (fine for local dev; restrict before deploying) and
-- the Content-Type header so JSON POSTs pass the preflight check.
corsPolicy :: CorsResourcePolicy
corsPolicy = simpleCorsResourcePolicy
  { corsMethods        = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
  , corsRequestHeaders = ["Content-Type"]
  }
