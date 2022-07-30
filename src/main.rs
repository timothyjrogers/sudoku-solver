use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

struct Board {
    rows: [[char; 9]; 9],
}

impl Board {
    fn new(rows: [[char; 9]; 9]) -> Self {
        Board { rows }
    }

    fn new_from_file(fname: &str) -> Self {
        let f = File::open(fname).expect("Unable to open data file");
        let reader = BufReader::new(f);
        let lines: Vec<String> = reader
            .lines()
            .collect::<Result<_, _>>()
            .unwrap();
        let mut rows = [['0'; 9]; 9];
        for (i, line) in lines.iter().enumerate() {
            for (j, val) in line.chars().enumerate() {
                rows[i][j] = val;
            }
        }
        Self { rows }
    }

    fn get_square(&self, row: usize, pos: usize) -> char {
        return self.rows[row][pos];
    }

    fn set_square(&mut self, row: usize, pos: usize, val: char) {
        self.rows[row][pos] = val;
    }

    fn get_square_options(&self, row: usize, pos: usize) -> Vec<char> {
        if self.rows[row][pos] != '_' { return vec![] }
        let mut options = vec!['1','2','3','4','5','6','7','8','9'];
        let mut used = vec![];
        let base_row = 3 * (row / 3);
        let base_col = 3 * (pos / 3);
        for square in self.rows[row] {
            used.push(square);
        }
        for i in 1..9 {
            used.push(self.rows[i][pos]);
        }
        for i in base_row..base_row+3 {
            for j in base_col..base_col+3 {
                used.push(self.rows[i][j]);
            }
        }
        options.retain(|val| !used.contains(val));
        return options;
    }

    fn validate_square(&self, row: usize, pos: usize) -> bool {
        if (self.rows[row][pos] == '_') { return false }
        let rowset: HashSet<char> = self.rows[row].into_iter().collect();
        if (rowset.len() < 9 || rowset.contains(&'_')) { return false; }
        let colset: HashSet<char> = [0,1,2,3,4,5,6,7,8].map(|i| self.rows[i][pos]).into_iter().collect();
        if (colset.len() < 9 || colset.contains(&'_')) { return false; }
        let mut quadset: HashSet<char> = HashSet::new();
        let base_row = 3 * (row / 3);
        let base_col = 3 * (pos / 3);
        for i in base_row..base_row+3 {
            for j in base_col..base_col+3 {
                quadset.insert(self.rows[i][j]);
            }
        }
        if (quadset.len() < 9 || quadset.contains(&'_')) { return false; }
        return true;
    }

    fn solved(&self) -> bool {
        for row in 0..9 {
            for square in 0..9 {
                if !self.validate_square(row, square) { return false; }
            }
        }
        return true;
    }

    fn get_next_open(&self) -> (usize,usize) {
        for i in 0..9 {
            for j in 0..9 {
                if (self.rows[i][j] == '_') { return (i,j) }
            }
        }
        return (0,0);
    }

    fn print(&self) {
        println!("{}", self.rows.map(|row| row.map(|square| square.to_string()).join(" ")).join("\n"));
    }
}

fn solve(board: Board) -> bool {
    if (board.solved()) {
        board.print();
        return true;
    }
    let next = board.get_next_open();
    let mut options = board.get_square_options(next.0, next.1);
    for option in options {
        let mut next_board = Board::new(board.rows);
        next_board.set_square(next.0, next.1, option);
        if (solve(next_board)) { return true; }
    }
    return false;
}

fn main() {
    let board = Board::new_from_file("test.txt");
    solve(board);
}
