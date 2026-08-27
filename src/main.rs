



fn main() {

    for row in 0..7{

        for column in 0..21 {

            if row == 0 || row == 6 {
                print!("#");
            } else if column == 0 || column == 20 {
                print!("#");
            } else{
                print!(" ");
            }

        }

        println!();
    }    
}