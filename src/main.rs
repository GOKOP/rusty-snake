use pancurses::{endwin, Input};
use std::{thread, time};

mod display;
mod mechanics;

fn main() {
    let screen = display::init_curses();
    let window = display::init_window(&screen).expect("Can't create subwindow");
    window.nodelay(true);

    let mut snake = mechanics::Snake::new((20, 10));

    let mut going = true;
    while going {
        match window.getch() {
            Some(Input::Character('q')) => going = false,
            _ => (),
        }
        display::print_game(&window, &snake);
        snake.advance();
        thread::sleep(time::Duration::from_millis(30));
    }
    endwin();
}
