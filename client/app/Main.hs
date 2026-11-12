-----------------------------------------------------------------------------
{-# LANGUAGE CPP               #-}
{-# LANGUAGE DeriveGeneric     #-}
{-# LANGUAGE LambdaCase        #-}
{-# LANGUAGE OverloadedStrings #-}
-----------------------------------------------------------------------------
module Main where
-----------------------------------------------------------------------------
import           GHC.Generics (Generic)
import           Miso
import           Miso.Html.Element as H
import           Miso.Html.Event as E
import           Miso.Html.Property as P
import           Miso.JSON (FromJSON)
import qualified Miso.CSS as CSS
import           Miso.CSS (StyleSheet)
-----------------------------------------------------------------------------
-- | Base URL of the Servant server (see server/app/Main.hs).
apiBase :: MisoString
apiBase = "http://127.0.0.1:3000/api"
-----------------------------------------------------------------------------
-- | Mirrors 'Api.Location' on the server. Field names are the JSON keys,
-- so keep the two in sync.
data Location = Location
  { locationId :: Int
  , name       :: MisoString
  , address    :: MisoString
  } deriving (Show, Eq, Generic)

instance FromJSON Location
-----------------------------------------------------------------------------
data Model = Model
  { greeting  :: Maybe MisoString
  , locations :: Maybe [Location]
  , apiError  :: Maybe MisoString
  } deriving (Show, Eq)

initialModel :: Model
initialModel = Model Nothing Nothing Nothing
-----------------------------------------------------------------------------
data Action
  = FetchAll
  | FetchHello
  | FetchLocations
  | GotHello (Response MisoString)
  | GotLocations (Response [Location])
  | RequestFailed (Response MisoString)
-----------------------------------------------------------------------------
#ifdef WASM
#ifndef INTERACTIVE
foreign export javascript "hs_start" main :: IO ()
#endif
#endif
-----------------------------------------------------------------------------
main :: IO ()
#ifdef INTERACTIVE
main = reload defaultEvents app
#else
main = startApp defaultEvents app
#endif
-----------------------------------------------------------------------------
app :: App Model Action
app = (component initialModel updateModel viewModel)
  { styles = [ Sheet sheet ]
  , mount  = Just FetchAll
  }
-----------------------------------------------------------------------------
updateModel :: Action -> Effect parent props Model Action
updateModel = \case
  FetchAll -> do
    issue FetchHello
    issue FetchLocations
  FetchHello ->
    getText (apiBase <> "/hello") [] GotHello RequestFailed
  FetchLocations ->
    getJSON (apiBase <> "/locations") [] GotLocations RequestFailed
  GotHello resp ->
    modify $ \m -> m { greeting = Just (body resp) }
  GotLocations resp ->
    modify $ \m -> m { locations = Just (body resp) }
  RequestFailed resp -> do
    let msg = describeError resp
    io_ (consoleError msg)
    modify $ \m -> m { apiError = Just msg }
-----------------------------------------------------------------------------
-- | A request with no status never reached the server (server down, CORS,
-- network error); otherwise report the status and any decoding message.
describeError :: Response MisoString -> MisoString
describeError resp = case status resp of
  Nothing -> "Could not reach the API at " <> apiBase
           <> maybe "" (" — " <>) (errorMessage resp)
  Just code -> "API request failed (" <> ms code <> ")"
           <> maybe "" (": " <>) (errorMessage resp)
-----------------------------------------------------------------------------
viewModel :: Model -> View context props Model Action
viewModel m = H.div_
  [ P.class_ "container" ]
  [ H.h1_ [ P.class_ "title" ] [ "Upstate NY Happy Hours" ]
  , H.p_ [ P.class_ "greeting" ]
    [ text (maybe "Contacting server…" id (greeting m)) ]
  , maybe (text "") viewError (apiError m)
  , viewLocations (locations m)
  , H.button_ [ E.onClick FetchAll, P.class_ "refresh-btn" ] [ "Refresh" ]
  ]

viewError :: MisoString -> View context props Model Action
viewError msg = H.p_ [ P.class_ "error" ] [ text msg ]

viewLocations :: Maybe [Location] -> View context props Model Action
viewLocations = \case
  Nothing -> H.p_ [] [ "Loading locations…" ]
  Just [] -> H.p_ [] [ "No locations yet." ]
  Just ls -> H.ul_ [ P.class_ "locations" ] (map viewLocation ls)

viewLocation :: Location -> View context props Model Action
viewLocation loc = H.li_ [ P.class_ "location" ]
  [ H.strong_ [] [ text (name loc) ]
  , H.span_ [ P.class_ "address" ] [ text (address loc) ]
  ]
-----------------------------------------------------------------------------
sheet :: StyleSheet
sheet =
  CSS.sheet_
  [ CSS.selector_ ":root"
    [ "--primary-color" =: "#4a6bff"
    , "--primary-hover" =: "#3451d1"
    , "--error-color" =: "#d13451"
    , "--background" =: "#f7f9fc"
    , "--text-color" =: "#333"
    , "--muted" =: "#666"
    ]
  , CSS.selector_ "body"
    [ CSS.fontFamily "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif"
    , CSS.display "flex"
    , CSS.justifyContent "center"
    , CSS.alignItems "center"
    , CSS.minHeight "100vh"
    , CSS.margin "0"
    , CSS.backgroundColor (CSS.var "background")
    , CSS.color (CSS.var "text-color")
    ]
  , CSS.selector_ ".container"
    [ CSS.backgroundColor CSS.white
    , CSS.padding (CSS.rem 2)
    , CSS.borderRadius (CSS.px 12)
    , CSS.boxShadow "0 4px 10px rgba(0, 0, 0, 0.1)"
    , CSS.width "min(32rem, calc(100vw - 2rem))"
    , CSS.boxSizing "border-box"
    ]
  , CSS.selector_ ".greeting"
    [ CSS.color (CSS.var "muted")
    ]
  , CSS.selector_ ".error"
    [ CSS.color (CSS.var "error-color")
    , CSS.fontWeight "bold"
    ]
  , CSS.selector_ ".locations"
    [ "list-style" =: "none"
    , CSS.padding "0"
    ]
  , CSS.selector_ ".location"
    [ CSS.display "flex"
    , CSS.flexDirection "column"
    , CSS.padding "0.75rem 0"
    , CSS.borderBottom "1px solid #eee"
    ]
  , CSS.selector_ ".address"
    [ CSS.color (CSS.var "muted")
    , CSS.fontSize (CSS.rem 0.9)
    ]
  , CSS.selector_ ".refresh-btn"
    [ CSS.marginTop (CSS.rem 1)
    , CSS.padding "0.5rem 1.25rem"
    , CSS.border "none"
    , CSS.borderRadius (CSS.px 6)
    , CSS.cursor "pointer"
    , CSS.color CSS.white
    , CSS.backgroundColor (CSS.var "primary-color")
    ]
  , CSS.selector_ ".refresh-btn:hover"
    [ CSS.backgroundColor (CSS.var "primary-hover")
    ]
  ]
-----------------------------------------------------------------------------
