use pancurses::*;

fn main() {
    let window = initscr();
    window.printw("siema kurwy");
    window.refresh();
    window.getch();
    endwin();
}
