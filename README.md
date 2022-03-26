# sadoku-web

An experiment with porting my sudoku puzzle engine from [sadoku] into a
[Yew]-based application running entire in WebAssembly.

To run it locally:

    cargo build
    cargo install trunk
    trunk serve

## Current Status

A Puzzle object is rendered as a nested series of Components (think React)
written entirely in Rust.

## Initial Takeaways

* The development loop is really rewarding, much like developing other React
  applications.
* For my small application, compile times were extremely fast and did not get
  in the way of flow.
* CLion's Rust plugin, which is very awesome, lacks visibility into identifiers
  within macros that use token stream parsing, so it cannot offer code
  suggestions when accessing a context property and using a method on it. A
  minor annoyance, error display works flawlessly so it's clear when you mess
  this up.

[sadoku]: https://github.com/askreet/sadoku
[Yew]: https://yew.rs/
