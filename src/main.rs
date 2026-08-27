use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, Hide, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

fn draw_board(snake_col: i32) {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        MoveTo(0, 0),
        Clear(ClearType::All)
    )
    .unwrap();

    for row in 0..10 {
        for column in 0..21 {
            if row == 0 || row == 9 {
                print!("#");
            } else if column == 0 || column == 20 {
                print!("#");
            } else if row == 3 && column == snake_col {
                print!(">");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    stdout.flush().unwrap();
}

fn main() {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide
    )
    .unwrap();

    let mut snake_col = 1;

    for _ in 0..20 {
        draw_board(snake_col);

        if snake_col < 19 {
            snake_col += 1;
        }

        thread::sleep(Duration::from_millis(300));
    }

    execute!(
        stdout,
        Show,
        LeaveAlternateScreen
    )
    .unwrap();
}