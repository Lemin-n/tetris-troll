use macroquad::{
    prelude::{Vec2, SKYBLUE},
    shapes::draw_rectangle,
};

use crate::tetromino::{Clock, Offset, PieceMat4};

pub struct TetrioI;
impl TetrioI {
    pub(crate) fn draw(t: &crate::tetromino::Tetromino, block: &Vec2) {
        match t.rotation {
            Clock::P12 => {
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
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
                    0. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    3. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P3 => {
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
                draw_rectangle(
                    3. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    4. * block.x + (t.props.x * block.x),
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
                    0. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    3. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P9 => {
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
                draw_rectangle(
                    3. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    4. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
        }
    }

    pub(crate) fn mat4(tetro: &crate::tetromino::Tetromino) -> (PieceMat4, Offset) {
        match tetro.rotation {
            Clock::P12 => (
                [
                    //? I
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                ],
                Offset {
                    up: 0,
                    down: 0,
                    left: 1,
                    right: 2,
                },
            ),
            Clock::P3 => (
                [
                    //? I
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                Offset {
                    up: 1,
                    down: 2,
                    left: 0,
                    right: 0,
                },
            ),
            Clock::P6 => (
                [
                    //? I
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                    [0, 1, 0, 0],
                ],
                Offset {
                    up: 0,
                    down: 0,
                    left: 1,
                    right: 2,
                },
            ),
            Clock::P9 => (
                [
                    //? I
                    [0, 0, 0, 0],
                    [1, 1, 1, 1],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                ],
                Offset {
                    up: 1,
                    down: 2,
                    left: 0,
                    right: 0,
                },
            ),
        }
    }

    pub(crate) fn color() -> macroquad::prelude::Color {
        SKYBLUE
    }
}
