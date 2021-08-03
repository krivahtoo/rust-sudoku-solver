use std::{env, fs, path::PathBuf};

struct Sudoku {
    board: [[u8; 9]; 9]
}

impl Sudoku {
    fn from(content: String) -> Self {
        let mut board: [[u8; 9]; 9] = [[0; 9]; 9];
        content.lines().zip(0..9).for_each(|(line, i)| {
            if line.chars().count() != 9 {
                panic!("Invalid input");
            }
            line.chars().zip(0..9).for_each(|(ch, j)| {
                match ch.to_digit(10) {
                    Some(n) if n <= 9 => board[i][j] = n as u8,
                    _ => board[i][j] = 0,
                }
            });
        });
        Self { board }
    }
    /// Solve the sudoku.
    fn solve(&mut self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if self.board[y][x] == 0 {
                    for n in 1..10 {
                        if self.possible(y,x,n) {
                            self.board[y][x] = n;
                            if self.solve() {
                                return true
                            }
                            self.board[y][x] = 0;
                        }
                    }
                    return false
                }
            }
        }
        true
    }
    /// Check if a value can be inserted into a position.
    fn possible(&self, y: usize, x: usize, n: u8) -> bool {
        for i in 0..9 {
            if self.board[y][i] == n {
                return false
            }
        }
        for j in 0..9 {
            if self.board[j][x] == n {
                return false
            }
        }
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;
        for i in y0..y0 + 3 {
            for j in x0..x0 + 3 {
                if self.board[i][j] == n{
                    return false
                }
            }
        }
        true
    }
    /// Pretty print tha sudoku.
    fn print(&self) {
        println!("┌─────┬─────┬─────┐");
        (0..9).for_each(|y| {
            (0..9).for_each(|x| {
                if x == 0 {
                    print!("│");
                }
                if self.board[y][x] == 0 {
                    print!(".");
                } else {
                    print!("{}", self.board[y][x]);
                }
                if (x+1) % 3 == 0 {
                    print!("│");
                } else {
                    print!(" ");
                }
            });
            println!();
            if y == 2 || y == 5 {
                println!("├─────┼─────┼─────┤");
            }
        });
        println!("└─────┴─────┴─────┘");
    }
}

fn main() {
    let file: PathBuf = PathBuf::from(env::args().nth(1).expect("No file path provided."));
    let content = fs::read_to_string(file).unwrap();
    let mut sudoku = Sudoku::from(content);
    println!("Before solving");
    sudoku.print();
    if sudoku.solve() {
        println!("solved");
        sudoku.print();
    } else {
        println!("Unsolvable");
    }
}
