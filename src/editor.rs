//import Terminal
use crate::Terminal;
//import termion, a library for low-level handling, manipulating and reading information about terminals
use termion::event::Key;



//srtuct for the editor
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

//implementation of the editor
impl Editor {
    pub fn run(&mut self) {
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

    //initialize the editor in default
    pub fn default() -> Self {
        Self { 
            should_quit: false,

            /*We unwrap the Terminal with expect, which does the 
            following: If we have a value, we return it. If we
             don’t have a value, we panic with the text passed to expect */
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    //fn to refresh and setup the screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {            
            println!("Goodbye.\r");            
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    //fn to process the key press
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        //store the red key
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('a') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    //fn to draw ~ in all lines
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height -1 {
            println!("~\r");
        }
    }

}


fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}