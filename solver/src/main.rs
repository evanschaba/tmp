const GRID_SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

struct SudokuSolver {
    board: Vec<Vec<u8>>,
    row_bits: [u16; GRID_SIZE],
    col_bits: [u16; GRID_SIZE],
    box_bits: [u16; GRID_SIZE],
}

impl SudokuSolver {
    fn new(board: Vec<Vec<u8>>) -> Self {
        let mut row_bits = [0u16; GRID_SIZE];
        let mut col_bits = [0u16; GRID_SIZE];
        let mut box_bits = [0u16; GRID_SIZE];

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if board[i][j] != 0 {
                    let num = board[i][j] - 1;

                    row_bits[i] |= 1 << num;
                    col_bits[j] |= 1 << num;
                    box_bits[(i / SUBGRID_SIZE) * SUBGRID_SIZE + j / SUBGRID_SIZE] |= 1 << num;
                }
            }
        }

        Self {
            board,
            row_bits,
            col_bits,
            box_bits,
        }
    }

    fn is_safe(&self, row: usize, col: usize, num: u8) -> bool {
        let num_bit = 1 << (num - 1);
        let box_index = (row / SUBGRID_SIZE) * SUBGRID_SIZE + col / SUBGRID_SIZE;

        (self.row_bits[row] & num_bit) == 0
            && (self.col_bits[col] & num_bit) == 0
            && (self.box_bits[box_index] & num_bit) == 0
    }

    fn place_number(&mut self, row: usize, col: usize, num: u8) {
        let num_bit = 1 << (num - 1);
        let box_index = (row / SUBGRID_SIZE) * SUBGRID_SIZE + col / SUBGRID_SIZE;

        self.row_bits[row] |= num_bit;
        self.col_bits[col] |= num_bit;
        self.box_bits[box_index] |= num_bit;

        self.board[row][col] = num;
    }

    fn remove_number(&mut self, row: usize, col: usize, num: u8) {
        let num_bit = 1 << (num - 1);
        let box_index = (row / SUBGRID_SIZE) * SUBGRID_SIZE + col / SUBGRID_SIZE;

        self.row_bits[row] &= !num_bit;
        self.col_bits[col] &= !num_bit;
        self.box_bits[box_index] &= !num_bit;

        self.board[row][col] = 0;
    }

    fn solve(&mut self) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.board[row][col] == 0 {
                    for num in 1..=9 {
                        if self.is_safe(row, col, num) {
                            self.place_number(row, col, num);
                            if self.solve() {
                                return true;
                            }
                            self.remove_number(row, col, num);
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
}

fn parse_sudoku(input: &str) -> Vec<Vec<u8>> {
    let mut sudoku = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    let rows: Vec<&str> = input.trim().lines().collect();

    if rows.len() != GRID_SIZE {
        panic!("Invalid number of rows in input.");
    }

    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<&str> = row.split_whitespace().collect();
        if cells.len() != GRID_SIZE {
            panic!("Invalid number of columns in row {}.", i + 1);
        }

        for (j, cell) in cells.iter().enumerate() {
            if let Ok(num) = cell.parse::<u8>() {
                if j < GRID_SIZE {
                    sudoku[i][j] = num;
                }
            }
        }
    }

    sudoku
}

fn print_sudoku(sudoku: &Vec<Vec<u8>>) {
    for (i, row) in sudoku.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!();
        }
        for (j, cell) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 {
                print!("  ");
            }
            print!("{} ", cell);
        }
        println!();
    }
}
fn main() {
    let sudoku_str = "9 6 5 8 4 0 0 2 7
         0 3 2 0 0 7 4 0 0
         0 0 1 0 0 9 8 6 3
         1 5 4 9 0 0 0 0 0
         6 0 9 0 5 8 0 0 0
         3 8 7 6 1 4 0 9 0
         0 0 0 7 0 6 5 0 0
         0 7 6 2 0 0 9 3 0
         5 9 8 0 3 0 6 7 0";

    let sudoku = parse_sudoku(sudoku_str);
    let mut solver = SudokuSolver::new(sudoku);

    if solver.solve() {
        println!("Solved Sudoku:");
        print_sudoku(&solver.board);
    } else {
        println!("No solution found.");
    }
}
