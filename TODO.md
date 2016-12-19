* derive PartialEq for GamePlay and others
* battleplanes-rust should not depend on iron
* more logging, error logging, use [error-chain](https://crates.io/crates/error-chain)
* build script via cargo for assets
* profile the RAM usage
* reduce memory usage
* use Arc
* make highlighted-temp-tile half transparent
* use https://github.com/iron/params/blob/master/examples/params.rs
* use https://crates.io/crates/crossbeam
* more exhaustive testing
  * review usage of pub, make pub only what's necessary to create new
    interfaces
  * move common gameplay flows from the bins to the lib
  * test REST endpoints
* create getters with https://github.com/emk/accessors if racer can deal with
  it
* implement REST endpoints
* build scripts to embed various frontends into the binary, akin to
  https://github.com/vitiral/rst
