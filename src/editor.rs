use std::io::{self, stdout, Write};
//termion is a library for low-level handling, manipulating and reading information about terminals
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//srtuct for the Editor
pub struct Editor {
    should_quit: bool,
}

//implementation of the Edtior
impl Editor {
    pub fn run(&mut self) {

        //set the terminal into raw mode (raw mode doesn't print out automatically)
        let _stdout = stdout().into_raw_mode().unwrap();

        //loop to handle all inputs
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    //fn to clear and setup the screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        
        if self.should_quit {            
            println!("Goodbye.\r");            
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1,1));
        }

        io::stdout().flush()
    }

    //fn to process the key press
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        //store the key redca
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('a') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    //fn to draw ~ in all lines
    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }

}

//fn to read keys
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}