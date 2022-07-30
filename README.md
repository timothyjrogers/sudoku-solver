# sudoku-solver

This a basic Sudoku puzzle solver written in Rust. It uses recursive backtracking to find the first valid solution to the puzzle. The code here was written as a refresher exercise and is not necessarily tidy.

## Running the solver

To run the solver first place your puzzle input in a file named `test.txt` in the working directory. Empty cells should be represented with underscores, cells should not have spaces between them, and each row of the puzzle should be on a new line. For example:

```
______1__
__1_476__
_6_____5_
_________
___3_57_8
4__29____
7_31____6
__8_6_4_1
____2___9
```

Then execute `cargo run`.

## Constraints

* Only 9x9 puzzles are supported
* Only returns the first puzzle solution found
* Assumes the puzzle is solvable, may timeout on random puzzles
