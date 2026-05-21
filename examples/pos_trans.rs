use macroquad::prelude::*;
use miniquad::window::set_window_size;

/// 你们不觉得盯着几个小球转圈很有意思吗
#[macroquad::main("Hello Position")]
async fn main() {
    set_window_size(800,450);
    
    loop {
        clear_background(DARKGRAY);
        
        let p0 = Ucc::CC;
        let r1 = ((get_time() * 200.) % 360.) as f32;
        let p1 =
            VecChain::new(p0.to_physical_vec())
                .join_back( |p| rotate_pos(Ucc(0.1,0.), p, r1));
        let p2 =
            p1.clone()
                .join_back( |p| refer_pos(VeC(0.1,0.), p, r1));
        let r3 = ((get_time() * 100.) % 360.) as f32;
        let p3 =
            p1.clone()
                .join_back( move |p| rotate_pos(Ucc::CC, p, r3));
        
        let p1r = p1.to_physical_vec();
        draw_text(format!("p1: {:?}", p1r), (0.,0.), CTR_LT, 20.0, WHITE);
        draw_circle(
            p1r,
            5.,
            WHITE
        );
        
        let p2r = p2.to_physical_vec();
        draw_text(format!("p2: {:?}", p2r), (0.,20.), CTR_LT, 20.0, WHITE);
        draw_circle(
            p2r,
            5.,
            WHITE
        );
        
        
        let p3r = p3.to_physical_vec();
        draw_text(format!("p3: {:?}", p2r), (0.,40.), CTR_LT, 20.0, WHITE);
        draw_circle(
            p3r,
            5.,
            WHITE
        );
        
        next_frame().await;
    }
}
