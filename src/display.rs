use crate::mechanics;
use crate::interface;
use pancurses::*;

static DEF_WIDTH: i32 = 40;
static DEF_HEIGHT: i32 = 20;

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

pub fn print_game(window: &Window, snake: &mechanics::Snake, lost: bool) {
    window.erase();
    window.border('#', '#', '#', '#', '#', '#', '#', '#');

    for (index, piece) in snake.body.iter().enumerate() {
        if index == 0 && lost {
            window.mvaddch(piece.1, piece.0, 'X');
        } else if index == 0 {
            window.mvaddch(piece.1, piece.0, '@');
        } else {
            window.mvaddch(piece.1, piece.0, 'o');
        }
    }

    window.refresh();
}

pub fn print_simple_menu(window: &Window, menu: &interface::SimpleMenu) {
    window.erase();
    
    let menu_height = (menu.options.len() + 2) as i32;
    let window_height = window.get_max_y();
    let menu_start_y = window_height/2 - menu_height/2;

    let window_width = window.get_max_x();
    let title_width = menu.title.len() as i32;
    let title_start_x = window_width/2 - title_width/2;

    window.mvaddstr(menu_start_y, title_start_x, &menu.title);

    let mut y = menu_start_y + 2;
    for (index, option) in menu.options.iter().enumerate() {
        let string: String;

        if index == menu.selected {
            string = format!("> {}", &option.text);
        } else {
            string = option.text.clone();
        }
        let x = window_width/2 - ((string.len()/2) as i32);
        window.mvaddstr(y, x, string);
        y += 1
    }

    window.refresh();
}
