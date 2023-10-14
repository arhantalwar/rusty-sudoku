fn main() {

    let mut board: [[u8; 9]; 9] = [
        [0, 7, 0, 0, 2, 0, 0, 4, 6],
        [0, 6, 0, 0, 0, 0, 8, 9, 0],
        [2, 0, 0, 8, 0, 0, 7, 1, 5],
        [0, 8, 4, 0, 9, 7, 0, 0, 0],
        [7, 1, 0, 0, 0, 0, 0, 5, 9],
        [0, 0, 0, 1, 3, 0, 4, 8, 0],
        [6, 9, 7, 0, 0, 2, 0, 0, 8],
        [0, 5, 8, 0, 0, 0, 0, 6, 0],
        [4, 3, 0, 0, 8, 0, 0, 0, 0],
    ];

        let _is_possible = possible(&board, 0, 0, 2);
        _print_board(board);

        if solve(&mut board) {
            _print_board(board);
        } else {
            println!("No solution found.");
        }
        

}

fn solve(board: &mut [[u8; 9]; 9]) -> bool {

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 {
                for z in 1..10 {
                    if possible(board, i, j, z) == true {
                        board[i][j] = z;
                        if solve(board) == true {
                            return true;
                        }
                        board[i][j] = 0;
                    }
                }
                return false;
            }
        }
    }

    return true;
    
}

fn possible(board: &[[u8; 9]; 9], row: usize, col: usize, number: u8) -> bool {


    // ROW CHECK
    for i in 0..9 {
        if board[row][i] == number {
            return false;
        }
    }

    // COL CHECK
    for i in 0..9 {
        if board[i][col] == number {
            return false;
        }
    }

    let grid_x = (row / 3) * 3;
    let grid_y = (col / 3) * 3;

    for i in 0..3 {
        for j in 0..3 {
            if board[grid_x + i][grid_y + j] == number {
                return false;
            }
        }
    }

    true
    
}

fn _print_board(board: [[u8; 9]; 9]) {

    for i in 0..9 {
        for j in board[i] {
            print!("{} ", j);
        }
        print!("\n");
    }

    print!("\n");
    
}
