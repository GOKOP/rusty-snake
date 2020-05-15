#[derive(Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Clone, Copy)]
pub struct BodyPiece {
    pub position: (i32, i32),
    pub direction: Direction,
}

impl BodyPiece {
    fn new(position: (i32, i32), direction: Direction) -> BodyPiece {
        BodyPiece {
            position: position,
            direction: direction,
        }
    }

    /// Moves the piece in its current direction
    fn r#move(&mut self) {
        match self.direction {
            Direction::UP => self.position.1 -= 1,
            Direction::DOWN => self.position.1 += 1,
            Direction::RIGHT => self.position.0 += 1,
            Direction::LEFT => self.position.0 -= 1,
        }
    }
}

pub struct Snake {
    pub body: Vec<BodyPiece>,
    growth: u32,
}

impl Snake {
    /// Creates new snake in the given position, pointing to the right
    pub fn new(position: (i32, i32)) -> Snake {
        Snake {
            body: vec![BodyPiece::new(position, Direction::RIGHT)],
            growth: 3,
        }
    }

    /// Changes direction of the snake's head to new_dir
    pub fn turn(&mut self, new_dir: &Direction) {
        self.body[0].direction = *new_dir;
    }

    /// Moves all other pieces in their directions and adds a new piece if self.growth is non-zero
    pub fn advance(&mut self) {
        if self.growth > 0 {
            if let Some(tail) = self.body.last().cloned() {
                self.body.push(tail);
            }
            // leaving growth as it was for later conditional
        }

        let body_len = self.body.len();

        for index in (0..body_len).rev() {
            // skipping the new piece
            if index == body_len - 1 && self.growth > 0 {
                self.growth -= 1; // ok now it can be decremented
                continue;
            }

            self.body[index].r#move();
            if index > 0 {
                self.body[index].direction = self.body[index - 1].direction;
            }
        }
    }
}
