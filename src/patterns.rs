use crate::game::Grid;

pub fn seed_glider(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (x, y + 1),
        (x + 1, y + 2),
        (x + 2, y),
        (x + 2, y + 1),
        (x + 2, y + 2),
    ];
    for (px, py) in pattern {
        if px < grid[0].len() && py < grid.len() {
            grid[py][px] = 1;
        }
    }
}

pub fn seed_lwss(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (x + 1, y),
        (x + 4, y),
        (x, y + 1),
        (x, y + 2),
        (x + 4, y + 2),
        (x, y + 3),
        (x + 1, y + 3),
        (x + 2, y + 3),
        (x + 3, y + 3),
    ];
    for (px, py) in pattern {
        if px < grid[0].len() && py < grid.len() {
            grid[py][px] = 1;
        }
    }
}

pub fn seed_blinker(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [(x, y), (x + 1, y), (x + 2, y)];
    for (px, py) in pattern {
        grid[py][px] = 1;
    }
}

pub fn seed_r_pentomino(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (x + 1, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y + 2),
        (x + 2, y + 2),
    ];
    for (px, py) in pattern {
        grid[py][px] = 1;
    }
}

pub fn seed_glider_gun(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (x + 24, y),
        (x + 22, y + 1),
        (x + 24, y + 1),
        (x + 12, y + 2),
        (x + 13, y + 2),
        (x + 20, y + 2),
        (x + 21, y + 2),
        (x + 34, y + 2),
        (x + 35, y + 2),
        (x + 11, y + 3),
        (x + 15, y + 3),
        (x + 20, y + 3),
        (x + 21, y + 3),
        (x + 34, y + 3),
        (x + 35, y + 3),
        (x + 0, y + 4),
        (x + 1, y + 4),
        (x + 10, y + 4),
        (x + 16, y + 4),
        (x + 20, y + 4),
        (x + 21, y + 4),
        (x + 0, y + 5),
        (x + 1, y + 5),
        (x + 10, y + 5),
        (x + 14, y + 5),
        (x + 16, y + 5),
        (x + 17, y + 5),
        (x + 22, y + 5),
        (x + 24, y + 5),
        (x + 10, y + 6),
        (x + 16, y + 6),
        (x + 24, y + 6),
        (x + 11, y + 7),
        (x + 15, y + 7),
        (x + 12, y + 8),
        (x + 13, y + 8),
    ];
    for (px, py) in pattern {
        if px < grid[0].len() && py < grid.len() {
            grid[py][px] = 1;
        }
    }
}
