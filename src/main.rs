use pancurses::{endwin, Input, Window};
use std::{thread, time};

static VERSION: &str = "v0.0.1";

mod display;
mod mechanics;
mod interface;

fn main() {
    let screen = display::init_curses();
    let window = display::init_window(&screen);

    let mut snake = mechanics::Snake::new((20, 10));
    let mut main_menu = create_main_menu();

    let mut going = true;
    let mut state = mechanics::State::MainMenu;

    while going {
        if state == mechanics::State::MainMenu {
            display::print_simple_menu(&window, &main_menu);
            let exec_option = main_menu.handle_input(window.getch());
            if exec_option {
                state = main_menu.options[main_menu.selected].target_state;
            }
            // don't waste cpu on refreshing this
            thread::sleep(time::Duration::from_millis(10));
        }
        else if state == mechanics::State::Game {
            handle_input(&window, &mut snake, &mut going);
            snake.advance();
            display::print_game(&window, &snake, false);
            thread::sleep(time::Duration::from_millis(50));

            if mechanics::check_if_lost(window.get_max_yx(), &snake) {
                state = mechanics::State::Lost;
            }
        } else if state == mechanics::State::Lost {
            display::print_game(&window, &snake, true);
            thread::sleep(time::Duration::from_millis(1000));
            snake = mechanics::Snake::new((20, 10));
            state = mechanics::State::MainMenu;
        } else if state == mechanics::State::Quit {
            going = false;
        }
    }
    endwin();
}

fn handle_input(window: &Window, snake: &mut mechanics::Snake, going: &mut bool) {
    match window.getch() {
        Some(Input::Character('q')) => *going = false,
        Some(Input::KeyUp) => snake.turn(mechanics::Direction::Up),
        Some(Input::KeyDown) => snake.turn(mechanics::Direction::Down),
        Some(Input::KeyRight) => snake.turn(mechanics::Direction::Right),
        Some(Input::KeyLeft) => snake.turn(mechanics::Direction::Left),
        _ => (),
    }
}

fn create_main_menu() -> interface::SimpleMenu {
    let mut options = Vec::new();
    
    options.push(interface::MenuOption::new("Play".to_string(), mechanics::State::Game));
    options.push(interface::MenuOption::new("Exit".to_string(), mechanics::State::Quit));

    interface::SimpleMenu::new("Rusty Snake".to_string(), VERSION.to_string(), options)
}
