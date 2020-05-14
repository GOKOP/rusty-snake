use pancurses::endwin;

mod display;
mod mechanics;

fn main() {
    let screen = display::init_curses();
    let window = display::init_window(&screen).expect("Can't create subwindow");

    let _snake = mechanics::Snake::new((20,10));

    display::print_game(&window);
    window.getch();
    endwin();
}
