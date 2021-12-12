//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::event;
use ggez::graphics::{self, Color, DrawMode};
use ggez::{timer, Context, GameResult};
use glam::Vec2;

const INPUT: &str = include_str!("./input.txt");

struct Tile {
    pos: Vec2,
    level: i32,
    flashing: bool,
}

impl Tile {
    fn new(x: f32, y: f32, h: i32) -> Self {
        Tile {
            pos: Vec2::new(x * 18.0 + y * 18.0, 500.0 - x * 10.0 + y * 10.0),
            level: h,
            flashing: false,
        }
    }

    fn render(&self, mb: &mut graphics::MeshBuilder) -> GameResult<()> {
        let (x, y) = (self.pos.x, self.pos.y);
        let y0 = y - self.level as f32;

        let col1 = match self.flashing {
            true => Color::from_rgb(160, 255, 160),
            false => Color::from_rgb(160, 160, 200),
        };
        let col2 = Color::from_rgb(255, 255, 255);
        let points = [
            [x, y0],
            [x + 16.0, y0 - 8.0],
            [x + 32.0, y0],
            [x + 16.0, y0 + 8.0],
        ];
        mb.polygon(DrawMode::fill(), &points, col1)?;
        mb.polygon(DrawMode::stroke(1.0), &points, col2)?;
        let col3 = Color::from_rgb(80, 80, 80);
        let col4 = Color::from_rgb(40, 40, 40);

        let points = [
            [x, y0],
            [x + 16.0, y0 + 8.0],
            [x + 32.0, y0],
            [x + 32.0, y + 1.0],
            [x + 16.0, y + 1.0 + 8.0],
            [x, y + 1.0],
        ];
        mb.polygon(DrawMode::fill(), &points, col4)?;
        mb.line(&[[x, y + 1.0], [x, y0]], 1.0, col3)?;
        mb.line(
            &[[x + 16.0, y0 + 8.0], [x + 16.0, y + 1.0 + 8.0]],
            1.0,
            col3,
        )?;
        mb.line(&[[x + 32.0, y0], [x + 32.0, y + 1.0]], 1.0, col3)?;
        Ok(())
    }

    fn increase(&mut self, amount: i32) -> bool {
        if !self.flashing {
            if self.level + amount >= 30 {
                self.level = 30;
                self.flashing = true;
                return true;
            } else {
                self.level += amount;
            }
        }
        false
    }
}

struct MainState {
    board: Vec<Vec<Tile>>,
    flashing: Vec<(i32, i32)>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut board = vec![];
        for (i, line) in INPUT.lines().enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                row.push(Tile::new(
                    j as f32,
                    i as f32,
                    3 * c.to_digit(10).unwrap() as i32,
                ));
            }
            board.push(row);
        }
        let s = MainState {
            board,
            flashing: vec![],
        };
        Ok(s)
    }

    fn propogate(&mut self, ti: i32, tj: i32) -> Vec<(i32, i32)> {
        let mut flashed = vec![];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if ti + i >= 0
                    && ti + i < self.board.len() as i32
                    && tj + j >= 0
                    && tj + j < self.board[(ti + i) as usize].len() as i32
                {
                    if self.board[(ti + i) as usize][(tj + j) as usize]
                        .increase(3)
                    {
                        flashed.push((ti + i, tj + j))
                    }
                }
            }
        }
        return flashed;
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 35;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            println!("{}", ggez::timer::fps(ctx));
            let mut fcnt = 0;
            for row in self.board.iter_mut() {
                for tile in row.iter_mut() {
                    if tile.flashing {
                        tile.level /= 2;
                        fcnt += 1;
                    }
                }
            }
            if self.flashing.len() > 0 {
                let mut next_flash = vec![];
                for (i, j) in self.flashing.clone().iter() {
                    next_flash.extend(self.propogate(*i, *j))
                }
                fcnt += next_flash.len();
                self.flashing = next_flash;
            }
            for y in 0..self.board.len() {
                let w = self.board[y].len();
                for xr in 0..w {
                    if fcnt == 0 {
                        self.board[y][w - xr - 1].increase(1);
                        if self.board[y][w - xr - 1].increase(1) {
                            self.flashing.push((y as i32, (w - xr - 1) as i32));
                        }
                    } else if self.flashing.len() == 0
                        && self.board[y][w - xr - 1].flashing
                    {
                        self.board[y][w - xr - 1].flashing = false;
                        self.board[y][w - xr - 1].level = 0;
                    }
                }
            }
        }
        timer::sleep(std::time::Duration::from_millis(1));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now = std::time::Instant::now();
        graphics::clear(ctx, Color::BLACK);
        let mut mb = graphics::MeshBuilder::new();
        for row in self.board.iter() {
            for tile in row.iter().rev() {
                tile.render(&mut mb)?;
            }
        }
        let mesh = mb.build(ctx)?;
        println!(
            "time 2 = {:?}",
            std::time::Duration::from(std::time::Instant::now() - now)
                .as_millis()
        );
        let now = std::time::Instant::now();
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
        println!(
            "time = {:?}",
            std::time::Duration::from(std::time::Instant::now() - now)
                .as_millis()
        );
        graphics::present(ctx)?;
        Ok(())
    }
}

// hello
pub fn main() -> GameResult {
    let conf = ggez::conf::Conf::new();
    let mut config_file = std::fs::File::create("conf.toml")?;
    conf.to_toml_file(&mut config_file)?;
    println!("Generated conf.toml");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut path = std::path::PathBuf::from(manifest_dir);
    path.push("resources");

    println!("adding resource path, {:?}", path);
    let cb = ggez::ContextBuilder::new(
        "day-11 animation",
        "Jonathan Samson & p88h @ github",
    )
    .add_resource_path(path);

    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
