use day_13::Paper;

use ggez::event;
use ggez::graphics::{self, Color, DrawMode, DrawParam, FilterMode, Rect, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use glam::Vec2;
use keyframe::{ease, functions::*, AnimationSequence, EasingFunction};
use std::{env, path};

#[macro_use]
use keyframe::keyframes;

const MARGIN: f32 = 100.0;
const TWO_MARGIN: f32 = MARGIN * 2.0;

struct FoldAnimation {
    scale_animation: AnimationSequence<Point2<f32>>,
}

fn scale_animation(starting_scale: Point2<f32>, duration: f32) -> AnimationSequence<Point2<f32>> {
    keyframes![
        (starting_scale, 0.0, EaseInOut),
        (
            [-starting_scale.x, starting_scale.y].into(),
            duration,
            EaseInOut
        )
    ]
}

struct MainState {
    ball_seq: AnimationSequence<Point2<f32>>,
    paper: Paper,
    fold_animation: Option<FoldAnimation>,
}

static INPUT: &str = include_str!("input.txt");
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            ball_seq: ball_sequence([0.0, 0.0].into(), [300.0, 300.0].into()),
            paper: Paper::new(INPUT),
            fold_animation: None,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let secs = ggez::timer::delta(ctx).as_secs_f64();
        self.ball_seq.advance_and_maybe_reverse(secs);

        if let Some(ref mut ani) = self.fold_animation {
            ani.scale_animation.advance_and_maybe_reverse(secs);
        } else {
            let bounds = self.paper.bounds();
            let max = std::cmp::max(bounds.0, bounds.1) as f32;
            let scale = (ggez::graphics::screen_coordinates(ctx).w - TWO_MARGIN) / max;
            self.fold_animation = Some(FoldAnimation {
                scale_animation: scale_animation([scale, scale].into(), 5.0),
            });
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("{}", ggez::timer::fps(ctx));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        draw_info(ctx, format!("Hello!"), [300.0, 60.0].into())?;

        let mut mb = graphics::MeshBuilder::new();
        for dot in self.paper.dots.iter() {
            mb.circle(
                DrawMode::fill(),
                [dot.0 as f32, dot.1 as f32],
                3.0,
                0.2,
                [1.0, 0.5, 0.5, 0.7].into(),
            )?;
        }
        let mesh = mb.build(ctx)?;

        let bounds = self.paper.bounds();
        let max = std::cmp::max(bounds.0, bounds.1) as f32;
        // let scale = (ggez::graphics::screen_coordinates(ctx).w - TWO_MARGIN) / max;
        let scale = match &self.fold_animation {
            Some(ani) => ani.scale_animation.now_strict().unwrap(),
            None => return Ok(()),
        };

        let draw_param = DrawParam::new()
            .offset([-MARGIN / scale.x, -MARGIN / scale.y])
            .scale(scale);
        graphics::draw(ctx, &mesh, draw_param)?;
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            [MARGIN, MARGIN, scale.x * max, scale.y * max].into(),
            Color::RED,
        )?;
        graphics::draw(ctx, &rectangle, DrawParam::new())?;

        let ball = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            60.0,
            1.0,
            Color::WHITE,
        )?;

        let ball_pos = self.ball_seq.now_strict().unwrap();
        graphics::draw(ctx, &ball, (ball_pos,))?;

        graphics::present(ctx)?;
        // ggez::timer::sleep(std::time::Duration::from_millis(15));
        Ok(())
    }
}

fn ball_sequence(
    ball_pos_start: Point2<f32>,
    ball_pos_end: Point2<f32>,
) -> AnimationSequence<Point2<f32>> {
    let duration = 1.0;
    keyframes![
        (ball_pos_start, 0.0, EaseInOut),
        (ball_pos_end, duration, EaseInOut)
    ]
}

fn draw_info(ctx: &mut Context, info: String, position: Point2<f32>) -> GameResult {
    let t = Text::new(TextFragment {
        text: info,
        font: None,
        scale: Some(ggez::graphics::PxScale::from(40.0)),
        ..Default::default()
    });
    graphics::draw(
        ctx,
        &t,
        DrawParam::default().dest(position).color(Color::WHITE),
    )
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Advent of Code, 2021, day 13 animation", "Jonathan Samson")
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
