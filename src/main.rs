use std::env;
mod hex_edit;

use hex_edit::*;

extern crate pancurses;
use pancurses::*;

fn main () {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: HexDump <file>");
        return;
    }
  
    let terminal_window = pancurses::initscr();

    curs_set(0);
    start_color();
    cbreak();
    noecho();
    resize_term(40, 120);

    let mut hex_editor = hex_editor_view::HexEditorView::new(terminal_window, &args[1]);

    loop {
        hex_editor.process_input();
        hex_editor.redraw();

        napms(10);
    }
}

