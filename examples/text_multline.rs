use macroquad::prelude::*;

#[macroquad::main("Text Multiline")]
async fn main() {
    let mut angle = 0.0;
    
    loop {
        clear_background(BLACK);
        
        let pos = (screen_width()/2.,screen_height()/2.);
        draw_circle(pos,2.,GREEN);
        
        draw_multiline_text_ex(
            "ccb\nccb!",
            pos,
            TEXT_LT,
            Some(1.0),
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        
        
        draw_text_ex(
            "_OOO",
            pos,
            TEXT_RB,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        angle -= 0.030;
        
        next_frame().await
    }
}
