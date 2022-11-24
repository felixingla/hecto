use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;


fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();

    //With for..in in combination with bytes(), we are asking rust to read 
    //byte from the stdin into the variable b, and to keep doing it 
    //until there are no more bytes to read
    for b in io::stdin().bytes(){

        let b = b.unwrap() as char;
        let c = b as char;

        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        println!("{}", c);
        
        //write q to quit the program
        if c == 'q' {
            break;
        }
    }
}
