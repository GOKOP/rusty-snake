use pancurses::*;

pub fn init_curses() -> Window {
    let screen = initscr();
    screen.keypad(true);
    screen
}

pub fn init_window(screen: &Window) -> Result<Window, i32> {
    //temporary
    let width = 40;
    let height = 20;

    let screen_size = screen.get_max_yx();
    screen.subwin(height, width, (screen_size.0/2)-(height/2), (screen_size.1/2)-(width/2))
}

pub fn print_game(window: &Window) {
    window.border('#', '#', '#', '#', '#', '#', '#', '#');
    window.refresh();
}
