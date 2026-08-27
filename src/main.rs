use std::{thread, time::Duration};

fn main() {
    let mut snake_col = 10;

    for _ in 0..10 {
        print!("\x1B[2J\x1B[1;1H");

        for row in 0..7 {
            for column in 0..21 {
                if row == 0 || row == 6 {
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

        if snake_col < 19 {
            snake_col += 1;
        }
        
        thread::sleep(Duration::from_millis(300));
        
    }
}