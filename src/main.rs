use pancurses::{endwin, Input, Window};
use std::{thread, time};

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

    let mut main_menu = interface::create_main_menu();
    let mut snake = mechanics::Snake::new(
        (
            loaded_settings.win_size.0 / 2,
            loaded_settings.win_size.1 / 2,
        ),
        loaded_settings.snake_len,
    );
    let mut fruit_manager = mechanics::FruitManager::new();
    new_fruit_xy(window.get_max_yx(), &snake, &mut fruit_manager);

    let mut going = true;
    let mut state = mechanics::State::MainMenu;

    while going {
        display::recenter(&screen, &mut window, &mut max_yx, loaded_settings.win_size);
            
        if state == mechanics::State::MainMenu {
            simple_menu_logic(&mut main_menu, &window, &colors, &mut state);
        } else if state == mechanics::State::Game {
            handle_input(&window, &mut snake, &mut state);
            snake.advance();
            display::print_game(&window, &snake, &fruit_manager.fruits, false, &colors);
            thread::sleep(time::Duration::from_millis(loaded_settings.snake_wait));

            if mechanics::check_if_lost(window.get_max_yx(), &snake) {
                state = mechanics::State::Lost;
            }

            if fruit_manager.fruit_eaten(&snake)
                || fruit_manager.fruits.len() < loaded_settings.min_fruits
            {
                snake.growth += 1;
                new_fruit_xy(window.get_max_yx(), &snake, &mut fruit_manager);
            }
        } else if state == mechanics::State::Lost {
            display::print_game(&window, &snake, &fruit_manager.fruits, true, &colors);
            thread::sleep(time::Duration::from_millis(1000));
            snake = mechanics::Snake::new(
                (
                    loaded_settings.win_size.0 / 2,
                    loaded_settings.win_size.1 / 2,
                ),
                loaded_settings.snake_len,
            );
            fruit_manager = mechanics::FruitManager::new();
            new_fruit_xy(window.get_max_yx(), &snake, &mut fruit_manager);
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

fn new_fruit_xy(
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

fn simple_menu_logic(menu: &mut interface::SimpleMenu, window: &Window, colors: &Vec<display::ColorWrap>, state: &mut mechanics::State) {
    if menu.handle_input(window.getch()) {
        *state = menu.options[menu.selected].target_state;
    }
    display::print_simple_menu(&window, &menu, &colors);

    // don't waste cpu on refreshing this
    thread::sleep(time::Duration::from_millis(10));
}
