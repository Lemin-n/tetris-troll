use macroquad::{
    prelude::{vec2, Vec2, PURPLE},
    shapes::draw_rectangle,
};

use crate::tetromino::{Clock, Offset, PieceMat4};

pub struct TetrioT;
impl TetrioT {
    #[allow(unused)]
    pub(crate) fn draw(t: &crate::tetromino::Tetromino, block: &Vec2) {
        match t.current_rotation {
            Clock::P12 => {
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P3 => {
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P6 => {
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P9 => {
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
        }
    }

    pub(crate) fn mat4(tetro: &crate::tetromino::Tetromino) -> (PieceMat4, Offset) {
        match tetro.current_rotation {
            Clock::P12 => (
                [
                    //? T
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 6, 6, 6],
                    [0, 0, 6, 0],
                ],
                Offset {
                    up: 2,
                    down: 0,
                    left: 1,
                    right: 0,
                },
            ),
            Clock::P3 => (
                [
                    //? T
                    [0, 0, 0, 0],
                    [0, 0, 6, 0],
                    [0, 6, 6, 0],
                    [0, 0, 6, 0],
                ],
                Offset {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                },
            ),
            Clock::P6 => (
                [
                    //? T
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 6, 0],
                    [0, 6, 6, 6],
                ],
                Offset {
                    up: 2,
                    down: 0,
                    left: 1,
                    right: 0,
                },
            ),
            Clock::P9 => (
                [
                    //? T
                    [0, 0, 0, 0],
                    [0, 6, 0, 0],
                    [0, 6, 6, 0],
                    [0, 6, 0, 0],
                ],
                Offset {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                },
            ),
        }
    }

    pub(crate) fn color() -> macroquad::prelude::Color {
        PURPLE
    }

    pub(crate) fn size(rotation: Clock) -> macroquad::prelude::Vec2 {
        match rotation {
            Clock::P12 => vec2(3.0, 2.0),
            Clock::P3 => vec2(2.0, 3.0),
            Clock::P6 => vec2(3.0, 2.0),
            Clock::P9 => vec2(2.0, 3.0),
        }
    }
}
