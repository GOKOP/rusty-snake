#[derive(PartialEq)]
pub enum State {
    GAME,
    LOST,
}

pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
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
            direction: Direction::RIGHT,
            growth: 3,
        }
    }

    /// Changes direction of the snake's head to new_dir
    pub fn turn(&mut self, new_dir: Direction) {
        self.direction = new_dir;
    }

    fn move_head(&mut self) {
        match self.direction {
            Direction::RIGHT => self.body[0].0 += 1,
            Direction::LEFT => self.body[0].0 -= 1,
            Direction::UP => self.body[0].1 -= 1,
            Direction::DOWN => self.body[0].1 += 1,
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
