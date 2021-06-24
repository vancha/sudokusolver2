extern crate gio;
extern crate gtk;

// To import all needed traits.
use gio::prelude::*;
use gtk::prelude::*;

const BLOCK_SIZE:i32 = 3;


fn solve(board: &mut Vec<Vec<i32>>) -> bool {
    for row in 0..(BLOCK_SIZE*BLOCK_SIZE) {
        for column in 0..(BLOCK_SIZE*BLOCK_SIZE) {
            if board[row as usize][column as usize] == 0 {
                for proposed_value in 1..(BLOCK_SIZE*BLOCK_SIZE)+1 {
                    if is_valid(board, row, column, proposed_value) {
                        board[row as usize][column as usize] = proposed_value;
                        if solve(board) {
                            return true;
                        }
                        board[row as usize][column as usize] = 0;
                    }
                }
                return false;
            }
        }
    }
    return true;
}

fn is_valid(board: &Vec<Vec<i32>>, r: i32, c: i32, proposed_value: i32) -> bool {
    for row in 0..(BLOCK_SIZE*BLOCK_SIZE) {
        if board[row as usize][c as usize] == proposed_value {
            return false;
        }
    }

    for col in 0..(BLOCK_SIZE*BLOCK_SIZE) {
        if board[r as usize][col as usize] == proposed_value {
            return false;
        }
    }

    for row in ((r / BLOCK_SIZE) * BLOCK_SIZE)..((r / BLOCK_SIZE + 1) * BLOCK_SIZE) {
        for col in ((c / BLOCK_SIZE) * BLOCK_SIZE)..((c / BLOCK_SIZE + 1) * BLOCK_SIZE) {
            if board[row as usize][col as usize] == proposed_value {
                return false;
            }
        }
    }
    return true;
}



fn main() {
    let uiapp = gtk::Application::new(
        Some("org.example.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    uiapp.connect_activate(|app| {
        //a 2d grid of entries, that the user will type their sudoku numbers in
        let mut sudo_list: Vec<Vec<gtk::Entry>> = vec![];
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("sudoku thingy");
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 6);
        for row in 0..(BLOCK_SIZE*BLOCK_SIZE) {
            let rowbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            //this entry model is going to be pushed to the list, representing the horizontal (rows) lists of buttons
            let mut entry_model: Vec<gtk::Entry> = vec![];
            //now for every row, we'll loop 9 times to create an entry for every column
            for _ in 0..(BLOCK_SIZE*BLOCK_SIZE) {
                let b = gtk::Entry::new();
                b.set_text("0");
                b.set_width_chars(2);
                entry_model.push(b.clone());
                rowbox.add(&b);
            }
            layout.add(&rowbox);
            //add the entries to the 2d grid
            sudo_list.push(entry_model);

            if row == (BLOCK_SIZE*BLOCK_SIZE)-1 {
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
                                .text()
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
    uiapp.run();
}
