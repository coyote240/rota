# rota - A Tarot Application

## Why?

As I migrate from an iPhone8 to a [Community Edition
PinePhone](https://pine64.com/) and [Mobian](https://mobian-project.org/), I am
replacing various apps with those developed by the community, or my own. In the
case of `rota`, I want a Gnome app that will let me draw a single card daily,
logging the results (with metrics?) and perhaps more advanced spreads in the
future.

This is my first real Rust application, and will be written using GTK+.

## Assets

rota is based on the public domain Rider-Waite-Smith tarot deck, and the
publicly availailable dataset provided by [Temple ov thee
Lemur](https://totl.net/Menu/Random/). Extra points to them for the TOPI
dialect and affectations.

In order to generate JSON based upon the RDF provided by TOTL, the `gen_assets`
program has been provided. Running `gen_assets` from the project root will
result in the output file being written to the `assets/` folder. This is
included in the application binary at compile time.

```shell
cargo run --bin gen_assets
```
