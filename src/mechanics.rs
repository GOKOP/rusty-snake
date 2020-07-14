use pancurses::{Input, Window};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Lost,
    Reset,
    Quit,
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Snake {
    pub body: Vec<(i32, i32)>,
    pub direction: Direction,
    // how much is the snake supposed to grow yet
    pub growth: u32,
}

impl Snake {
    pub fn new(position: (i32, i32), size: u32) -> Snake {
        Snake {
            body: vec![position],
            direction: Direction::Up,
            growth: size,
        }
    }

    pub fn turn(&mut self, new_dir: Direction) {
        if (new_dir == Direction::Up && self.direction != Direction::Down)
            || (new_dir == Direction::Down && self.direction != Direction::Up)
            || (new_dir == Direction::Right && self.direction != Direction::Left)
            || (new_dir == Direction::Left && self.direction != Direction::Right)
        {
            self.direction = new_dir;
        }
    }

    fn move_head(&mut self) {
        match self.direction {
            Direction::Right => self.body[0].0 += 1,
            Direction::Left => self.body[0].0 -= 1,
            Direction::Up => self.body[0].1 -= 1,
            Direction::Down => self.body[0].1 += 1,
        }
    }

    pub fn advance(&mut self) {
        // add a piece if there's too few
        if self.growth > 0 {
            if let Some(tail) = self.body.last().cloned() {
                self.body.push(tail);
            }
            // leaving growth as it was for a later conditional
        }

        let body_len = self.body.len();

        // move in reverse so that the entire snake isn't just copying whatever the head does
        for index in (0..body_len).rev() {
            // skipping the new piece
            if index == body_len - 1 && self.growth > 0 {
                self.growth -= 1; // ok now it can be decremented
                continue;
            }

            if index == 0 {
                self.move_head();
            } else {
                self.body[index] = self.body[index - 1];
            }
        }
    }

    pub fn inside(&self, pos: (i32, i32)) -> bool {
        for piece in &self.body {
            if pos == *piece {
                return true;
            }
        }
        false
    }

    pub fn check_if_lost(&self, max_pos: (i32, i32)) -> bool {
        for (index, piece) in self.body.iter().enumerate() {
            if (index == 0
                && (piece.0 <= 0
                    || piece.1 <= 0
                    || piece.0 >= max_pos.1 - 1
                    || piece.1 >= max_pos.0 - 1))
                || (index != 0 && *piece == self.body[0])
            {
                return true;
            }
        }
        false
    }
}

pub struct FruitManager {
    pub fruits: Vec<(i32, i32)>,
}

impl FruitManager {
    pub fn new() -> FruitManager {
        FruitManager {
            fruits: Vec::<(i32, i32)>::new(),
        }
    }

    pub fn place_new(&mut self, max_xy: (i32, i32), snake: &Snake) {
        let mut rng = rand::thread_rng();

        let mut x = 0;
        let mut y = 0;

        while x == 0 || y == 0 || !self.fruit_unique((x, y)) || snake.inside((x, y)) {
            x = rng.gen_range(1, max_xy.0 - 2);
            y = rng.gen_range(1, max_xy.1 - 2);
        }

        self.fruits.push((x, y));
    }

    fn fruit_unique(&self, new_fruit: (i32, i32)) -> bool {
        for fruit in &self.fruits {
            if new_fruit == *fruit {
                return false;
            }
        }

        true
    }

    pub fn fruit_eaten(&mut self, snake: &Snake) -> bool {
        let mut remove_index: i32 = -1;

        // find if a fruit was eaten
        for (index, fruit) in self.fruits.iter().enumerate() {
            if snake.body[0] == *fruit {
                remove_index = index as i32;
                break;
            }
        }

        // remove the fruit
        if remove_index >= 0 {
            self.fruits.remove(remove_index as usize);
            true
        } else {
            false
        }
    }
}

pub fn handle_input(window: &Window, mut snake: &mut Snake, mut state: &mut State) {
    match window.getch() {
        Some(Input::Character(c)) => handle_char_input(c, &mut snake, &mut state),
        Some(Input::KeyUp) => snake.turn(Direction::Up),
        Some(Input::KeyDown) => snake.turn(Direction::Down),
        Some(Input::KeyRight) => snake.turn(Direction::Right),
        Some(Input::KeyLeft) => snake.turn(Direction::Left),
        _ => (),
    }
}

// exists to lowercase input properly
fn handle_char_input(input: char, snake: &mut Snake, state: &mut State) {
    let input_lower = input.to_lowercase().to_string();

    match input_lower.as_str() {
        "q" => *state = State::Reset,
        "w" => snake.turn(Direction::Up),
        "k" => snake.turn(Direction::Up),
        "s" => snake.turn(Direction::Down),
        "j" => snake.turn(Direction::Down),
        "d" => snake.turn(Direction::Right),
        "l" => snake.turn(Direction::Right),
        "a" => snake.turn(Direction::Left),
        "h" => snake.turn(Direction::Left),
        _ => (),
    }
}
