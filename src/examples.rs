#![allow(unused)]

use std::collections::HashMap;
use crate::l_system::*;
use crate::parser::*;
use crate::turtle::*;

pub fn turtle_example() -> TurtleProgram {
    TurtleProgram {
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
    }
}

type LSystemExample = (&'static str, &'static str, fn() -> LSystem);

pub const ALGAE: LSystemExample = (
    "algae",
r#"LSYSTEM (
    A,
    (A -> AB, B -> A),
    (A -> (MOVE 40), B -> (MOVE 40, TURN 25))
)"#,
    || LSystem::new(
        "A",
        HashMap::from([
            ('A', "AB".into()),
            ('B', "A".into()),
        ]),
        HashMap::from([
            ('A', vec![TurtleCommand::Move(40.0)]),
            ('B', vec![TurtleCommand::Move(40.0), TurtleCommand::Turn(25.0)]),
        ]),
    )
);

pub const KOCH: LSystemExample = (
    "koch",
r#"LSYSTEM (
    F,
    (F -> F+F-F-F+F),
    (F -> (MOVE 10), + -> (TURN 90), - -> (TURN -90))
)"#, 
    || LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+F-F-F+F".into())
        ]),
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(90.0)]),
            ('-', vec![TurtleCommand::Turn(-90.0)]),
        ]),
    )
);

pub const SIERPINSKI: LSystemExample = (
    "sierpinski",
r#"LSYSTEM (
    F-G-G,
    (F -> F-G+F+G-F, G -> GG),
    (F -> (MOVE 10), G -> (MOVE 10), + -> (TURN 120), - -> (TURN -120))
)"#,
    || LSystem::new(
        "F-G-G",
        HashMap::from([
            ('F', "F-G+F+G-F".into()),
            ('G', "GG".into()),
        ]),
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('G', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(120.0)]),
            ('-', vec![TurtleCommand::Turn(-120.0)]),
        ]),
    )
);

pub const TREE: LSystemExample = (
    "tree",
r#"LSYSTEM (
    [0]++[0]++[0]++[0],
    (1 -> 11, 0 -> 1[+0]-0),
    (0 -> (MOVE 5),
     1 -> (MOVE 5),
     [ -> (PUSH),
     ] -> (POP),
     + -> (TURN 45),
     - -> (TURN -45))
)"#,
    || LSystem::new(
        "[0]++[0]++[0]++[0]",
        HashMap::from([
            ('1', "11".into()),
            ('0', "1[+0]-0".into()),
        ]),
        HashMap::from([
            ('0', vec![TurtleCommand::Move(5.0)]),
            ('1', vec![TurtleCommand::Move(5.0)]),
            ('[', vec![TurtleCommand::Push]),
            (']', vec![TurtleCommand::Pop]),
            ('+', vec![TurtleCommand::Turn(45.0)]),
            ('-', vec![TurtleCommand::Turn(-45.0)]),
        ]),
    )
);

pub const DRAGON: LSystemExample = (
    "dragon",
r#"LSYSTEM (
    F,
    (F -> F+G, G -> F-G),
    (F -> (MOVE 10),
     G -> (MOVE 10),
     + -> (TURN 90),
     - -> (TURN -90))
)"#,
    || LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+G".into()),
            ('G', "F-G".into()),
        ]),
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('G', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(90.0)]),
            ('-', vec![TurtleCommand::Turn(-90.0)]),
        ])
    )
);

pub const PLANT: LSystemExample = (
    "plant",
r#"LSYSTEM (
    ++X,
    (X -> F+[[X]-X]-F[-FX]+X, F -> FF),
    (F -> (MOVE 10),
     + -> (TURN 25),
     - -> (TURN -25),
     [ -> (PUSH),
     ] -> (POP))
)"#,
    || LSystem::new(
        "++X",
        HashMap::from([
            ('X', "F+[[X]-X]-F[-FX]+X".into()),
            ('F', "FF".into()),
        ]),
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(25.0)]),
            ('-', vec![TurtleCommand::Turn(-25.0)]),
            ('[', vec![TurtleCommand::Push]),
            (']', vec![TurtleCommand::Pop]),
        ]),
    )
);

pub const LEVY: LSystemExample = (
    "levy",
r#"LSYSTEM (
    F,
    (F -> +F--F+),
    (F -> (MOVE 10), + -> (TURN 45), - -> (TURN -45))
)"#,
    || LSystem::new(
        "F",
        HashMap::from([
            ('F', "+F--F+".into())
        ]),
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(45.0)]),
            ('-', vec![TurtleCommand::Turn(-45.0)]),
        ]),
    )
);

pub fn all_examples() -> [LSystemExample; 7] {
    [ALGAE, KOCH, SIERPINSKI, TREE, DRAGON, PLANT, LEVY]
}
