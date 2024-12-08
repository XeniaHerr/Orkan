# About
Internal document for holding down some ideas

# Specifications

## Desired Featues

- Select a binary run from the path
- Be able to switch between fuzzy matching and substring matching
- Be able to specify font, color, highlighting



## Desing

Binaries are wrapped in a struct and read into a list. On each keystroke the list is sorted regarding the matching algorithm. The list is then rendered up to the first non matching element.


The left and Top margins are capped by 0.9. Everything else is stupid.



## Notes
- After reading a bit in the documentation of the toolbox, i think i should use the layer-shell for the window. The window should not be movable, nor resizable. It should be on the top layer.

- Fonts can be found with the fontconfig crate. This requires the Fonctonfig library to be installed. I don't know how to handle this right now. (It is in the nix shell, but distribution might be a problem)


## Optimising Rendering

The font rendering right now is highly unoptimized. On every frame every pixel is beeing recalculated and redrawn. 

### Possible Optimisations

- Reuse older BUffers.
 For this to work i will need to be able to make shure that the old buffer is still intact and available to draw onto. I will then also have to deal with the scenario that the buffer is not available yet.

- Reusing Glyphs
I could store a Hashmap of charactes and their corresponding pixle values. This map could be build up while rendering characters. It will probably be better that rerendering everything


### Definetly needed

- Calcualting x position of a glyph based on the characters individially. I shouldn't be dependent on the `font.layout` function for calcualting the pixel bounding boxes. The same function could also handle calculating the render width of a string. This will be needed later for determining if a entry has enough space left to be rendered fully.


## Command Line Arguments

A list of possible options that i may want to support.

- `--left-margin` : int : The left margin of the window
- `--top-magrin` : int : The top margin of the window
- `--font` : str : The font to use
- `--color` : hex : the color with Alpha channel to use for the text
- `--background-color` : hex : the color of the background
- `--highlight-color` : hex : the color of the highlight


# Desing of the Font Optimiser


```rust
struct Optimiser {

font : &Font,

cache : HashMap<char, GlyphMap>
}


struct Glyphmap {

vec<u8> : pixels,

stride : usize,

}
}
```

