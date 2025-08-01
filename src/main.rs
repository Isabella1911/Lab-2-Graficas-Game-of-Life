mod game;
mod patterns;

use game::{Game, WIDTH, HEIGHT};
use minifb::{Key, Window, WindowOptions};
use patterns::*;
use std::{thread, time::Duration};

const SCALE: usize = 6;

/// Dibuja un píxel ampliado en la ventana según el tamaño de escala.
/// Usa coordenadas lógicas (x, y) del universo de Game of Life.
fn point(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    for dy in 0..SCALE {
        for dx in 0..SCALE {
            let px = x * SCALE + dx;
            let py = y * SCALE + dy;
            if px < WIDTH * SCALE && py < HEIGHT * SCALE {
                buffer[py * WIDTH * SCALE + px] = color;
            }
        }
    }
}

fn main() {
    // Crear el juego y ventana
    let mut game = Game::new();
    let mut buffer = vec![0; WIDTH * SCALE * HEIGHT * SCALE];

    let mut window = Window::new(
        "Conway's Game of Life - Rust + point()",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    // PATRONES INICIALES
    seed_glider(&mut game.state, 2, 2);
    seed_lwss(&mut game.state, 20, 5);
    seed_blinker(&mut game.state, 50, 10);
    seed_r_pentomino(&mut game.state, 70, 20);
    seed_glider_gun(&mut game.state, 5, 40);

    // LOOP de simulación
    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.update();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = if game.state[y][x] == 1 {
                    0xFFFFD700 // Amarillo dorado (célula viva)
                } else {
                    0xFF000000 // Negro (muerta)
                };
                point(&mut buffer, x, y, color);
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)
            .expect("Falló al actualizar la ventana");

        thread::sleep(Duration::from_millis(80));
    }
}
