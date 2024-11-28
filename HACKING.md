# About
Internal document for holding down some ideas

# Specifications

## Desired Featues

- Select a binary run from the path
- Be able to switch between fuzzy matching and substring matching
- Be able to specify font, color, highlighting



## Desing

Binaries are wrapped in a struct and read into a list. On each keystroke the list is sorted regarding the matching algorithm. The list is then rendered up to the first non matching element.
