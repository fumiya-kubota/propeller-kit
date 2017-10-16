extern crate ggez;
use ggez::{GameResult, Context, conf};
use ggez::event;
use ggez::graphics;
use std::time::Duration;

struct MainState {
    pos_x: f32,
    text: graphics::Text
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(_ctx, "/rounded-mgenplus-1c-regular.ttf", 32)?;
        let text = graphics::Text::new(_ctx, "ほげほげ", &font)?;
        let s = MainState {pos_x: 0.0, text};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         graphics::DrawMode::Fill,
                         graphics::Point {
                             x: self.pos_x,
                             y: 380.0,
                         },
                         100.0,
                         32)?;
        for x in 0..100 {
            graphics::draw(ctx, &self.text, graphics::Point{x: 5.0 * x as f32, y: 100.0}, 0.0)?;
        }
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_width = 800;
    c.window_height = 600;
    let c = c;
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}