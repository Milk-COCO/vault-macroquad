use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Container Example".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let poppins = load_ttf_font("examples/poppins.ttf").await.unwrap().shared();
    let label = Label::new("Inside a Container!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()), 32.0);
    let button = Button::new(400.0, 80.0, "eeeeeeee".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.75, 0.5, 1.0), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.75, 0.5, 1.0),Some(poppins.clone()));

    let sublabel = Label::new("Inside a SUB Container!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(poppins.clone()), 32.0);
    let subbutton = Toggle::new(300.0, 60.0, "hhhhhhhh".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(1.0, 1.0, 0.5, 1.0), Some(poppins.clone()));

    let mut subcontainer = Container::new(
        Direction::Vertical,
        Align::Center,
        20.0, Color::new(0.05, 0.05, 0.1, 1.0),
        Some((20.0, 20.0, 20.0, 20.0)), Some((5.0, Color::new(1.0, 0.5, 0.5, 1.0)))
    );
    subcontainer.add_child(sublabel);
    subcontainer.add_child(subbutton);

    let mut container = Container::new(
        Direction::Horizontal,
        Align::Center,
        20.0, Color::new(0.05, 0.05, 0.1, 1.0),
        Some((20.0, 20.0, 20.0, 20.0)), Some((5.0, Color::new(1.0, 0.5, 0.5, 1.0)))
    );

    container.add_child(label);
    container.add_child(button);
    container.add_child(subcontainer); // This is completely legal, as Container implements Widget trait!

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        container.process((screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0));
        container.draw((screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0));

        next_frame().await;
    }
}
