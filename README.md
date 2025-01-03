# Advent of Code


## Automation

This repo leverages three awesome utilities:
- [cargo-make](https://sagiegurari.github.io/cargo-make)
- [cargo-generate](https://cargo-generate.github.io/cargo-generate)
- [aoc-cli](https://github.com/scarvalhojr/aoc-cli)


## Adding a year
There is no tooling yet.  So, just do the following
- create a folder for the year, and copy the Makefile.toml over:
```sh
mkdir 2025
cp 2024/Makefile.toml 2025
```
- Edit the new Makefile.toml to change `Year = "2024" to the new year.


## Adding a day

Ensure the Makefile.toml has been properly added to the root folder for the year (such as `2024`).

Now simply use `cargo-make` to generate the new day
```sh
cargo make next-day
```

This will execute a python script to figure out what the next day is, and create a couple
temporary files as input to the next set of processes.

`cargo generate` will then be called using the files in the `template` folder to create
the new rust app for the day.

`aoc` will then be used to get the puzzle input for the new day
