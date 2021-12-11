//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::event;
use ggez::graphics::{self, Color, DrawMode};
use ggez::{Context, GameResult};
use glam::{IVec2, Vec2};

const INPUT: &str = include_str!("./input.txt");

fn vec_itof(vec: &IVec2) -> Vec2 {
    Vec2::new(vec.x as f32, vec.y as f32)
}

fn list_vec_itof(list: &[IVec2]) -> Vec<Vec2> {
    list.iter().map(|vec| vec_itof(&vec)).collect::<Vec<Vec2>>()
}

fn draw_poly(
    ctx: &mut Context,
    points: &[IVec2],
    color: (u8, u8, u8),
    draw_mode: DrawMode,
) -> GameResult<()> {
    let points = points
        .iter()
        .map(|vec| vec_itof(vec))
        .collect::<Vec<Vec2>>();
    let poly = graphics::Mesh::new_polygon(
        ctx,
        draw_mode,
        &points[..],
        Color::from_rgb(color.0, color.1, color.2),
    )?;
    graphics::draw(ctx, &poly, (Vec2::new(0.0, 0.0),))?;
    Ok(())
}

fn draw_line(
    ctx: &mut Context,
    points: &[IVec2],
    color: (u8, u8, u8),
) -> GameResult<()> {
    let line = ggez::graphics::Mesh::new_line(
        ctx,
        &list_vec_itof(points)[..],
        1.0,
        Color::from_rgb(color.0, color.1, color.2),
    )?;
    graphics::draw(ctx, &line, (Vec2::new(0.0, 0.0),))?;
    Ok(())
}

struct Tile {
    loc: IVec2,
    pos: IVec2,
    level: i32,
    flashing: bool,
}

impl Tile {
    fn new(x: i32, y: i32, h: i32) -> Self {
        Tile {
            loc: IVec2::new(y, x),
            pos: IVec2::new(x * 18 + y * 18, 240 - x * 10 + y * 10),
            level: h,
            flashing: false,
        }
    }

    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let (x, y) = (self.pos.x, self.pos.y);
        let y0 = y - self.level;

        let col1 = match self.flashing {
            true => (160, 255, 160),
            false => (160, 160, 200),
        };
        let col2 = (255, 255, 255);
        let points = [
            IVec2::new(x, y0),
            IVec2::new(x + 16, y0 - 8),
            IVec2::new(x + 32, y0),
            IVec2::new(x + 16, y0 + 8),
        ];
        draw_poly(ctx, &points, col1, DrawMode::fill())?;
        draw_poly(ctx, &points, col2, DrawMode::stroke(1.0))?;
        let col3 = (80, 80, 80);
        let col4 = (40, 40, 40);

        let points = [
            IVec2::new(x, y0),
            IVec2::new(x + 16, y0 + 8),
            IVec2::new(x + 32, y0),
            IVec2::new(x + 32, y + 1),
            IVec2::new(x + 16, y + 1 + 8),
            IVec2::new(x, y + 1),
            IVec2::new(x, y0),
        ];
        draw_poly(ctx, &points[..], col4, DrawMode::fill())?;
        // draw_poly(ctx, &points[..], col4, DrawMode::fill())?;
        // draw_line(ctx, &&[IVec2::new(x, y), IVec2::new(x, y0)][..], col3)?;
        // draw_line(
        //     ctx,
        //     &[IVec2::new(x + 16, y0 + 8), IVec2::new(x + 16, y + 8)][..],
        //     col3,
        // )?;
        // draw_line(
        //     ctx,
        //     &&[IVec2::new(x + 32, y0), IVec2::new(x + 32, y)][..],
        //     col3,
        // )?;

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

    // fn propogate(&mut self, board: &mut Vec<Vec<Tile>>) -> Vec<(i32, i32)> {
    //     let (ti, tj) = (self.loc.x, self.loc.y);
    //     let mut flashed = vec![];
    //     for i in [-1, 0, 1] {
    //         for j in [-1, 0, 1] {
    //             if ti + i >= 0
    //                 && ti + i < board.len() as i32
    //                 && tj + j >= 0
    //                 && tj + j < board[i as usize].len() as i32
    //             {
    //                 if board[i as usize][j as usize].increase(3) {
    //                     flashed.push((i, j))
    //                 }
    //             }
    //         }
    //     }
    //     return flashed;
    // }
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
                    j as i32,
                    i as i32,
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
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        for row in self.board.iter() {
            for tile in row.iter().rev() {
                tile.render(ctx)?;
            }
        }

        // let circle = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     Vec2::new(0.0, 0.0),
        //     50.0,
        //     2.0,
        //     Color::WHITE,
        // )?;
        // let points = [
        //     Vec2::new(0.0, 0.0),
        //     Vec2::new(200.0, 300.0),
        //     Vec2::new(-200.0, 300.0),
        //     Vec2::new(0.0, 0.0),
        // ];
        // let poly = graphics::Mesh::new_polygon(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     &points[..],
        //     Color::WHITE,
        // )?;
        // graphics::draw(
        //     ctx,
        //     &circle,
        //     (Vec2::new(self.ball.loc.x, self.ball.loc.y),),
        // )?;

        // graphics::draw(
        //     ctx,
        //     &poly,
        //     (Vec2::new(self.ball.loc.x, self.ball.loc.y),),
        // )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
