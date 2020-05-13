use pancurses::*;

static DEF_WIDTH: i32 = 40;
static DEF_HEIGHT: i32 = 20;

pub fn init_curses() -> Window {
    let screen = initscr();
    screen.keypad(true);
    screen
}

pub fn init_window(screen: &Window) -> Result<Window, i32> {
    let screen_size = screen.get_max_yx();
    screen.subwin(DEF_HEIGHT, DEF_WIDTH, (screen_size.0/2)-(DEF_HEIGHT/2), (screen_size.1/2)-(DEF_WIDTH/2))
}

pub fn print_game(window: &Window) {
    window.border('#', '#', '#', '#', '#', '#', '#', '#');
    window.refresh();
}
