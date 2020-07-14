use crate::interface;
use crate::mechanics;
use pancurses::*;

// which color pair is used for what
const COLOR_SNAKE: i16 = 1;
const COLOR_DEAD: i16 = 2;
const COLOR_FRAME: i16 = 3;
const COLOR_SCORE: i16 = 4;
const COLOR_FRUIT: i16 = 5;
const COLOR_MENU_TITLE: i16 = 6;
const COLOR_MENU_OPTION: i16 = 7;
const COLOR_MENU_SELECTED: i16 = 8;

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

    // dummy ColorWrap is used to call Display::print() when colors are disabled
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

        // type left to be infered because ncurses expects u32 but pdcurses expects u64
        window.attron(COLOR_PAIR(self.pair as _));
        if self.bold {
            window.attron(A_BOLD);
        }
    }

    fn disable(&self, window: &Window) {
        if self.dummy {
            return;
        }

        window.attroff(COLOR_PAIR(self.pair as _));
        if self.bold {
            window.attroff(A_BOLD);
        }
    }
}

pub struct Display {
    screen: Window, // the entire terminal
    colors: Vec<ColorWrap>,
    colorful: bool,     // are we using colors?
    win_good: bool, // is window the right size or is it for a "TERM TOO SMALL" message?
    pub window: Window, // the small window that the game will actually use
    pub win_size: (i32, i32),
    pub screen_max_yx: (i32, i32), // stored so changes can be traced
}

impl Display {
    pub fn new(win_size: (i32, i32), use_color: bool) -> Display {
        let screen = init_curses();
        let colors = init_colors(use_color);
        let window_wrap = init_window(&screen, win_size);
        let max_yx = screen.get_max_yx();
        let colorful = colors.len() > 1;

        Display {
            screen: screen,
            colors: colors,
            window: window_wrap.0,
            win_size: win_size,
            win_good: window_wrap.1,
            screen_max_yx: max_yx,
            colorful: colorful,
        }
    }

    // print a char or a string in the given color
    fn print<T>(&self, pos: (i32, i32), item: T, color_index: i16)
    where
        T: ToString,
    {
        self.colors[color_index as usize].enable(&self.window);
        self.window.mvaddstr(pos.1, pos.0, item.to_string());
        self.colors[color_index as usize].disable(&self.window);
    }

    fn print_border(&self, ch: char) {
        let mut color_index = 0;

        if self.colorful {
            color_index = COLOR_FRAME;
        }

        // print top and bottom as two long strings
        let mut horizontal = String::new();
        for _ in 0..self.window.get_max_x() {
            horizontal = format!("{}{}", horizontal, ch);
        }

        self.print((0, 0), &horizontal, color_index);
        self.print((0, self.window.get_max_y() - 1), &horizontal, color_index);

        // right and left as individual strings
        for y in 1..self.window.get_max_y() - 1 {
            self.print((0, y), ch.to_string(), color_index);
            self.print(
                (self.window.get_max_x() - 1, y),
                ch.to_string(),
                color_index,
            );
        }
    }

    fn print_snake(&self, snake: &mechanics::Snake, lost: bool) {
        // init color index variables
        let mut snake_color = 0;
        let mut dead_color = 0;

        // use actual colors indexes if supposed to
        if self.colorful { 
            snake_color = COLOR_SNAKE;
            dead_color = COLOR_DEAD;
        }

        // print in reverse so that the head isn't covered by other pieces when lost
        for (index, piece) in snake.body.iter().enumerate().rev() {
            if index == 0 && lost {
                self.print((piece.0, piece.1), 'X', dead_color);
            } else if index == 0 {
                self.print((piece.0, piece.1), '@', snake_color);
            } else {
                self.print((piece.0, piece.1), 'o', snake_color);
            }
        }
    }

    fn print_fruits(&self, fruits: &Vec<(i32, i32)>) {
        let mut fruit_color = 0;

        if self.colorful {
            fruit_color = COLOR_FRUIT;
        }

        for fruit in fruits {
            self.print((fruit.0, fruit.1), '*', fruit_color);
        }
    }

    fn print_score(&self, snake_len: usize) {
        let mut score_color = 0;

        if self.colorful {
            score_color = COLOR_SCORE;
        }

        let score = format!("Body: {}", snake_len);
        self.print((1, self.window.get_max_y() - 1), score, score_color);
    }

