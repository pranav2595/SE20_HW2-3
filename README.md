[![DOI](https://zenodo.org/badge/289756633.svg)](https://zenodo.org/badge/latestdoi/289756633)

[![Run on Repl.it](https://repl.it/badge/github/pranav2595/SE20_HW2-3)](https://repl.it/github/pranav2595/SE20_HW2-3)

# SE20_HW2-3

This repo has been created for HW2 and HW3 of SE 2020. We implemented [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in [Ruby](https://www.ruby-lang.org/en/), [Rust](https://www.rust-lang.org) and [Go](https://golang.org) with some minor bugs in it. The purpose of this repo is to rank different programming languages in the order of how easy (or hard) it is to debug the code. 

## Conway's Game of Life Rules
GOL is in a 2D square grid, where each cell has two possible states: live or dead. We will be assigned 1 as live cell, and 0 as dead cell. Every cell interacts with its [8 neighboring cells](https://en.wikipedia.org/wiki/Moore_neighborhood) by the following rules: 
- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

For more details refer to [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

## How to Run
You can run our code on [Repl.it](https://repl.it/github/pranav2595/SE20_HW2-3). Read the README.md on each of the directories for detailed instruction. Repl now supports importing repos from Github! 
