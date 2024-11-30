# About
Internal document for holding down some ideas

# Specifications

## Desired Featues

- Select a binary run from the path
- Be able to switch between fuzzy matching and substring matching
- Be able to specify font, color, highlighting



## Desing

Binaries are wrapped in a struct and read into a list. On each keystroke the list is sorted regarding the matching algorithm. The list is then rendered up to the first non matching element.



## Notes
- After reading a bit in the documentation of the toolbox, i think i should use the layer-shell for the window. The window should not be movable, nor resizable. It should be on the top layer.
