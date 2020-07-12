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
    pub window: Window, // the small window that the game will actually use
    pub win_size: (i32, i32),
    pub screen_max_yx: (i32, i32), // stored so changes can be traced
}

impl Display {
    pub fn new(win_size: (i32, i32)) -> Display {
        let screen = init_curses();
        let colors = init_colors();
        let window = init_window(&screen, win_size);
        let max_yx = screen.get_max_yx();
        let colorful = colors.len() > 1;

        Display {
            screen: screen,
            colors: colors,
            window: window,
            win_size: win_size,
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

    // not using pancurses::Window:border() because it can't deal with unicode
    fn print_border(&self, ch: char, color_index: i16) {
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

    pub fn print_game(&self, snake: &mechanics::Snake, fruits: &Vec<(i32, i32)>, lost: bool) {
        // init colors indexes (self.colors[0] will be dummy ColorWrap if not using colors)
        let mut frame_color = 0;
        let mut fruit_color = 0;
        let mut snake_color = 0;
        let mut dead_color = 0;
        let mut score_color = 0;

        // set proper values if using colors
        if self.colorful {
            frame_color = COLOR_FRAME;
            fruit_color = COLOR_FRUIT;
            snake_color = COLOR_SNAKE;
            dead_color = COLOR_DEAD;
            score_color = COLOR_SCORE;
        }

        self.window.erase();
        self.print_border('█', frame_color);

        for fruit in fruits {
            self.print((fruit.0, fruit.1), '*', fruit_color);
        }

        for (index, piece) in snake.body.iter().enumerate().rev() {
            if index == 0 && lost {
                self.print((piece.0, piece.1), 'X', dead_color);
            } else if index == 0 {
                self.print((piece.0, piece.1), '@', snake_color);
            } else {
                self.print((piece.0, piece.1), 'o', snake_color);
            }
        }

        // displaying body length in the corner
        let score = format!("Body: {}", snake.body.len());
        self.print((1, self.window.get_max_y() - 1), score, score_color);

        self.window.refresh();
    }

    pub fn print_simple_menu(&self, menu: &interface::SimpleMenu) {
        self.window.erase();

        let menu_height = (menu.options.len() + 2) as i32;
        let window_height = self.win_size.1;
        let menu_start_y = window_height / 2 - menu_height / 2;

        let window_width = self.win_size.0;
        let title_width = menu.title.len() as i32;
        let title_start_x = window_width / 2 - title_width / 2;

        if self.colorful {
            self.print((title_start_x, menu_start_y), &menu.title, COLOR_MENU_TITLE);
        } else {
            self.print((title_start_x, menu_start_y), &menu.title, 0);
        }

        let mut y = menu_start_y + 2; // 2 = title + empty line
        for (index, option) in menu.options.iter().enumerate() {
            // if not colorful then add an indicator ">"
            // and center accordingly

            if index == menu.selected && !self.colorful {
                let string = format!("> {}", &option.text);
                let x = window_width / 2 - ((string.len() / 2) as i32);
                self.window.mvaddstr(y, x, string);

            // otherwise color will be used for that
            } else {
                let x = window_width / 2 - ((&option.text.len() / 2) as i32);

                if index == menu.selected {
                    self.print((x, y), &option.text, COLOR_MENU_SELECTED);
                } else if self.colorful {
                    self.print((x, y), &option.text, COLOR_MENU_OPTION);
                } else {
                    self.window.mvaddstr(y, x, &option.text);
                }
            }
            y += 1
        }

        // place menu.bottom_text in the bottom right corner
        let bottom_text_x = self.window.get_max_x() - 1 - (menu.bottom_text.len() as i32);
        self.window.mvaddstr(
            self.window.get_max_y() - 1,
            bottom_text_x,
            &menu.bottom_text,
        );

        self.window.refresh();
    }

    pub fn recenter(&mut self) {
        if self.screen.get_max_yx() == self.screen_max_yx {
            return;
        }

        self.screen.clear();
        self.screen.refresh();

        self.window = init_window(&self.screen, self.win_size);
        self.screen_max_yx = self.screen.get_max_yx();
    }
}

fn init_colors() -> Vec<ColorWrap> {
    // return Vec with single dummy ColorWrap if not using colors
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
    screen
}

fn init_window(screen: &Window, size: (i32, i32)) -> Window {
    let screen_size = screen.get_max_yx();
    let window = screen
        .subwin(
            size.1,
            size.0,
            (screen_size.0 / 2) - (size.1 / 2),
            (screen_size.1 / 2) - (size.0 / 2),
        )
        .expect("Can't create subwindow");

    window.nodelay(true);
    window.keypad(true);

    window
}
