use macroquad::prelude::*;

struct Coso {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("TetrisTroll")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Coso> = vec![];
    let mut circle = Coso {
        size: 52.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);

        //? input handlers❗
        // * @see https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }

        //? Clamp X and Y to be within the screen
        circle.x = circle.x.min(screen_width()).max(0.0);
        circle.y = circle.y.min(screen_height()).max(0.0);

        //? instances
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Coso {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(x - 30.0, y - 30.0, 45.0, BROWN);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        //? game world❗
        for cosito in &mut squares {
            cosito.y += cosito.speed * delta_time;
        }
        //? optimization: Remove squares below bottom of screen
        squares.retain(|square| square.y < screen_width() + square.size);

        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for cosito in &squares {
            draw_rectangle(
                cosito.x - cosito.size / 2.0,
                cosito.y - cosito.size / 2.0,
                cosito.size,
                cosito.size,
                PINK,
            );
        }
        next_frame().await
    }
}
