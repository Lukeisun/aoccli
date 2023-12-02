# AOC CLI Helper Tool

Pretty simple [Advent of Code](https://adventofcode.com/) AOC helper tool.

Downloads input file/examples and writes them to your filesystem. Also submits answers from stdin.

To be honest, not sure how useful this will be to others.

## To use

> [!NOTE]
> If no DAY YEAR provided, current date and year will be assumed

This program relies on an environment variable `SESSION` whose value is your session cookie on the AoC website.
You can supply this via a .env file located in your project directory, or through exporting it in your shell.

`aoc DAY YEAR`

Running this will download the test input, scrape the examples, and open the problem's page via xdg-open (will not work on windows)

`aoc submit DAY YEAR PART_NUM`

Running this will submit based on stdin. Usage of this was made in mind of this following workflow.

`cat input.in | {program name} | aoc submit DAY YEAR PART_NUM`

## Stuff I want to do in the future

- ~~Better CLI Parsing~~
- Make the program a little more fun (display some ASCII art on a correct submission? :D)
- Maybe make it a little more cleaner as I learn more about Rust.
-
