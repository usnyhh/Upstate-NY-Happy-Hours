module Main where

import           Data.Function            ((&))
import           Data.String              (fromString)
import           Network.Wai.Handler.Warp (defaultSettings, runSettings,
                                           setHost, setPort)

import           Api                      (app)

host :: String
host = "127.0.0.1"

port :: Int
port = 3000

main :: IO ()
main = do
  putStrLn $ "API server running on http://" <> host <> ":" <> show port
  runSettings (defaultSettings & setHost (fromString host) & setPort port) app
