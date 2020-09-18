# TODO

* game over if clicked on bomb
    * show bombs
    * message with modal?
* tiles must know how many neighbouring bombs
    * when clicked: show number
* if tile clicked, then neighoubring non-bomb tiles without bomb neighbours are also shown
    * propagation
* game won if all tiles except bombs have been clicked
* only show bombs when game won or game over

# Componenten

* Home  
    * Clock
    * Smiley -> win/verlies
        * NewGameInput
    * HowManyBombsMinusFlags
    * Board 
        * Tile


# Inspiration

SPA Rust+ASM: http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/

Timer: https://github.com/yewstack/yew/blob/v0.17/examples/timer/src/lib.rs

Yew Examples: https://github.com/yewstack/yew/tree/v0.17/examples

* timer
* counter
* large_table

