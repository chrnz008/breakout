use macroquad::prelude::*;

//for ball dxy is direction for pad it is width and height
struct Dir {
    dx: f32,
    dy: f32,
}

#[macroquad::main("breakout")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const BALL_RAD: f32 = 10f32;
    const PAD_LENGTH: f32 = 100f32;
    const PAD_WIDTH: f32 = 20f32;
    // const VILL_SIZE: u32 = 4;

    let screen_width = screen_width();
    let screen_height = screen_height();

    //ball direction
    let mut ball_dir: Dir = Dir { dx: 1f32, dy: 1f32 };

    let mut pad_pos = Rect {
        x: (screen_width - PAD_LENGTH) / 2.0,
        y: screen_height - 50.0,
        w: PAD_LENGTH,
        h: PAD_WIDTH,
    };

    let mut ball_pos = Circle {
        x: (screen_width) / 2.0,
        y: screen_height - 50.0 - BALL_RAD,
        r: BALL_RAD,
    };

    let mut pause: bool = false;

    loop {
        clear_background(DARKGRAY);

        if !pause {
            //game logic
            let delta_time = get_frame_time();

            //temporary vars for storing motion state
            let nx = ball_pos.x + MOVEMENT_SPEED * delta_time * ball_dir.dx;
            let ny = ball_pos.y + MOVEMENT_SPEED * delta_time * ball_dir.dy;

            let nballpos = Circle {
                x: nx,
                y: ny,
                ..ball_pos
            };

            //collision with walls
            if !(ball_pos.r..screen_width - ball_pos.r).contains(&nx)
                || nballpos.overlaps_rect(&pad_pos)
            {
                ball_dir.dx *= -1.0;
            }
            if !(ball_pos.r..screen_height - ball_pos.r).contains(&ny)
                || nballpos.overlaps_rect(&pad_pos)
            {
                ball_dir.dy *= -1.0;
            }

            //updated postion
            ball_pos.x = nx;
            ball_pos.y = ny;

            //pad control
            if is_key_down(KeyCode::Right) {
                pad_pos.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                pad_pos.x -= MOVEMENT_SPEED * delta_time;
            }

            pad_pos.x = clamp(pad_pos.x, 0.0, screen_width - pad_pos.h);
        }
        draw_circle(ball_pos.x, ball_pos.y, ball_pos.r, YELLOW);
        //pause the game state
        if is_key_down(KeyCode::Space) {
            pause = !pause;
        }

        draw_rectangle(pad_pos.x, pad_pos.y, pad_pos.w, pad_pos.h, RED);

        next_frame().await
    }
}
