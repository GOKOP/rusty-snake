use clap::crate_version;
use crate::mechanics::State;
use pancurses::Input;

pub struct MenuOption {
    pub text: String,
    pub target_state: State,
}

impl MenuOption {
    pub fn new(text: String, target_state: State) -> MenuOption {
        MenuOption {
            text: text,
            target_state: target_state,
        }
    }
}

pub struct SimpleMenu {
    pub title: String,
    pub bottom_text: String,
    pub options: Vec<MenuOption>,
    pub selected: usize,
}

impl SimpleMenu {
    pub fn new(title: String, bottom_text: String, options: Vec<MenuOption>) -> SimpleMenu {
        SimpleMenu {
            title: title,
            bottom_text: bottom_text,
            options: options,
            selected: 0,
        }
    }

    /// move down the options list and wrap to the top
    fn move_down(&mut self) {
        if self.selected >= self.options.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    /// move up the options list and wrap to the bottom
    fn move_up(&mut self) {
        if self.selected <= 0 {
            self.selected = self.options.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    /// handle user input; return true means that target_state should be applied
    pub fn handle_input(&mut self, input: Option<Input>) -> bool {
        match input {
            Some(Input::Character(value)) => return self.handle_char_input(value),
            Some(Input::KeyUp) => self.move_up(),
            Some(Input::KeyDown) => self.move_down(),
            Some(Input::KeyRight) => return true,
            _ => (),
        }

        false
    }

    /// handle user input when it's a char (made for converting to lowercase)
    fn handle_char_input(&mut self, input: char) -> bool {
        let input_lower = input.to_lowercase().to_string();

        match input_lower.as_str() {
            "k" => self.move_up(),
            "j" => self.move_down(),
            "w" => self.move_up(),
            "s" => self.move_down(),
            "\n" => return true,
            "l" => return true,
            "d" => return true,
            _ => (),
        }

        false
    }
}

fn create_main_menu() -> SimpleMenu {
    let mut options = Vec::new();

    options.push(MenuOption::new(
        "Play".to_string(),
        mechanics::State::Game,
    ));
    options.push(MenuOption::new(
        "Exit".to_string(),
        State::Quit,
    ));

    SimpleMenu::new(
        "Rusty Snake".to_string(),
        crate_version!().to_string(),
        options,
    )
}

