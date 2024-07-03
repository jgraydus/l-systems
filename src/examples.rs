#![allow(unused)]

use std::collections::HashMap;
use crate::l_system::*;
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

pub fn algae() -> LSystem {
    LSystem::new(
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
    )
}

pub fn koch() -> LSystem {
    LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+F-F-F+F".into())
        ]),
        6,
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(90.0)]),
            ('-', vec![TurtleCommand::Turn(-90.0)]),
        ]),
    )
}

pub fn sierpinski() -> LSystem {
    LSystem::new(
        "F-G-G",
        HashMap::from([
            ('F', "F-G+F+G-F".into()),
            ('G', "GG".into()),
        ]),
        7,
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('G', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(120.0)]),
            ('-', vec![TurtleCommand::Turn(-120.0)]),
        ]),
    )
}

pub fn tree() -> LSystem {
    LSystem::new(
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
    )
}

pub fn dragon() -> LSystem {
    LSystem::new(
        "F",
        HashMap::from([
            ('F', "F+G".into()),
            ('G', "F-G".into()),
        ]),
        13,
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('G', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(90.0)]),
            ('-', vec![TurtleCommand::Turn(-90.0)]),
        ])
    )
}

pub fn plant() -> LSystem {
    LSystem::new(
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
    )
}

pub fn levy() -> LSystem {
    LSystem::new(
        "F",
        HashMap::from([
            ('F', "+F--F+".into())
        ]),
        13,
        HashMap::from([
            ('F', vec![TurtleCommand::Move(10.0)]),
            ('+', vec![TurtleCommand::Turn(45.0)]),
            ('-', vec![TurtleCommand::Turn(-45.0)]),
        ]),
    )
}

