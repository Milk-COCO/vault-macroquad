use macroquad::prelude::*;

#[macroquad::main("Text Inputs!")]
async fn main() {
    let simhei = load_ttf_font("examples/simhei.ttf").await.unwrap().shared();
    let dark = Color::new(0.05, 0.05, 0.1, 1.0);
    let purple = Color::new(0.5, 0.5, 1.0, 1.0);
    let label = Label::new("Text Inputs!".to_string(), Color::new(0.05, 0.05, 0.1, 1.0), Color::new(0.5, 0.5, 1.0, 1.0), Some(simhei.clone()), 48.0);
    let textinp = TextInput::new(Some(29),512.0, 64.0, dark, purple, dark, purple, Some(simhei.clone()));
    let textinp2 = TextInput::new(Some(29), 512.0, 64.0, dark, purple, dark, purple, Some(simhei.clone()));
    let mut container = Container::new(Direction::Vertical, Align::Center, 20.0, dark, None, None);
    container.add_child(label);
    container.add_child(textinp);
    container.add_child(textinp2);

    let mut previous_text = String::new();
    
    let mut previous_text2 = String::new();

    loop {
        clear_background(dark);
        
        container.process((screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0));
        container.draw((screen_width() / 2.0 - container.width() / 2.0, screen_height() / 2.0 - container.height() / 2.0));
        // println!("Text: {}", container.get_child_as::<TextInput>(1).unwrap().get_text());
        // println!("Text2: {}", container.get_child_as::<TextInput>(2).unwrap().get_text());
        let textinp = container.get_child_as::<TextInput>(1).unwrap();
        let textinp2 = container.get_child_as::<TextInput>(2).unwrap();
        if textinp.get_text() != previous_text {
            println!("Text: {}", textinp.get_text());
            previous_text = textinp.get_text();
        }
        if textinp2.get_text() != previous_text2 {
            println!("Text2: {}", textinp2.get_text());
            previous_text2 = textinp2.get_text();
        }
        
        textinp.draw_context_menu();
        textinp2.draw_context_menu();

        next_frame().await;
    }
}
