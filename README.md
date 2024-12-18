
# About

Orkan ist a dmenu like tool to launch Programms. It is mostly written in Rust and only supports the Wayland Protocol. X11 is not planned, use dmenu. Also, it's written my me for me, so don't expect that i will be very active on issues and merge requests. But feel free to open them, i will look at them.




# Read

- [dmenu](https://tools.suckless.org/dmenu/)
- [wayland-toolkit-example](https://github.com/Smithay/client-toolkit/blob/master/examples/simple_window.rs)


# Dependencies
This will use the following dependencies:

## Smithay-client-toolkit
For handling the wayland protocol

## Wayland-client
Needed Wayland Interface

## Rusttype
For Font rendering. There wont be anything else rendered, so this is the only dependency for rendering

## Fontconfig
Used for determining the absolute path to a font.




# Roadmap
---

- [x] List all Binaries
- [x] Draw Window
- [x] Show search results
- [x] Specify Possible Program Arguments
- [x] Render Text in Window
- [ ] Optimise Rendering
- [x] Handle Keyboard Input
- [ ] Handle Select
- [x] Launch Program




