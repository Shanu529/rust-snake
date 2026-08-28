use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn draw_board(
    snake_row: i32,
    snake_col: i32,
    apple_row: i32,
    apple_col: i32,
) {
    let mut stdout = io::stdout();

    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();

    for row in 0..10 {
        for column in 0..21 {
            if row == 0 || row == 9 {
                print!("#");
            } else if column == 0 || column == 20 {
                print!("#");
            } else if row == snake_row && column == snake_col {
                print!(">");
            } else if row == apple_row && column == apple_col {
                print!("🍎");
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

    let mut snake_row = 3;
    let mut snake_col = 1;

    let apple_row = 3;
    let apple_col = 10;

    let mut direction = Direction::Right;

    loop {
        draw_board(
            snake_row,
            snake_col,
            apple_row,
            apple_col,
        );

        // Check keyboard input
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => direction = Direction::Up,
                    KeyCode::Down => direction = Direction::Down,
                    KeyCode::Left => direction = Direction::Left,
                    KeyCode::Right => direction = Direction::Right,
                    _ => {}
                }
            }
        }

        // Move snake
        match direction {
            Direction::Right => {
                snake_col += 1;
            }

            Direction::Left => {
                snake_col -= 1;
            }

            Direction::Up => {
                snake_row -= 1;
            }

            Direction::Down => {
                snake_row += 1;
            }
        }

        // Check if snake hits wall
        if snake_col == 0
            || snake_col == 20
            || snake_row == 0
            || snake_row == 9
        {
            break;
        }

        thread::sleep(Duration::from_millis(300));
    }

    execute!(stdout, Show, LeaveAlternateScreen).unwrap();
}