    pub fn print_game(&self, snake: &mechanics::Snake, fruits: &Vec<(i32, i32)>, lost: bool) {
        if !self.win_good {
            return;
        }

        self.window.erase();

        self.print_border('█');
        self.print_fruits(fruits);
        self.print_snake(&snake, lost);
        self.print_score(snake.body.len());

        self.window.refresh();
    }

    fn print_menu_title(&self, y: i32, title: &str) {
        let title_width = title.len() as i32;
        let x = self.win_size.0 / 2 - title_width / 2; // centered

        if self.colorful {
            self.print((x, y), title, COLOR_MENU_TITLE);
        } else {
            self.print((x, y), title, 0);
        }
    }

    fn print_menu_option(&self, y: i32, text: &str, selected: bool) {
        let x = self.win_size.0 / 2 - (text.len() as i32) / 2; // centered

        // if color can't be used to indicate the selected option, ">" will be
        if selected && !self.colorful {
            let string = format!(">{}", text);
            let x = self.win_size.0 / 2 - (string.len() as i32) / 2;
            self.print((x, y), &string, 0);
        } else if selected {
            self.print((x, y), text, COLOR_MENU_SELECTED);
        } else if self.colorful {
            self.print((x, y), text, COLOR_MENU_OPTION);
        } else {
            self.print((x, y), text, 0);
        }
    }

    // print menu's bottom text in the bottom right corner
    fn print_menu_bottom_text(&self, text: &str) {
        let x = self.win_size.0 - 1 - (text.len() as i32);
        let y = self.win_size.1 - 1;
        self.print((x, y), text, 0);
    }

    pub fn print_simple_menu(&self, menu: &interface::SimpleMenu) {
        if !self.win_good {
            return;
        }

        self.window.erase();

        let menu_height = (menu.options.len() + 2) as i32;
        let menu_start_y = self.win_size.1 / 2 - menu_height / 2; // menu centered

        self.print_menu_title(menu_start_y, &menu.title);

        let mut y = menu_start_y + 2; // 2 = title + empty line
        for (index, option) in menu.options.iter().enumerate() {
            self.print_menu_option(y, &option.text, index == menu.selected);
            y += 1;
        }

        self.print_menu_bottom_text(&menu.bottom_text);

        self.window.refresh();
    }

    pub fn recenter(&mut self) {
        if self.screen.get_max_yx() == self.screen_max_yx {
            return;
        }

        self.screen.clear();
        self.screen.refresh();

        let window_wrap = init_window(&self.screen, self.win_size);
        self.window = window_wrap.0;
        self.win_good = window_wrap.1;
        self.screen_max_yx = self.screen.get_max_yx();
    }
}

fn init_colors(use_color: bool) -> Vec<ColorWrap> {
    // return Vec with single dummy ColorWrap if not using colors

    // this is a seperate if to avoid calling start_color() which by itself alters display
    if !use_color {
        return vec![ColorWrap::new_dummy()];
    }

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

fn init_curses() -> Window {
    let screen = initscr();
    screen.keypad(true);
    screen.nodelay(true);
    curs_set(0);
    cbreak();
	set_title("Rusty Snake");
    screen
}

// bool value indicates whether the window is fine
// or it's just for displaying error "win too small"
// in which case it shouldn't be altered
fn init_window(screen: &Window, size: (i32, i32)) -> (Window, bool) {
    let screen_size = screen.get_max_yx();
    let maybe_window = screen
        .subwin(
            size.1,
            size.0,
            (screen_size.0 / 2) - (size.1 / 2),
            (screen_size.1 / 2) - (size.0 / 2),
        );

    let window: Window;
    let ok: bool;

    match maybe_window {
        Ok(win) => {
            window = win;
            ok = true;
        },
        Err(_) => {
            window = error_window(&screen);
            ok = false;
        },
    }

    window.nodelay(true);
    window.keypad(true);

    (window, ok)
}

fn error_window(screen: &Window) -> Window {
    let window = screen.subwin(
        screen.get_max_y(), 
        screen.get_max_x(), 
        0, 
        0,
    ).expect("Can't create subwindow");
    window.addstr("TERM TOO SMALL");
    window
}
