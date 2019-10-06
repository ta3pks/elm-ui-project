module Main exposing (main)

import Browser exposing (Document, document)
import Element exposing (el, fill, height, layout, none, text, width)
import Html exposing (Html)


type alias Model =
    ()


main : Program { config : {} } Model msg
main =
    document
        { view = view
        , init = \{ config } -> ( (), Cmd.none )
        , subscriptions = \_ -> Sub.none
        , update = \_ _ -> ( (), Cmd.none )
        }


view : model -> Document msg
view _ =
    { title = "elm-ui"
    , body =
        [ el [ width fill, height fill ] none
            |> layout [ width fill, height fill ]
        ]
    }
