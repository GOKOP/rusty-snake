use pancurses::{endwin, Window};
use std::{thread, time};

mod display;
mod interface;
mod mechanics;
mod settings;

fn main() {
    // init stuff
    let settings = settings::create();
    let loaded_settings = settings::read_cli_args(&settings);

    let mut display = display::Display::new(loaded_settings.win_size, loaded_settings.use_color);

    let mut main_menu = interface::create_main_menu();
    let mut snake = mechanics::Snake::new(
        (
            loaded_settings.win_size.0 / 2,
            loaded_settings.win_size.1 / 2,
        ),
        loaded_settings.snake_len,
    );
    let mut fruit_manager = mechanics::FruitManager::new();

    let mut going = true;
    let mut state = mechanics::State::MainMenu;

    while going {
        display.recenter(); // in case the terminal was resized

        if state == mechanics::State::MainMenu {
            simple_menu_logic(&mut main_menu, &display, &mut state);
        } else if state == mechanics::State::Game {
            mechanics::handle_input(&display.window, &mut snake, &mut state);
            snake.advance();
            display.print_game(&snake, &fruit_manager.fruits, false);
            thread::sleep(time::Duration::from_millis(loaded_settings.snake_wait));

            if snake.check_if_lost(display.window.get_max_yx()) {
                state = mechanics::State::Lost;
            }

            if fruit_manager.fruit_eaten(&snake)
                || fruit_manager.fruits.len() < loaded_settings.min_fruits
            {
                snake.growth += 1;
                fruit_manager.place_new(display.window.get_max_yx(), &snake);
            }
        } else if state == mechanics::State::Lost {
            display.print_game(&snake, &fruit_manager.fruits, true);
            thread::sleep(time::Duration::from_millis(1000));

            // reset the game
            mechanics::reset(&mut snake, &mut fruit_manager, loaded_settings.win_size, loaded_settings.snake_len);
            flush_input(&display.window);

            // and go back to the menu
            state = mechanics::State::MainMenu;
        } else if state == mechanics::State::Quit {
            going = false;
        }
    }
    endwin();
}

fn flush_input(window: &Window) {
    while let Some(_) = window.getch() {
        // do nothing
    }
}

fn simple_menu_logic(
    menu: &mut interface::SimpleMenu,
    display: &display::Display,
    state: &mut mechanics::State,
) {
    if menu.handle_input(display.window.getch()) {
        *state = menu.options[menu.selected].target_state;
    }
    display.print_simple_menu(&menu);

    // don't waste cpu on refreshing this
    thread::sleep(time::Duration::from_millis(10));
}
