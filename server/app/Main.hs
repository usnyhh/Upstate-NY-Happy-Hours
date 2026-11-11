{-# LANGUAGE DataKinds         #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TypeOperators     #-}
module Main where

import           Data.Function                  ((&))
import           Data.Text                      (Text)
import           Network.HTTP.Types             (status200)
import           Network.Wai                    (responseFile)
import           Network.Wai.Application.Static (defaultFileServerSettings,
                                                 ss404Handler)
import           Network.Wai.Handler.Warp       (defaultSettings, runSettings,
                                                 setHost, setPort)
import           Servant
import           System.FilePath                ((</>))

-- | JSON/text endpoints live under /api; everything else is the miso client.
type Api =
       "api" :> ApiRoutes
  :<|> Raw

type ApiRoutes =
       "hello" :> Get '[PlainText] Text

-- | Where `make build` / `miso build` put the compiled client.
staticDir :: FilePath
staticDir = "../client/public"

apiServer :: Server ApiRoutes
apiServer = pure "Hello from Servant API!"

-- | Serve the client's files, falling back to index.html so that
-- client-side routes survive a page refresh.
frontend :: Server Raw
frontend = serveDirectoryWith settings
  where
    settings = (defaultFileServerSettings staticDir)
      { ss404Handler = Just indexHtml }

indexHtml :: Application
indexHtml _ sendResponse = sendResponse $
  responseFile status200 [("Content-Type", "text/html; charset=utf-8")]
    (staticDir </> "index.html") Nothing

app :: Application
app = serve (Proxy :: Proxy Api) (apiServer :<|> frontend)

main :: IO ()
main = do
  putStrLn "Server running on http://localhost:3000"
  runSettings (defaultSettings & setHost "127.0.0.1" & setPort 3000) app
