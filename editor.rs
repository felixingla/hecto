//import Document
use crate::Document;
use crate::Row;
//import Terminal
use crate::Terminal;
//import termion, a library for low-level handling, manipulating and reading information about terminals
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

//struct for the curstor position
#[derive(Default)]
pub struct Position {            
    pub x: usize,            
    pub y: usize,            
}

//srtuct for the editor
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

//implementation of the editor
impl Editor {

    //fn to handle all inputs via loop
    pub fn run(&mut self) {
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
            document: Document::open(),
            cursor_position: Position::default(),
        }
    }



    //fn to refresh and setup the screen
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
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
            Key::Up            
            | Key::Down            
            | Key::Left            
            | Key::Right            
            | Key::PageUp            
            | Key::PageDown            
            | Key::End            
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    //fn to move the cursor around the screen
    fn move_cursor(&mut self, key: Key) {
            let Position {mut y, mut x} = self.cursor_position;
            let size = self.terminal.size();
            let height = size.height.saturating_sub(1) as usize;
            let width = size.width.saturating_sub(1) as usize;
            
            match key{
                Key::Up => y = y.saturating_sub(1),
                Key::Down => {
                    if y < height {
                        y = y.saturating_add(1);
                    }
                }
                Key::Left => x = x.saturating_sub(1),
                Key::Right => {
                    if x < width {
                        x = x.saturating_add(1);
                    }
                }
                Key::PageUp => y = 0,
                Key::PageDown => y = height,
                Key::Home => y = 0,
                Key::End => y = width,
                _ => (),
            }
            self.cursor_position = Position { x, y}
        }

    //fn to draw welcome message
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Editor in Rust -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    
    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;            
        let row = row.render(start, end);            
        println!("{}\r", row)
    }

    //fn to draw ~ for every row
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            }
            else if terminal_row == height / 3 {
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