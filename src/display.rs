use crate::interface;
use crate::mechanics;
use pancurses::*;

static DEF_WIDTH: i32 = 40;
static DEF_HEIGHT: i32 = 20;

// which color pair is used for what
static COLOR_SNAKE: i16 = 1;
static COLOR_DEAD: i16 = 2;
static COLOR_FRAME: i16 = 3;
static COLOR_SCORE: i16 = 4;
static COLOR_FRUIT: i16 = 5;
static COLOR_MENU_TITLE: i16 = 6;
static COLOR_MENU_OPTION: i16 = 7;
static COLOR_MENU_ACTIVE: i16 = 8;

// bool value in return tells whether pair at index is supposed to be bold
// on failure return an empty vector because I'd have to create it anyway
pub fn init_colors() -> Vec<bool> {
    if !has_colors() {
        return Vec::<bool>::new();
    }

    let mut background = COLOR_BLACK;
    if use_default_colors() == OK {
        background = -1;
    }

    let mut bold_values = vec![false]; // index 0 refers to color pair 0 which is the default one

    init_pair(COLOR_SNAKE, COLOR_GREEN, background);
    bold_values.push(true);

    init_pair(COLOR_DEAD, COLOR_GREEN, background);
    bold_values.push(true);

    init_pair(COLOR_FRAME, COLOR_WHITE, COLOR_WHITE);
    bold_values.push(false);

    init_pair(COLOR_SCORE, background, COLOR_WHITE);
    bold_values.push(false);

    init_pair(COLOR_FRUIT, COLOR_RED, background);
    bold_values.push(false);

    init_pair(COLOR_MENU_TITLE, COLOR_RED, background);
    bold_values.push(true);

    init_pair(COLOR_MENU_OPTION, COLOR_WHITE, background);
    bold_values.push(true);

    init_pair(COLOR_MENU_ACTIVE, COLOR_WHITE, COLOR_RED);
    bold_values.push(true);

    bold_values
}

pub fn init_curses() -> Window {
    let screen = initscr();
    screen.keypad(true);
    screen.nodelay(true);
    curs_set(0);
    cbreak();
    screen
}

pub fn init_window(screen: &Window) -> Window {
    let screen_size = screen.get_max_yx();
    let window = screen
        .subwin(
            DEF_HEIGHT,
            DEF_WIDTH,
            (screen_size.0 / 2) - (DEF_HEIGHT / 2),
            (screen_size.1 / 2) - (DEF_WIDTH / 2),
        )
        .expect("Can't create subwindow");

    window.nodelay(true);
    window.keypad(true);

    window
}

pub fn print_game(window: &Window, snake: &mechanics::Snake, fruits: &Vec<(i32, i32)>, lost: bool) {
    window.erase();
    //window.border('#', '#', '#', '#', '#', '#', '#', '#');
    print_border(&window, '█');

    for fruit in fruits {
        window.mvaddch(fruit.1, fruit.0, '*');
    }

    for (index, piece) in snake.body.iter().enumerate() {
        if index == 0 && lost {
            window.mvaddch(piece.1, piece.0, 'X');
        } else if index == 0 {
            window.mvaddch(piece.1, piece.0, '@');
        } else {
            window.mvaddch(piece.1, piece.0, 'o');
        }
    }

    // displaying body length in the corner
    let score = format!("Body: {}", snake.body.len());
    window.mvaddstr(window.get_max_y() - 1, 1, score);

    window.refresh();
}

/// print window border with unicode support
fn print_border(window: &Window, ch: char) {
    let mut horizontal = String::new();

    for _ in 0..window.get_max_x() {
        horizontal = format!("{}{}", horizontal, ch);
    }

    window.mvaddstr(0, 0, &horizontal);
    window.mvaddstr(window.get_max_y() - 1, 0, &horizontal);

    for y in 1..window.get_max_y() - 1 {
        window.mvaddstr(y, 0, ch.to_string());
        window.mvaddstr(y, window.get_max_x() - 1, ch.to_string());
    }
}

pub fn print_simple_menu(window: &Window, menu: &interface::SimpleMenu) {
    window.erase();

    let menu_height = (menu.options.len() + 2) as i32;
    let window_height = window.get_max_y();
    let menu_start_y = window_height / 2 - menu_height / 2;

    let window_width = window.get_max_x();
    let title_width = menu.title.len() as i32;
    let title_start_x = window_width / 2 - title_width / 2;

    window.mvaddstr(menu_start_y, title_start_x, &menu.title);

    let mut y = menu_start_y + 2;
    for (index, option) in menu.options.iter().enumerate() {
        let string: String;

        if index == menu.selected {
            string = format!("> {}", &option.text);
        } else {
            string = option.text.clone();
        }
        let x = window_width / 2 - ((string.len() / 2) as i32);
        window.mvaddstr(y, x, string);
        y += 1
    }

    let bottom_text_x = window.get_max_x() - 1 - (menu.bottom_text.len() as i32);
    window.mvaddstr(window.get_max_y() - 1, bottom_text_x, &menu.bottom_text);

    window.refresh();
}
