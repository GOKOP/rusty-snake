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
static COLOR_MENU_SELECTED: i16 = 8;

#[derive(Clone, Copy)]
pub struct ColorWrap {
    pair: i16,
    bold: bool,
    dummy: bool,
}

impl ColorWrap {
    fn new(pair: i16, fcolor: i16, bcolor: i16, bold: bool) -> ColorWrap {
        init_pair(pair, fcolor, bcolor);
        ColorWrap {
            pair: pair,
            bold: bold,
            dummy: false,
        }
    }

    fn new_dummy() -> ColorWrap {
        ColorWrap {
            pair: 0,
            bold: false,
            dummy: true,
        }
    }

    fn enable(&self, window: &Window) {
        if self.dummy {
            return;
        }

        window.attron(COLOR_PAIR(self.pair as u32));
        if self.bold {
            window.attron(A_BOLD);
        }
    }

    fn disable(&self, window: &Window) {
        if self.dummy {
            return
        }

        window.attroff(COLOR_PAIR(self.pair as u32));
        if self.bold {
            window.attroff(A_BOLD);
        }
    }
}

// on failure return a vector with a dummy ColorWrap
pub fn init_colors() -> Vec<ColorWrap> {
    if !has_colors() || start_color() == ERR {
        return vec![ColorWrap::new_dummy()];
    }

    let mut background = COLOR_BLACK;
    if use_default_colors() == OK {
        background = -1;
    }

    // occupy index 0 so that indexes correspond to color pairs
    // (I think that pair 0 is supposed to be the default color or sth)
    let mut colors = vec![ColorWrap::new_dummy()];

    colors.push(ColorWrap::new(COLOR_SNAKE, COLOR_YELLOW, background, true));
    colors.push(ColorWrap::new(COLOR_DEAD, COLOR_RED, background, true));
    colors.push(ColorWrap::new(COLOR_FRAME, COLOR_WHITE, COLOR_WHITE, false));
    colors.push(ColorWrap::new(COLOR_SCORE, COLOR_BLACK, COLOR_WHITE, false));
    colors.push(ColorWrap::new(COLOR_FRUIT, COLOR_RED, background, false));
    colors.push(ColorWrap::new(COLOR_MENU_TITLE, COLOR_RED, background, true));
    colors.push(ColorWrap::new(COLOR_MENU_OPTION, COLOR_WHITE, background, true));
    colors.push(ColorWrap::new(COLOR_MENU_SELECTED, COLOR_WHITE, COLOR_YELLOW, true));

    colors
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

// print a char or a string in given color
fn print<T>(window: &Window, pos: (i32, i32), item: T, color: ColorWrap) where T: ToString {
    color.enable(&window);
    window.mvaddstr(pos.1, pos.0, item.to_string());
    color.disable(&window);
}

pub fn print_game(window: &Window, snake: &mechanics::Snake, fruits: &Vec<(i32, i32)>, lost: bool, colors: &Vec<ColorWrap>) {
    let mut frame_color = colors[0];
    let mut fruit_color = colors[0];
    let mut snake_color = colors[0];
    let mut dead_color = colors[0];
    let mut score_color = colors[0];

    if colors.len() > 1 {
        frame_color = colors[COLOR_FRAME as usize];
        fruit_color = colors[COLOR_FRUIT as usize];
        snake_color = colors[COLOR_SNAKE as usize];
        dead_color = colors[COLOR_DEAD as usize];
        score_color = colors[COLOR_SCORE as usize];
    }

    window.erase();
    print_border(&window, '█', frame_color);

    for fruit in fruits {
        print(&window, (fruit.0, fruit.1), '*', fruit_color);
    }

    for (index, piece) in snake.body.iter().enumerate() {
        if index == 0 && lost {
            print(&window, (piece.0, piece.1), 'X', dead_color);
        } else if index == 0 {
            print(&window, (piece.0, piece.1), '@', snake_color);
        } else {
            print(&window, (piece.0, piece.1), 'o', snake_color);
        }
    }

    // displaying body length in the corner
    let score = format!("Body: {}", snake.body.len());
    print(&window, (1, window.get_max_y() - 1), score, score_color);

    window.refresh();
}

/// print window border
fn print_border(window: &Window, ch: char, color: ColorWrap) {
    let mut horizontal = String::new();

    for _ in 0..window.get_max_x() {
        horizontal = format!("{}{}", horizontal, ch);
    }

    print(&window, (0, 0), &horizontal, color);
    print(&window, (0, window.get_max_y() - 1), &horizontal, color);

    for y in 1..window.get_max_y() - 1 {
        print(&window, (0, y), ch.to_string(), color);
        print(&window, (window.get_max_x() - 1, y), ch.to_string(), color);
    }
}

pub fn print_simple_menu(window: &Window, menu: &interface::SimpleMenu, colors: &Vec<ColorWrap>) {
    window.erase();

    let menu_height = (menu.options.len() + 2) as i32;
    let window_height = window.get_max_y();
    let menu_start_y = window_height / 2 - menu_height / 2;

    let window_width = window.get_max_x();
    let title_width = menu.title.len() as i32;
    let title_start_x = window_width / 2 - title_width / 2;

    if colors.len() == 1 {
        print(&window, (title_start_x, menu_start_y), &menu.title, colors[0]);
    } else {
        print(&window, (title_start_x, menu_start_y), &menu.title, colors[COLOR_MENU_TITLE as usize]);
    }

    let mut y = menu_start_y + 2;
    for (index, option) in menu.options.iter().enumerate() {

        if index == menu.selected && colors.len() == 1 {
            let string = format!("> {}", &option.text);
            let x = window_width / 2 - ((string.len() / 2) as i32);
            window.mvaddstr(y, x, string);

        } else {
            let x = window_width / 2 - ((&option.text.len() / 2) as i32);

            if index == menu.selected {
                print(&window, (x, y), &option.text, colors[COLOR_MENU_SELECTED as usize]);
            } else if colors.len() > 1 {
                print(&window, (x, y), &option.text, colors[COLOR_MENU_OPTION as usize]);
            } else {
                window.mvaddstr(y, x, &option.text);
            }
        }

        y += 1
    }

    let bottom_text_x = window.get_max_x() - 1 - (menu.bottom_text.len() as i32);
    window.mvaddstr(window.get_max_y() - 1, bottom_text_x, &menu.bottom_text);

    window.refresh();
}
