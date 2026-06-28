use miniquad::window::set_ime_enabled;
use macroquad::prelude::*;

#[macroquad::main("Line Test")]
async fn main() {
    set_ime_enabled(false);

    // 变量定义
    let mut show_points = true;

    let mut origin_x = screen_width() as f64 / 2.0;
    let mut origin_y = screen_height() as f64 / 2.0;
    let mut angle = 45.0f64;
    let mut thickness = 5.0f64;

    let mut p1_x = screen_width() as f64 / 2.0 - 100.0;
    let mut p1_y = screen_height() as f64 / 2.0;
    let mut p2_x = screen_width() as f64 / 2.0 + 100.0;
    let mut p2_y = screen_height() as f64 / 2.0;

    // Mode 4 专用：线段长度
    let mut seg_length = 200.0f64;

    let mut mode = 1; // 1: Anchor, 2: Through, 3: Segment(2pts), 4: Segment(Anchor)

    let colors = [WHITE, RED, GREEN, BLUE, YELLOW, PURPLE];
    let mut color_idx = 0;

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        if is_key_pressed(KeyCode::Key0) { show_points = false; }
        else if is_key_pressed(KeyCode::Key9) { show_points = true; }

        // --- 模式切换 ---
        if is_key_pressed(KeyCode::Key1) { mode = 1; }
        if is_key_pressed(KeyCode::Key2) { mode = 2; }
        if is_key_pressed(KeyCode::Key3) { mode = 3; }
        if is_key_pressed(KeyCode::Key4) { mode = 4; }

        let mut speed = 5.0;
        let mut rad = 0.1;
        macro_rules! speed_up_with {
            ($key: expr) => {
                if is_key_down($key) {
                    speed *= speed;
                    rad *= 10.;
                }
            };
        }


        macro_rules! speed_100_with {
            ($key: expr) => {
                if is_key_down($key) {
                    speed *= 100.;
                    rad *= 10.;
                }
            };
        }
        macro_rules! speed_down_with {
            ($key: expr) => {
                if is_key_down($key) {
                    speed = 1./speed;
                    rad /= 1000.;
                }
            };
        }
        speed_up_with!(KeyCode::LeftControl);
        speed_up_with!(KeyCode::LeftShift);
        speed_up_with!(KeyCode::LeftAlt);
        speed_up_with!(KeyCode::CapsLock);
        speed_100_with!(KeyCode::RightControl);
        speed_100_with!(KeyCode::RightShift);
        speed_100_with!(KeyCode::RightAlt);
        speed_down_with!(KeyCode::C);
        speed_down_with!(KeyCode::V);
        speed_down_with!(KeyCode::X);
        speed_down_with!(KeyCode::Z);

        // --- 控制逻辑 ---
        match mode {
            1 | 4 => {
                // 模式 1 & 4: WASD 控原点, QE 控角度
                if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { origin_y -= speed; }
                if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { origin_y += speed; }
                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { origin_x -= speed; }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { origin_x += speed; }

                if is_mouse_button_down(MouseButton::Left) {
                    (origin_x, origin_y) = (mouse_position().0 as f64, mouse_position().1 as f64);
                }

                if is_key_down(KeyCode::Q) { angle -= rad; }
                if is_key_down(KeyCode::E) { angle += rad; }

                // 模式 4 特有：Z/X 控长度；模式 1：Z/X 控粗细
                if mode == 4 {
                    if is_key_down(KeyCode::Z) { seg_length -= speed; }
                    if is_key_down(KeyCode::X) { seg_length += speed; }
                } else {
                    if is_key_down(KeyCode::Z) { thickness -= rad; }
                    if is_key_down(KeyCode::X) { thickness += rad; }
                }
            },
            2 | 3 => {
                // 模式 2 & 3: WASD 控 P1, 方向键控 P2
                if is_key_down(KeyCode::W) { p1_y -= speed; }
                if is_key_down(KeyCode::S) { p1_y += speed; }
                if is_key_down(KeyCode::A) { p1_x -= speed; }
                if is_key_down(KeyCode::D) { p1_x += speed; }

                if is_key_down(KeyCode::Up) { p2_y -= speed; }
                if is_key_down(KeyCode::Down) { p2_y += speed; }
                if is_key_down(KeyCode::Left) { p2_x -= speed; }
                if is_key_down(KeyCode::Right) { p2_x += speed; }

                if is_mouse_button_down(MouseButton::Left) {
                    (p1_x, p1_y) = (mouse_position().0 as f64, mouse_position().1 as f64);
                }

                if is_mouse_button_down(MouseButton::Right) {
                    (p2_x, p2_y) = (mouse_position().0 as f64, mouse_position().1 as f64);
                }

                // Z/X 控粗细
                if is_key_down(KeyCode::Z) { thickness -= rad; }
                if is_key_down(KeyCode::X) { thickness += rad; }
            },
            _ => {}
        }

        if is_key_pressed(KeyCode::Space) {
            color_idx = (color_idx + 1) % colors.len();
        }

        // 重置
        if is_key_pressed(KeyCode::R) {
            origin_x = screen_width() as f64 / 2.0;
            origin_y = screen_height() as f64 / 2.0;
            angle = 45.0;
            p1_x = screen_width() as f64 / 2.0 - 100.0;
            p1_y = screen_height() as f64 / 2.0;
            p2_x = screen_width() as f64 / 2.0 + 100.0;
            p2_y = screen_height() as f64 / 2.0;
            seg_length = 200.0;
            thickness = 5.0;
        }

        // --- 绘制 ---
        match mode {
            1 => {
                draw_infinite_line(
                    (origin_x, origin_y),
                    angle as f32,
                    thickness as f32,
                    colors[color_idx]
                );
                if show_points {
                    draw_circle((origin_x, origin_y), 8.0, RED);
                    draw_circle_lines((origin_x, origin_y), 8.0, 2.0, WHITE);
                    // 朝向辅助线
                    let rad_angle = angle.to_radians() - std::f64::consts::PI / 2.;
                    let dir_len = 50.0;
                    let dx = rad_angle.cos() * dir_len;
                    let dy = rad_angle.sin() * dir_len;
                    draw_line((origin_x, origin_y), (origin_x + dx, origin_y + dy), 2.0, YELLOW);
                }
            },
            2 => {
                draw_infinite_line_through(
                    (p1_x, p1_y),
                    (p2_x, p2_y),
                    thickness as f32,
                    colors[color_idx]
                );
                if show_points {
                    draw_circle((p1_x, p1_y), 8.0, BLUE);
                    draw_circle((p2_x, p2_y), 8.0, RED);
                }
            },
            3 => {
                draw_line(
                    (p1_x, p1_y),
                    (p2_x, p2_y),
                    thickness as f32,
                    colors[color_idx]
                );
                if show_points {
                    draw_circle((p1_x, p1_y), 8.0, BLUE);
                    draw_circle((p2_x, p2_y), 8.0, RED);
                }
            },
            4 => {
                draw_line_through(
                    (origin_x, origin_y),
                    angle as f32,
                    seg_length as f32,
                    thickness as f32,
                    colors[color_idx]
                );
                if show_points {
                    draw_circle((origin_x, origin_y), 8.0, RED);
                    draw_circle_lines((origin_x, origin_y), 8.0, 2.0, WHITE);

                    // 朝向辅助线
                    let rad_angle = angle.to_radians() - std::f64::consts::PI / 2.;
                    let dir_len = 50.0;
                    let dx = rad_angle.cos() * dir_len;
                    let dy = rad_angle.sin() * dir_len;
                    draw_line((origin_x, origin_y), (origin_x + dx, origin_y + dy), 2.0, YELLOW);
                }
            },
            _ => {}
        }

        // UI 信息
        let mode_name = match mode {
            1 => "1: Infinite (Anchor)",
            2 => "2: Infinite (Through)",
            3 => "3: Segment (2 Points)",
            4 => "4: Segment (Anchor+Len)",
            _ => "Unknown"
        };

        let points_info = match mode {
            1 => format!("({}, {})\nAngle: {}\n", origin_x, origin_y, angle),
            2|3 => format!("P1: ({}, {})\nP2: ({}, {})\n", p1_x, p1_y, p2_x, p2_y),
            4 => format!("({}, {})\nAngle: {}\nLen = {}", origin_x, origin_y, angle, seg_length),
            _ => "Unknown".parse().unwrap()
        };


        let info = format!(
            "{}\n{}\nThick: {:.1}\nShow Points: {}\n\nControls:\n1-4: Switch Mode\nWASD: {} \nArrows: {} \nQ/E: Rotate\nZ/X: {} \nSpace: Color\nR: Reset",
            mode_name,
            points_info,
            thickness,
            show_points,
            if mode == 1 || mode == 4 { "Move Origin" } else { "Move P1" },
            if mode == 2 || mode == 3 { "Move P2" } else { "N/A" },
            if mode == 4 { "Length" } else { "Thickness" }
        );
        draw_multiline_text(&info, (10.0, 10.0), CTR_LT, 30.0, None, WHITE);

        next_frame().await
    }
}