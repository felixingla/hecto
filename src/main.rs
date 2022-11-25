use std::io;
use std::io::Read;
use std::io::stdout;
use termion::raw::IntoRawMode;


//function to compare char to 0b0000_1111 byte (CTRL + A)
fn to_ctrl_byte(c: char) -> u8{
    let byte = c as u8;
    byte & 0b0000_1111
}

//function that prints an error message and exits the program
fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();

    //With for..in in combination with bytes(), we are asking rust to read 
    //byte from the stdin into the variable b, and to keep doing it 
    //until there are no more bytes to read
    for b in io::stdin().bytes(){

        //unrwap() gets the result of the computation
        match b {

            //If variable b is an Ok value, unbind the b contents into b:
            Ok(b) => {
                let c = b as char;

                //is_control() tests whether a character is a control character. 
                //Control characters are non-printable characters that we don’t want to print to the screen. 
                //ASCII codes 0–31 are all control characters, and 127 is also a control character. 
                //ASCII codes 32–126 are all printable.
                if c.is_control() {
                    println!("Byte input: {:?}, Character output is non-printable \r", b);
                } else {
                    println!("Byte input: {:?}, Character output: {}\r", b, c);
                    
                    //print out the binary representation of the variable b
                    //println!("{:#b}", b);
                }
                if b == to_ctrl_byte('q') {
                    println!("End of program");
                    break;
                }
            }
            
            //If variable b is an Err value, do this:
            Err(err) => die(err),
        }
        

        //print out the binary representation of the variable b

    }
}
