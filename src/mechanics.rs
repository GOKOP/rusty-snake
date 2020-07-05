#[derive(PartialEq, Clone, Copy)]
pub enum State {
    MainMenu,
    Game,
    Lost,
    Quit,
}

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
    pub fn new(position: (i32, i32)) -> Snake {
        Snake {
            body: vec![position],
            direction: Direction::Up,
            growth: 3,
        }
    }

    /// Changes direction of the snake's head to new_dir
    pub fn turn(&mut self, new_dir: Direction) {
        self.direction = new_dir;
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
}

pub fn check_if_lost(max_pos: (i32,i32), snake: &Snake) -> bool {
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
