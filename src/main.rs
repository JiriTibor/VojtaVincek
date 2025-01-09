use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]

async fn main() {
    let mut color = RED;

    let mut frame = 0;
    const SPEED: i32 = 30;
    
    let texture: Texture2D = load_texture("cat.png").await.unwrap();
    
    loop {
        if frame == SPEED {
            if color == RED {color = BLUE}
            else {color = RED}
            frame = 0;
        }
        
        clear_background(YELLOW);    

        draw_rectangle(screen_width() / 2.0 - 120.0, 20.0, 280.0, 60.0, color);
        draw_text("AHOJ VOJTO", screen_width() / 2.0 - 120.0, 60.0, 60.0, GREEN);

        draw_texture(&texture, 0., 0., WHITE);

        frame += 1;
        next_frame().await
    }
}
