use pancurses::*;
use hex_edit::binary_file::*;
use std::*;

pub struct HexEditorView {
    _terminal_window: Window,
    _file: BinaryFile,
    _row_offset: u64
} 

trait ControlKey {
    fn get_control_char(&self) -> Option<char>;
}

impl ControlKey for char {
    fn get_control_char(&self) -> Option<char> {
        let self_int = *self as u8;

        if self.is_control() { Some((self_int + 64) as char) } else { None }
    }
}

trait NibbleToChar {
    /// Returns the hex value of the n'th nibble. N is ordered from high to low byte order (i.e. little endian).
    /// If nibble_num is greater than the nuber of nibbles in the type, this function will panic.
    /// Examples:
    ///     0xF0.get_nibble_as_char(0) -> 'F'
    ///     0xF0.get_nibble_as_char(1) -> '0'
    fn get_nibble_as_char(&self, nibble_num: u8) -> char;
}

fn get_nibble(x: u8) -> char {
        match x {
            0x0 => '0',
            0x1 => '1',
            0x2 => '2',
            0x3 => '3',
            0x4 => '4',
            0x5 => '5',
            0x6 => '6',
            0x7 => '7',
            0x8 => '8',
            0x9 => '9',
            0xA => 'A',
            0xB => 'B',
            0xC => 'C',
            0xD => 'D',
            0xE => 'E',
            0xF => 'F',
            _ => { panic!("{} is out of a nibble range", x); }
        }
    }

impl NibbleToChar for u8 {
    fn get_nibble_as_char(&self, nibble_num: u8) -> char {
        match nibble_num {
            0 => get_nibble(self & 0x0F),
            1 => get_nibble((self >> 4) as u8 & 0x0F),
            _ => {panic!("bad")}
        }       
    }

    
}

const OFFSET_COLS: u8 = 20; 

impl HexEditorView {  
    pub fn new(terminal_window: Window, file_path: &String) -> HexEditorView {
        let binary_file = match BinaryFile::open(file_path) {
            Ok(file) => file,
            Err(x) => { panic!("Failed to load file {}. Reason: {:?}", file_path, x); } 
        };

        HexEditorView {
            _terminal_window: terminal_window,
            _file: binary_file,
            _row_offset: 0
        }
    }

    pub fn redraw(&self) {
        let window_dim = self._terminal_window.get_max_yx();     

        self._terminal_window.erase();

        self.draw_data_area();

        self._terminal_window.refresh();
    }

    fn max_rows(&self) -> u64 {
        if self._file.length() as u64 % self.bytes_per_line() as u64 == 0 {
            self._file.length() as u64 / self.bytes_per_line() as u64
        } else {
            self._file.length() as u64 / self.bytes_per_line() as u64 + 1
        }
    }

    fn draw_data_area(&self) {
        let window_rows = self._terminal_window.get_max_y();
        let window_cols = self._terminal_window.get_max_x();
        let mut offset: u64 = self._row_offset * self.bytes_per_line() as u64;
        let data = self._file.as_slice();

        let bytes_per_line = self.bytes_per_line() as u64;
        let max_rows = self.max_rows();

        let last_row_in_view: u64 = if max_rows * bytes_per_line >= offset {
            max_rows - offset / bytes_per_line
        } else {
            0
        };

        let last_row = cmp::min(window_rows as u64, last_row_in_view);

        for row in 0..last_row as i32 {
            self._terminal_window.mvprintw(row, 0, &format!("{:#X}", offset));

            let mut cur_col = OFFSET_COLS;

            for byte_in_row in 0..self.bytes_per_line() {
                let index = (offset + byte_in_row as u64) as usize;

                if index >= data.len() {
                    break;
                }

                let byte = data[index];

                self._terminal_window.mvaddch(row, cur_col as i32, byte.get_nibble_as_char(0));
                self._terminal_window.mvaddch(row, (cur_col + 1) as i32, byte.get_nibble_as_char(1));

                cur_col += 3;
            } 

            offset += self.bytes_per_line() as u64;
        }
    }

    fn draw_control_bar(window: &Window) {
        let window_dim = window.get_max_yx();
    }  

    fn bytes_per_line(&self) -> u16 {
        let cols = self._terminal_window.get_max_x() as u16;
        
        let usable_cols = cols - OFFSET_COLS as u16;

        (usable_cols + 1) / 3
    }

    fn line_down (&mut self) {
        if self._row_offset + 1 < self.max_rows() {
            self._row_offset += 1;
        } else {
            self._row_offset = self.max_rows() - 1;
        }
    }

    fn line_up (&mut self) {
        if self._row_offset > 0 {
            self._row_offset -= 1; 
        }
    }

    fn page_up (&mut self) {
        let rows = self._terminal_window.get_max_y() as u64;

        self._row_offset = if self._row_offset > rows { self._row_offset - rows } else { 0 };  
    }

    fn page_down (&mut self) {
        let rows = self._terminal_window.get_max_y() as u64;

        self._row_offset =  if self._row_offset + rows < self.max_rows() { self._row_offset + rows } else { self.max_rows() - 1};
    }

    pub fn process_input(&mut self) {
        self._terminal_window.nodelay(true);
        self._terminal_window.keypad(true);

        match self._terminal_window.getch() {
            Some(Input::Character(raw_char)) => {
                if raw_char.is_control() {
                    self.process_control_key(raw_char);
                } else {
                    println!("kitty");
                }
            },
            Some(Input::KeyDown) => {
                self.line_down();
            },
            Some(Input::KeyUp) => {
                self.line_up();
            },
            Some(Input::KeyNPage) => {
                self.page_down();
            },
            Some(Input::KeyPPage) => {
                self.page_up();
            }
            _ => {}
        }
    }

    
    fn process_control_key(&self, raw_char: char) {
        match raw_char.get_control_char() {
            Some(unwrapped_char) => {
                match unwrapped_char {
                    'C' => {
                       process::exit(0);
                    }
                    _ => {}
                }           
            }
            _ => { panic!("Expected control key, but got None.") }
        }
    }
}