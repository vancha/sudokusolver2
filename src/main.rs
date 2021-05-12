fn solve(board: &mut Vec<Vec<i32>>) -> bool {
    for r in 0i32..9 {
        for c in 0i32..9 {
            if board[r as usize][c as usize] == 0 {
                for d in 1..10 {
                    if isValid(board, r, c, d) {
                        board[r as usize][c as usize] = d;
                        if solve(board) {
                            return true;
                        }
                        board[r as usize][c as usize] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn isValid(board: &Vec<Vec<i32>>, r: i32, c: i32, d: i32) -> bool {
    for row in 0..9 {
        if board[row as usize][c as usize] == d {
            return false;
        }
    }

    for col in 0..9 {
        if board[r as usize][col as usize] == d {
            return false;
        }
    }

    for row in ((r / 3) * 3)..((r / 3 + 1) * 3) {
        for col in ((c / 3) * 3)..((c / 3 + 1) * 3) {
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



/*
fn main() {
    let mut board = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 6],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    println!("{:?}", solve(&mut board));
    for x in board.clone() {
        println!("{:?}", x);
    }
}*/

extern crate gtk;
extern crate gio;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_title("Basic example");
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 6);
        for row in 0..9 {
            let row = gtk::Box::new(gtk::Orientation::Horizontal,6);
            for column in 0..9 {
                //let column = gtk::Box::new(gtk::Orientation::Horizontal, 16);
                let b = gtk::Entry::new();
                b.set_width_chars(2);
                row.add(&b);
            }
            layout.add(&row);
        }
        win.add(&layout);
        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
