use miniquad::window::{set_window_ratio, set_window_size};
use macroquad::prelude::*;

#[macroquad::main("Hello Piced Button")]
async fn main() {
    let poppins = load_ttf_font("examples/poppins.ttf").await.unwrap().shared();
    let pic = load_image("examples/rustacean_happy.png").await.unwrap();
    let pic = Texture2D::from_image(&pic);
    let c1 = Color::new(0.05, 0.05, 0.1, 1.0);
    let c2 = Color::new(0.5, 0.5, 1.0, 1.0);
    let c3 = Color::new(1.0, 1.0, 0.5, 1.0);
    let c4 = Color::new(0.5, 0.75, 0.5, 1.0);
    let label = Label::new("Hello, world!".to_string(), CTR_LT, c1, c2, Some(poppins.clone()), 36.0);
    let mut button = Button::new(500.0, 80.0, (0.,0.), "Clickity Clickity Click".to_string(), c4, c1, BLANK, BLANK, Some(poppins.clone()), Some(pic.clone()));
    let mut toggle = Toggle::new(150.0, 50.0, CTR_LT, "Toggle Me".to_string(), c1, c3, BLANK, BLANK, Some(poppins.clone()), Some(pic));
    
    loop {
        clear_background(c1);
        
        label.draw((screen_width() / 2.0 - label.width() / 2.0, screen_height() / 2.0 - label.height() / 2.0 - 100.0));
        
        button.process(PoS::CC);
        button.draw(PoS::CC);
        toggle.process((screen_width() - toggle.width() - 10.0, 10.0));
        toggle.draw((screen_width() - toggle.width() - 10.0, 10.0));
        
        if button.is_clicked() {
            println!("Button clicked!");
        }
        
        next_frame().await;
    }
}
