use pancurses::*;

fn main() {
    let screen = init_curses();
    let window = init_window(&screen).expect("Can't create subwindow");

    window.border('#', '#', '#', '#', '#', '#', '#', '#');
    window.refresh();
    window.getch();
    endwin();
}

fn init_curses() -> Window {
    let screen = initscr();
    screen.keypad(true);
    screen
}

fn init_window(screen: &Window) -> Result<Window, i32> {
    //temporary
    let width = 40;
    let height = 20;

    let screen_size = screen.get_max_yx();
    screen.subwin(height, width, (screen_size.0/2)-(height/2), (screen_size.1/2)-(width/2))
}
