mod turtle;
mod l_system;

use std::collections::HashMap;
use web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

use turtle::*;
use l_system::*;

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        let canvas = get_canvas();

        let context = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let canvas_size = (canvas.width() as f64, canvas.height() as f64);
/*
        let program = TurtleProgram {
            turtle: Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 3.0,
                    state: PenState::Down,
                }
            },
            commands: vec![
                TurtleCommand::Repeat(8, vec![
                    TurtleCommand::Push,
                    TurtleCommand::PenUp,
                    TurtleCommand::Move(200.0),
                    TurtleCommand::PenDown,
                    TurtleCommand::Repeat(18, vec![
                        TurtleCommand::Push,
                        TurtleCommand::Move(50.0),
                        TurtleCommand::Pop,
                        TurtleCommand::Turn(20.0),
                    ]),
                    TurtleCommand::Pop,
                    TurtleCommand::Turn(45.0),
                ]),
            ],
            canvas_size,
        };

        program.execute(&context);
*/
/*
        let algae = LSystem::new(
            "A",
            HashMap::from([
                ('A', "AB".into()),
                ('B', "A".into()),
            ]),
            5,
            HashMap::from([
                ('A', vec![TurtleCommand::Move(40.0)]),
                ('B', vec![TurtleCommand::Move(40.0), TurtleCommand::Turn(25.0)]),
            ]),
        );
        algae.compile(canvas_size).execute(&context);

        let koch = LSystem::new(
            "F",
            HashMap::from([
                ('F', "F+F-F-F+F".into())
            ]),
            4,
            HashMap::from([
                ('F', vec![TurtleCommand::Move(10.0)]),
                ('+', vec![TurtleCommand::Turn(90.0)]),
                ('-', vec![TurtleCommand::Turn(-90.0)]),
            ]),
        );
        koch.compile(canvas_size).execute(&context);

        let sierpinski = LSystem::new(
            "F-G-G",
            HashMap::from([
                ('F', "F-G+F+G-F".into()),
                ('G', "GG".into()),
            ]),
            6,
            HashMap::from([
                ('F', vec![TurtleCommand::Move(10.0)]),
                ('G', vec![TurtleCommand::Move(10.0)]),
                ('+', vec![TurtleCommand::Turn(120.0)]),
                ('-', vec![TurtleCommand::Turn(-120.0)]),
            ]),
        );
        sierpinski.compile(canvas_size).execute(&context);

        let tree = LSystem::new(
            "[0]++[0]++[0]++[0]",
            HashMap::from([
                ('1', "11".into()),
                ('0', "1[+0]-0".into()),
            ]),
            6,
            HashMap::from([
                ('0', vec![TurtleCommand::Move(5.0)]),
                ('1', vec![TurtleCommand::Move(5.0)]),
                ('[', vec![TurtleCommand::Push]),
                (']', vec![TurtleCommand::Pop]),
                ('+', vec![TurtleCommand::Turn(45.0)]),
                ('-', vec![TurtleCommand::Turn(-45.0)]),
            ]),
        );
        tree.compile(canvas_size).execute(&context);

        let dragon = LSystem::new(
            "F",
            HashMap::from([
                ('F', "F+G".into()),
                ('G', "F-G".into()),
            ]),
            12,
            HashMap::from([
                ('F', vec![TurtleCommand::Move(10.0)]),
                ('G', vec![TurtleCommand::Move(10.0)]),
                ('+', vec![TurtleCommand::Turn(90.0)]),
                ('-', vec![TurtleCommand::Turn(-90.0)]),
            ])
        );
        dragon.compile(canvas_size).execute(&context);

        let plant = LSystem::new(
            "++X",
            HashMap::from([
                ('X', "F+[[X]-X]-F[-FX]+X".into()),
                ('F', "FF".into()),
            ]),
            5,
            HashMap::from([
                ('F', vec![TurtleCommand::Move(10.0)]),
                ('X', vec![]),
                ('+', vec![TurtleCommand::Turn(25.0)]),
                ('-', vec![TurtleCommand::Turn(-25.0)]),
                ('[', vec![TurtleCommand::Push]),
                (']', vec![TurtleCommand::Pop]),
            ]),
        );
        plant.compile(canvas_size).execute(&context);
*/
        let levy = LSystem::new(
            "F",
            HashMap::from([
                ('F', "+F--F+".into())
            ]),
            10,
            HashMap::from([
                ('F', vec![TurtleCommand::Move(10.0)]),
                ('+', vec![TurtleCommand::Turn(45.0)]),
                ('-', vec![TurtleCommand::Turn(-45.0)]),
            ]),
        );
        levy.compile(canvas_size).execute(&context);
        
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::turtle::*;

    #[test]
    fn test_01() {
        let mut program = TurtleProgram {
            turtle: Turtle {
                location: (0.0, 0.0),
                orientation: 0.0,
                pen: Pen {
                    color: (1.0, 1.0, 1.0),
                    width: 3.0,
                    state: PenState::Down,
                }
            },
            commands: vec![
                TurtleCommand::Turn(45.0_f64.to_radians()),
                TurtleCommand::Move(100.0),
                TurtleCommand::Turn(45.0_f64.to_radians()),
                TurtleCommand::Move(100.0),
            ],
            canvas_size: (0.0, 0.0),
        };

        let result = program.run();
        println!("{:?}", result);
    }
}

