pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;

pub type Grid = [[u8; WIDTH]; HEIGHT];

pub struct Game {
    pub state: Grid,
    next: Grid,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: [[0; WIDTH]; HEIGHT],
            next: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn update(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_neighbors(x, y);
                let cell = self.state[y][x];

                self.next[y][x] = match (cell, neighbors) {
                    (1, 2) | (1, 3) => 1,
                    (0, 3) => 1,
                    _ => 0,
                };
            }
        }

        std::mem::swap(&mut self.state, &mut self.next);
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && nx < WIDTH as isize && ny < HEIGHT as isize {
                    count += self.state[ny as usize][nx as usize];
                }
            }
        }
        count
    }
}
