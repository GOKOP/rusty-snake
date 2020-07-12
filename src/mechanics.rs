use pancurses::{Input, Window};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Lost,
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
    pub growth: u32,
}

impl Snake {
    /// Creates new snake in the given position, pointing to the right
    pub fn new(position: (i32, i32), size: u32) -> Snake {
        Snake {
            body: vec![position],
            direction: Direction::Up,
            growth: size,
        }
    }

    /// Changes direction of the snake's head to new_dir
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

    /// Moves all other pieces in their directions and adds a new piece if self.growth is non-zero
    pub fn advance(&mut self) {
        if self.growth > 0 {
            if let Some(tail) = self.body.last().cloned() {
                self.body.push(tail);
            }
            // leaving growth as it was for a later conditional
        }

        let body_len = self.body.len();

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

    /// checks if given position is inside of the snake
    pub fn inside(&self, pos: (i32, i32)) -> bool {
        for piece in &self.body {
            if pos == *piece {
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

    /// place a new fruit in a random spot between (0,0) and max_pos-1 exclusively
    pub fn place_new(&mut self, max_pos: (i32, i32), snake: &Snake) {
        let mut rng = rand::thread_rng();

        let mut x = 0;
        let mut y = 0;

        while x == 0 || y == 0 || !self.fruit_unique((x, y)) || snake.inside((x, y)) {
            x = rng.gen_range(1, max_pos.0 - 1);
            y = rng.gen_range(1, max_pos.1 - 1);
        }

        self.fruits.push((x, y));
    }

    /// check if new_fruit with given position doesn't already exist
    fn fruit_unique(&self, new_fruit: (i32, i32)) -> bool {
        for fruit in &self.fruits {
            if new_fruit == *fruit {
                return false;
            }
        }

        true
    }

    /// check if a fruit has been eaten and remove it
    pub fn fruit_eaten(&mut self, snake: &Snake) -> bool {
        let mut remove_index: i32 = -1;

        for (index, fruit) in self.fruits.iter().enumerate() {
            if snake.body[0] == *fruit {
                remove_index = index as i32;
                break;
            }
        }

        if remove_index >= 0 {
            self.fruits.remove(remove_index as usize);
            true
        } else {
            false
        }
    }
}

pub fn check_if_lost(max_pos: (i32, i32), snake: &Snake) -> bool {
    for (index, piece) in snake.body.iter().enumerate() {
        if (index == 0
            && (piece.0 <= 0
                || piece.1 <= 0
                || piece.0 >= max_pos.1 - 1
                || piece.1 >= max_pos.0 - 1))
            || (index != 0 && *piece == snake.body[0])
        {
            return true;
        }
    }

    false
}

pub fn handle_input(window: &Window, snake: &mut Snake, state: &mut State) {
    match window.getch() {
        Some(Input::Character('q')) => *state = State::MainMenu,
        Some(Input::KeyUp) => snake.turn(Direction::Up),
        Some(Input::KeyDown) => snake.turn(Direction::Down),
        Some(Input::KeyRight) => snake.turn(Direction::Right),
        Some(Input::KeyLeft) => snake.turn(Direction::Left),
        _ => (),
    }
}
