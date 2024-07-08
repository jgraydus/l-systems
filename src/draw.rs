use web_sys;
use wasm_bindgen::prelude::*;
use futures::stream::{Stream, StreamExt};
use std::pin::{Pin};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x0: f64, pub x1: f64, pub y0: f64, pub y1: f64
}

#[derive(Clone, Debug)]
pub enum DrawCommand {
    BeginPath,
    SetLineWidth(f64),
    SetStrokeStyle(String),
    MoveTo(f64, f64),
    LineTo(f64, f64),
    Stroke,
}

impl DrawCommand {
    fn exec(&self,
            context: &web_sys::CanvasRenderingContext2d,
            delta_x: f64,
            delta_y: f64,
            alpha: f64,
            beta: f64) {
        match self {
            DrawCommand::BeginPath => {
                context.begin_path();
            }
            DrawCommand::SetLineWidth(lw) => {
                context.set_line_width(*lw);
            }
            DrawCommand::SetStrokeStyle(s) => {
                context.set_stroke_style(&JsValue::from_str(&s));
            }
            DrawCommand::MoveTo(x, y) => {
                let x = (*x - delta_x) * alpha;
                let y = (*y - delta_y) * beta;
                context.move_to(x, y);
            }
            DrawCommand::LineTo(x, y) => {
                let x = (*x - delta_x) * alpha;
                let y = (*y - delta_y) * beta;
                context.line_to(x, y);
            }
            DrawCommand::Stroke => {
                context.stroke();
            }
        }
    }

    pub fn exec_all(commands: &Vec<DrawCommand>,
                    ctx: &web_sys::CanvasRenderingContext2d,
                    viewport: Viewport) {
        let canvas = ctx.canvas().expect("canvas missing!");
        let (width, height) = (canvas.width() as f64, canvas.height() as f64);
        let Viewport { x0, x1, y0, y1 } = viewport;
        let alpha = width / (x1 - x0);
        let beta = height / (y0 - y1);

        for command in commands.iter() {
            command.exec(ctx, x0, y1, alpha, beta);
        }
    }

    pub async fn exec_stream(commands: &mut Pin<Box<dyn Stream<Item=DrawCommand>>>,
                             ctx: &web_sys::CanvasRenderingContext2d,
                             viewport: Viewport) {
        let canvas = ctx.canvas().expect("canvas missing!");
        let (width, height) = (canvas.width() as f64, canvas.height() as f64);
        let Viewport { x0, x1, y0, y1 } = viewport;
        let alpha = width / (x1 - x0);
        let beta = height / (y0 - y1);

        while let Some(command) = commands.next().await {
            command.exec(ctx, x0, y1, alpha, beta);
        }
    }
}

