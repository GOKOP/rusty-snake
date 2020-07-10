use pancurses::{endwin, Input, Window};
use std::{thread, time};

const VERSION: &str = "v0.2.0";

mod display;
mod interface;
mod mechanics;
mod settings;

fn main() {
    let settings = settings::create();
    let loaded_settings = settings::read_cli_args(&settings);

    let screen = display::init_curses();
    let colors = display::init_colors();
    let mut window = display::init_window(&screen, loaded_settings.win_size);
    let mut max_yx = screen.get_max_yx();

    let mut main_menu = create_main_menu();
    let mut snake = mechanics::Snake::new((loaded_settings.win_size.0/2, loaded_settings.win_size.1/2));
    let mut fruit_manager = mechanics::FruitManager::new();
    new_fruit_wrapper(window.get_max_yx(), &snake, &mut fruit_manager);

    let mut going = true;
    let mut state = mechanics::State::MainMenu;

    while going {
        display::recenter(&screen, &mut window, &mut max_yx, loaded_settings.win_size);

        if state == mechanics::State::MainMenu {
            display::print_simple_menu(&window, &main_menu, &colors);
            let exec_option = main_menu.handle_input(window.getch());
            if exec_option {
                state = main_menu.options[main_menu.selected].target_state;
            }
            // don't waste cpu on refreshing this
            thread::sleep(time::Duration::from_millis(10));
        } else if state == mechanics::State::Game {
            handle_input(&window, &mut snake, &mut state);
            snake.advance();
            display::print_game(&window, &snake, &fruit_manager.fruits, false, &colors);
            thread::sleep(time::Duration::from_millis(50));

            if mechanics::check_if_lost(window.get_max_yx(), &snake) {
                state = mechanics::State::Lost;
            }

            if fruit_manager.fruit_eaten(&snake) {
                snake.growth += 1;
                new_fruit_wrapper(window.get_max_yx(), &snake, &mut fruit_manager);
            }
        } else if state == mechanics::State::Lost {
            display::print_game(&window, &snake, &fruit_manager.fruits, true, &colors);
            thread::sleep(time::Duration::from_millis(1000));
            snake = mechanics::Snake::new((loaded_settings.win_size.0/2, loaded_settings.win_size.1/2));
            fruit_manager = mechanics::FruitManager::new();
            new_fruit_wrapper(window.get_max_yx(), &snake, &mut fruit_manager);
            flush_input(&window);
            state = mechanics::State::MainMenu;
        } else if state == mechanics::State::Quit {
            going = false;
        }
    }
    endwin();
}

fn handle_input(window: &Window, snake: &mut mechanics::Snake, state: &mut mechanics::State) {
    match window.getch() {
        Some(Input::Character('q')) => *state = mechanics::State::MainMenu,
        Some(Input::KeyUp) => snake.turn(mechanics::Direction::Up),
        Some(Input::KeyDown) => snake.turn(mechanics::Direction::Down),
        Some(Input::KeyRight) => snake.turn(mechanics::Direction::Right),
        Some(Input::KeyLeft) => snake.turn(mechanics::Direction::Left),
        _ => (),
    }
}

fn create_main_menu() -> interface::SimpleMenu {
    let mut options = Vec::new();

    options.push(interface::MenuOption::new(
        "Play".to_string(),
        mechanics::State::Game,
    ));
    options.push(interface::MenuOption::new(
        "Exit".to_string(),
        mechanics::State::Quit,
    ));

    interface::SimpleMenu::new("Rusty Snake".to_string(), VERSION.to_string(), options)
}

fn new_fruit_wrapper(
    max_yx: (i32, i32),
    snake: &mechanics::Snake,
    fruit_manager: &mut mechanics::FruitManager,
) {
    let max_xy = (max_yx.1, max_yx.0);
    fruit_manager.place_new(max_xy, &snake);
}

fn flush_input(window: &Window) {
    while let Some(_) = window.getch() {
        // do nothing
    }
}
