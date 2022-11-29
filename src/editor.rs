//import Terminal
use crate::Terminal;
//import termion, a library for low-level handling, manipulating and reading information about terminals
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {            
    pub x: usize,            
    pub y: usize,            
}

//srtuct for the editor
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    //fn to refresh and setup the screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {            
            Terminal::clear_screen();
            println!("Goodbye.\r");            
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
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

    //fn to draw welcome message
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    //fn to draw ~ for every row
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

}


fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}