use devtimer::DevTime;
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

    let mut benchmark_timer = DevTime::new_simple();
    let mut sleep_tune_timer = DevTime::new_simple();

    let mut state = mechanics::State::MainMenu;

    loop {
        display.recenter(); // in case the terminal was resized

        if state == mechanics::State::MainMenu {
            simple_menu_logic(&mut main_menu, &display, &mut state);
        //
        } else if state == mechanics::State::Game {
            mechanics::handle_input(&display.window, &mut snake, &mut state);
            snake.advance();

            if snake.check_if_lost(display.window.get_max_yx()) {
                state = mechanics::State::Lost;
            }

            if fruit_manager.fruit_eaten(&snake)
                || fruit_manager.fruits.len() < loaded_settings.min_fruits
            {
                snake.growth += 1;
                fruit_manager.place_new(display.window.get_max_yx(), &snake);
            }

            if loaded_settings.benchmark {
                benchmark_timer.stop();
                display.print_game(
                    &snake,
                    &fruit_manager.fruits,
                    false,
                    loaded_settings.snake_wait,
                    benchmark_timer.time_in_millis().unwrap_or(0),
                );
                benchmark_timer.start();
            } else {
                display.print_game(&snake, &fruit_manager.fruits, false, 0, 0);
            }

            sleep_tune_timer.stop();

            let wait_nanos = loaded_settings.snake_wait * 1000000;
            let elapsed_nanos: u64 = sleep_tune_timer.time_in_nanos().unwrap_or(0) as u64;
            let sleep_nanos: u64;

            if elapsed_nanos < wait_nanos {
                sleep_nanos = wait_nanos - elapsed_nanos;
            } else {
                sleep_nanos = 0;
            }

            thread::sleep(time::Duration::from_nanos(sleep_nanos));
            sleep_tune_timer.start();
        //
        } else if state == mechanics::State::Lost {
            display.print_game(&snake, &fruit_manager.fruits, true, 0, 0);
            thread::sleep(time::Duration::from_millis(1000));
            state = mechanics::State::Reset;
        //
        } else if state == mechanics::State::Reset {
            benchmark_timer.stop();
            sleep_tune_timer.stop();
            snake = mechanics::Snake::new(
                (display.win_size.0 / 2, display.win_size.1 / 2),
                loaded_settings.snake_len,
            );
            fruit_manager = mechanics::FruitManager::new();
            fruit_manager.place_new(display.win_size, &snake);
            flush_input(&display.window);
            state = mechanics::State::MainMenu;
        //
        } else if state == mechanics::State::Quit {
            break;
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
