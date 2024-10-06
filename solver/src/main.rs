fn parse_sudoku(input: &str, cell_delim: &str, row_delim: &str, group_delim: &str) -> Vec<Vec<u8>> {
    let mut sudoku = Vec::new();

    for group in input.split(group_delim) {
        for row in group.trim().split(row_delim) {
            let cells: Vec<u8> = row
                .trim()
                .split(cell_delim)
                .filter_map(|num| num.trim().parse::<u8>().ok())
                .collect();

            if !cells.is_empty() {
                sudoku.push(cells);
            }
        }
    }

    sudoku
}

fn is_valid_sudoku(sudoku: &[Vec<u8>]) -> bool {
    // Check if each row is valid (no duplicates in non-zero values)
    for row in sudoku {
        if !is_unique(&row) {
            return false;
        }
    }

    // Check if each column is valid (no duplicates in non-zero values)
    for col_idx in 0..sudoku.len() {
        let mut col = Vec::new();
        for row in sudoku {
            col.push(row[col_idx]);
        }
        if !is_unique(&col) {
            return false;
        }
    }

    // Check if each 3x3 subgrid is valid
    for block_row in (0..9).step_by(3) {
        for block_col in (0..9).step_by(3) {
            let mut block = Vec::new();
            for i in 0..3 {
                for j in 0..3 {
                    block.push(sudoku[block_row + i][block_col + j]);
                }
            }
            if !is_unique(&block) {
                return false;
            }
        }
    }

    true
}

fn is_unique(numbers: &[u8]) -> bool {
    let mut seen = vec![false; 10]; // We use a vector to track which numbers (1-9) have been seen

    for &num in numbers {
        if num != 0 {
            if seen[num as usize] {
                return false; // Duplicate found
            }
            seen[num as usize] = true;
        }
    }

    true
}

fn main() {
    // Example input
    let sudoku_str = "9 6 5  8 4 0  0 2 7
         0 3 2  0 0 7  4 0 0
         0 0 1  0 0 9  8 6 3

         1 5 4  9 0 0  0 0 0
         6 0 9  0 5 8  0 0 0
         3 8 7  6 1 4  0 9 0

         0 0 0  7 0 6  5 0 0
         0 7 6  2 0 0  9 3 0
         5 9 8  0 3 0  6 7 0";

    // Parse the input into a 2D grid
    let sudoku = parse_sudoku(sudoku_str, " ", "\n", "\n\n");

    // Check if the Sudoku is valid
    if is_valid_sudoku(&sudoku) {
        println!("The Sudoku grid is valid.");
    } else {
        println!("The Sudoku grid is invalid.");
    }
    // Print the parsed Sudoku grid
    for row in sudoku {
        println!("{:?}", row);
    }
}
