module Main exposing (main)

import Element exposing (el, fill, height, layout, text, width)
import Html exposing (Html)


main : Html msg
main =
    el [ width fill, height fill ] none
        |> layout [ width fill, height fill ]
