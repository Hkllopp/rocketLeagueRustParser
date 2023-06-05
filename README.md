# rocketLeagueRustParser

This repo is a rust parser for the raw rocket league replay files. It doesn't do any analysis, it is just a simple implementation of the [boxcars](https://crates.io/crates/boxcar) crate that parses the files and store the parsed data in JSON format.

## Installation

You simply need to install rust on your machine. You can follow the instructions on the official website https://www.rust-lang.org/tools/install

## Configuration

You need to store your replays in a specific folder, with a specific extension and the parser will fill a specific output folder with the parsed data in JSON format.
The raw_replays_folder, parsed_replays_folder and replays_extension are defined in the config.toml file. Those folders need to be at the root of the project.

## Usage

To use the parser you can run it from the terminal with the command 
```bash
cargo run
```
or you can build the project with 
```bash
cargo build --release
```
then run the generated executable in the target/release folder. 