// donut.rs
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor::Hide,
    execute, 
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
};

const WIDTH: usize = 80;
const HEIGHT: usize = 40;

pub fn create() {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, Hide).unwrap();

    let mut buffer: Vec<Vec<char>> = vec![vec![' '; WIDTH]; HEIGHT];
    let mut output: Vec<Vec<char>> = vec![vec![' '; WIDTH]; HEIGHT];

    let mut theta: f64 = 0.0;

    loop {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let x_rot = (x as f64 - WIDTH as f64 / 2.0) * cos_theta
                    - (y as f64 - HEIGHT as f64 / 2.0) * sin_theta;
                let y_rot = (x as f64 - WIDTH as f64 / 2.0) * sin_theta
                    + (y as f64 - HEIGHT as f64 / 2.0) * cos_theta;
                let z = 1.0 / (8.0 * x_rot * x_rot + 1.0 + y_rot * y_rot.sqrt());
                let inverse_z = 1.0 - z;

                let xp = (WIDTH as f64 / 2.0 + x_rot * inverse_z * 16.0) as usize;
                let yp = (HEIGHT as f64 / 2.0 + y_rot * inverse_z * 8.0) as usize;

                if xp < WIDTH && yp < HEIGHT {
                    buffer[yp][xp] = '#';
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                output[y][x] = buffer[y][x];
            }
        }

        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            SetBackgroundColor(Color::Black)
        )
        .unwrap();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Black),
                    Print(output[y][x])
                )
                .unwrap();
            }
            stdout.write(b"\n").unwrap();
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(16));

        theta += 0.04;
    }

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        ResetColor,
        terminal::Clear(ClearType::All)
    )
    .unwrap();
}
