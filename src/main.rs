use pancurses::endwin;
mod display;

fn main() {
    let screen = display::init_curses();
    let window = display::init_window(&screen).expect("Can't create subwindow");

    display::print_game(&window);
    window.getch();
    endwin();
}
