use std::cell::RefCell;
use std::rc::Rc;
use macroquad::prelude::*;

#[macroquad::main("Text")]
async fn main() {
    let font = load_ttf_font("./examples/DancingScriptRegular.ttf")
        .await
        .unwrap();
    let font = Rc::new(RefCell::new(font));

    let mut angle = 0.0;

    loop {
        clear_background(BLACK);

        draw_text_ex("Custom font size:", (20.0, 20.0), TEXT_LB, TextParams::default());
        let mut y = 20.0;

        for font_size in (30..100).step_by(20) {
            let text = format!("size = {}", font_size);
            let params = TextParams {
                font_size: font_size as f32,
                ..Default::default()
            };

            y += font_size as f32;
            draw_text_ex(text, (20.0, y), TEXT_LB, params);
        }
        
        draw_text_ex("Dynamic font size:", (320.0, 400.0), TEXT_LB, TextParams::default());
        draw_text_ex(
            "QwQ",
            (320.0, 450.0,),
            TEXT_LB,
            TextParams {
                font_size: get_time().cos() as f32 * 20. + 50.0,
                font_scale: 1.0,
                ..Default::default()
            },
        );
        
        
        draw_text_ex("Dynamic font scale:", (20.0, 400.0), TEXT_LB, TextParams::default());
        draw_text_ex(
            "WoW",
            (20.0, 450.0,),
            TEXT_LB,
            TextParams {
                font_size: 50.0,
                font_scale: get_time().sin() as f32 / 2.0 + 1.0,
                ..Default::default()
            },
        );

        draw_text_ex("Custom font:", (400.0, 20.0), TEXT_LB, TextParams::default());
        draw_text_ex(
            "abcd",
            (400.0, 70.0),
            TEXT_LB,
            TextParams {
                font_size: 50.0,
                font: Some(font.clone()),
                ..Default::default()
            },
        );

        draw_text_ex(
            "abcd",
            (400.0, 160.0),
            TEXT_LB,
            TextParams {
                font_size: 100.0,
                font: Some(font.clone()),
                ..Default::default()
            },
        );

        draw_text_ex(
            "----",
            (screen_width() / 4.0 * 2.0, screen_height() / 3.0 * 2.0),
            TEXT_LB,
            TextParams {
                font_size: 70.0,
                font: Some(font.clone()),
                rotation: angle,
                ..Default::default()
            },
        );

        // let center = get_text_center("OOOO", None, 70.0, 1.0, angle * 2.0);
        // draw_text_ex(
        //     "OOOO",
        //     (screen_width() / 4.0 * 3.0 - center.x, screen_height() / 3.0 * 2.0 - center.y,),
        //     // 先前不存在此字段
        //     TEXT_LB,
        //     TextParams {
        //         font_size: 70.0,
        //         rotation: angle * 2.0,
        //         ..Default::default()
        //     },
        // );
        
        // 上一个被注释掉的示例是旧的旋转原点偏移的实现方案
        draw_text_ex(
            "LT",
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            TEXT_LT,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        
        draw_text_ex(
            "LB",
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            TEXT_LB,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        
        draw_text_ex(
            "RT",
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            TEXT_RT,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        
        draw_text_ex(
            "RB",
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            TEXT_RB,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        draw_text_ex(
            "CC",
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            TEXT_CC,
            TextParams {
                font_size: 70.0,
                rotation: angle * 2.0 + 3.14 / 2.,
                ..Default::default()
            },
        );
        
        draw_circle(
            (screen_width() / 4.0 * 3.0, screen_height() / 3.0 * 2.0),
            2.,
            GREEN
        );

        angle -= 0.030;

        next_frame().await
    }
}
