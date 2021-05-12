fn solve(board:&mut Vec<Vec<i32>>)-> bool {
    for r in 0i32..9 {
        for c in 0i32..9 {
            if board[r as usize][c as usize] == 0 {
                for d in 1..10 {
                    if isValid(board, r,c,d) {
                        board[r as usize][c as usize] = d;
                        if solve(board) {
                            return true
                        } 
                        board[r as usize][c as usize] = 0;
                    }
                }
                return false
            }
        }
    }
    true
}

fn isValid(board: &Vec<Vec<i32>>, r:i32,c:i32,d:i32)->bool {
    for row in 0..9 {
        if board[row as usize][c as usize] == d {
            return false;
        }
    }

    for col in 0..9 {
        if board[r as usize][col as usize] == d {
            return false
        }
    }
    
    for row in ((r/3)*3)..((r/3+1)*3){
        for col in ((c / 3)*3)..((c/3 +1) *3) {
            if board[row as usize][col as usize] == d {
                return false;
            }
        }
    }

    return true;
}

fn solveSudoku(board: &mut Vec<Vec<i32>>) {
    solve(board);
}

fn main() {
    let mut board = vec![
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,6],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        ];

    println!("{:?}",solve(&mut board));
    for x in board.clone() {
        println!("{:?}",x);
    }
}
