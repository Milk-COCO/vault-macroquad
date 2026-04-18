use miniquad::window::screen_size;
use macroquad::measure::{PoS, VeC};
use macroquad::prelude::*;

#[macroquad::main("Measure measures a measure!")]
async fn main() {
    init_box();
    ui_box().insert(
        "a button",
        Button::default()
            .with_text("a W".to_string())
    );
    ui_box().insert(
        "b button",
        Button::default()
            .with_text("b H".to_string())
    );
    ui_box().insert(
        "c button",
        Button::default()
            .with_text("c WH".to_string())
    );
    let mut screen_size = screen_size();
    loop {
        let p1 = PoS::CT.to_physical();
        let p2 = PoS::LC.to_physical();
        let p3 = PoS::CC.to_physical();
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        if screen_size != miniquad::window::screen_size() {
            screen_size = miniquad::window::screen_size();
            info!("{:?} , to {:?} {:?} {:?} ", screen_size, p1,p2,p3);
        }
        
        let a = ui_box().button("a button");
        if a.size(
            VeC::ww(0.1)
        ).process(p1).is_clicked() {
            info!("a");
        }
        a.draw(p1);
        
        let b = ui_box().button("b button");
        if b.size(
            VeC::hh(0.1)
        ).process(p2).is_clicked() {
            info!("b");
        }
        b.draw(p2);
        
        let c = ui_box().button("c button");
        if c.size(
            VeC::splat(0.1)
        ).process(p3).is_clicked() {
            info!("c");
        }
        c.draw(p3);
        
        next_frame().await;
    }
}
