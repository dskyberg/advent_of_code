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

The `next-day` task will do several things.
- Leverage `cargo-generate` to copy the `template` folder to the new day
- Leverage  `aoc-cli` to pull down the day's puzzle input
- Run some python code to create the src/lib.rs file, which will contain the input as static text
- Leverage `cargo-make` to add common crates and do a test build
- Clean up temp files
- Lauch the Zed editor for both the `aoc_utils` and new day projects.
