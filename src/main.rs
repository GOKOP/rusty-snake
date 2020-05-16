use pancurses::{endwin, Input, Window};
use std::{thread, time};

mod display;
mod mechanics;

fn main() {
    let screen = display::init_curses();
    let window = display::init_window(&screen).expect("Can't create subwindow");
    window.nodelay(true);
    window.keypad(true);

    let mut snake = mechanics::Snake::new((20, 10));

    let mut going = true;
    while going {
        handle_input(&window, &mut snake, &mut going);
        snake.advance();
        display::print_game(&window, &snake);
        thread::sleep(time::Duration::from_millis(50));
    }
    endwin();
}

fn handle_input(window: &Window, snake: &mut mechanics::Snake, going: &mut bool) {
    match window.getch() {
        Some(Input::Character('q')) => *going = false,
        Some(Input::KeyUp) => snake.turn(mechanics::Direction::UP),
        Some(Input::KeyDown) => snake.turn(mechanics::Direction::DOWN),
        Some(Input::KeyRight) => snake.turn(mechanics::Direction::RIGHT),
        Some(Input::KeyLeft) => snake.turn(mechanics::Direction::LEFT),
        _ => (),
    }
}
