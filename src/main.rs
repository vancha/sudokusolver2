extern crate gio;
extern crate gtk;

// To import all needed traits.
use gio::prelude::*;
use gtk::prelude::*;

use std::env;


fn solve(board: &mut Vec<Vec<i32>>) -> bool {
    for r in 0i32..9 {
        for c in 0i32..9 {
            if board[r as usize][c as usize] == 0 {
                for d in 1..10 {
                    if is_valid(board, r, c, d) {
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

fn is_valid(board: &Vec<Vec<i32>>, r: i32, c: i32, d: i32) -> bool {
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



fn main() {
    let uiapp = gtk::Application::new(
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        let mut sudo_list: Vec<Vec<gtk::Entry>> = vec![];
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_title("Basic example");
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 6);
        for row in 0..9 {
            let rowbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            let mut entry_model: Vec<gtk::Entry> = vec![];
            for _ in 0..9 {
                let b = gtk::Entry::new();
                b.set_text("0");
                b.set_width_chars(2);
                entry_model.push(b.clone());
                rowbox.add(&b);
            }

            layout.add(&rowbox);
            sudo_list.push(entry_model);

            if row == 8 {
                let calculate_button = gtk::Button::new();
                calculate_button.set_label("Solve!");
                layout.add(&calculate_button);

                let sl2 = sudo_list.clone();
                calculate_button.connect_clicked(move |_| {
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

                    for (i1, x) in sl2.iter().enumerate() {
                        for (i2, y) in x.iter().enumerate() {
                            board[i1][i2] = y
                                .clone()
                                .downcast::<gtk::Entry>()
                                .unwrap()
                                .get_text()
                                .as_str()
                                .parse::<i32>()
                                .unwrap();
                        }
                    }
                    if solve(&mut board) {
                        //do this on a thread, show a spinner

                        for (i1, x) in sl2.iter().enumerate() {
                            for (i2, y) in x.iter().enumerate() {
                                y.clone()
                                    .downcast::<gtk::Entry>()
                                    .unwrap()
                                    .set_text((board[i1][i2]).to_string().as_str());
                            }
                        }
                    } else {
                       if gtk::init().is_err() {
                            println!("Failed to initialize GTK.");
                            return;
                        }
                        gtk::MessageDialog::new(None::<&gtk::Window>,
                                           gtk::DialogFlags::empty(),
                                           gtk::MessageType::Info,
                                           gtk::ButtonsType::Ok,
                                           "Hello World").run();
                    }
                });
            }
        }
        win.add(&layout);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
