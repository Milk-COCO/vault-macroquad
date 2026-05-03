use macroquad::prelude::*;

#[macroquad::main("Wheel Test")]
async fn main() {
    
    loop {
        clear_background(BLACK);
        
        let wheel = mouse_wheel();
        
        draw_text(&format!("X: {:.2}, Y: {:.2}", wheel.0, wheel.1), (10.0, 10.0), CTR_LT, 20.0, GREEN);
        
        next_frame().await;
    }
}