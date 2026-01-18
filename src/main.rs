use macroquad::prelude::*;

//for ball dxy is direction for pad it is width and height
struct Pos{
    x:f32,
    y:f32,
}

struct Dir{
    dx:f32,
    dy:f32,
}

#[macroquad::main("breakout")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    const BALL_RAD:f32 = 10f32;
    const PAD_LENGTH:f32 = 100f32;
    const PAD_WIDTH:f32 = 20f32;
    const VILL_SIZE:u32 = 4;

    let screen_width = screen_width();
    let screen_height = screen_height();

    //ball direction
    let mut ball_dir:Dir= Dir{
        dx:1f32,
        dy:1f32,
    };

    let mut pad_pos :Pos = Pos{
        x:(screen_width - PAD_LENGTH)/2.0,
        y:screen_height-50.0,
    };

    let mut ball_pos :Pos = Pos{
        x: pad_pos.x+PAD_LENGTH/2.0,
        y: pad_pos.y-10.0,
    };


    // Vec<bool> villans =vec![true;VILL_SIZE];

    loop{
        clear_background(DARKGRAY);

        let delta_time = get_frame_time();

        //collision with walls
        //TODO:split the logic of pad collsion
        //collision with pad after || (floating point comparision is a mess)

        if !(BALL_RAD..screen_width-BALL_RAD).contains(&ball_pos.x) ||((pad_pos.y..=pad_pos.y+PAD_WIDTH).contains(&ball_pos.y)&&(pad_pos.x..=pad_pos.x+PAD_LENGTH).contains(&ball_pos.x)) {
            ball_dir.dx*=-1.0;
        }
        if !(BALL_RAD..screen_height-BALL_RAD).contains(&ball_pos.y) || ((pad_pos.x..=pad_pos.x+PAD_LENGTH).contains(&ball_pos.x) && ( pad_pos.y-ball_pos.y-BALL_RAD)<=0.0){
            ball_dir.dy*=-1.0;
        }

        //ball motion
        ball_pos.x+=MOVEMENT_SPEED*delta_time*ball_dir.dx;
        ball_pos.y+=MOVEMENT_SPEED*delta_time*ball_dir.dy;

        draw_circle(ball_pos.x,ball_pos.y,BALL_RAD,YELLOW);

        //pad control
        if is_key_down(KeyCode::Right) {
           pad_pos.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            pad_pos.x -= MOVEMENT_SPEED * delta_time;
        }

        pad_pos.x = clamp(pad_pos.x, 0.0, screen_width-PAD_LENGTH);

        draw_rectangle(pad_pos.x,pad_pos.y,PAD_LENGTH,PAD_WIDTH,RED);

        //draw villans
        // for vil in villans {
        //     if (vil){
        //         draw_rectangle(pad_pos.x,pad_pos.y,PAD_LENGTH,PAD_WIDTH,DARKPURPLE);
        //     }
        // }

        next_frame().await
    }
}
