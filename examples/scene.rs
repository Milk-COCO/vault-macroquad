use std::any::Any;
use macroquad::prelude::*;
use macroquad::{assert_no_input, impl_scene, scene_flow};

#[macroquad::main("scene-example")]
async fn main() {
    init_box();
    scene_flow!(MainScene);
}

pub struct MainScene;

impl_scene! { for MainScene {
    async fn ready(&mut self, input: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> {
        assert_no_input!(input);
        
        Ok(SceneAction::Overlay(Scene1.boxed(), None))
    }
    
    async fn process(&mut self) -> anyhow::Result<SceneAction> {
        clear_background(Color::from_hex(0x1e1f22));
        
        Ok(SceneAction::None)
    }
}}

pub struct Scene1;

impl_scene!{for Scene1 {
    async fn ready(&mut self, input: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> {
        assert_no_input!(input);
        
        ui_box().insert("scene2",Button::default().with_text("scene2".to_string()));
        Ok(SceneAction::None)
    }
    
    async fn process(&mut self) -> anyhow::Result<SceneAction> {
        Ok(SceneAction::None)
    }
    
    async fn ui(&mut self) -> anyhow::Result<SceneAction> {
        let chart_list_button = ui_box().get_mut::<Button>("scene2").unwrap();
        let chart_list_button_pos = (0.,0.);
        if chart_list_button.process(chart_list_button_pos).is_clicked() {
            return Ok(SceneAction::Push(Scene2::new(),None));
        }
        chart_list_button.draw(chart_list_button_pos);
        Ok(SceneAction::None)
    }
    
    async fn draw(&mut self) -> anyhow::Result<SceneAction> {
        draw_text("ccb",(200.,200.),CTR_LT,30.,WHITE);
        Ok(SceneAction::None)
    }
    
    async fn on_death(&mut self) -> anyhow::Result<SceneAction> {
        ui_box().remove("scene2");
        Ok(SceneAction::None)
    }
    
    async fn on_result(&mut self, child: Box<dyn Scene>, _result: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> {
        info!("back to scene1");
        if child.as_any().is::<Scene2>() {
            info!("from scene2");
        }
        Ok(SceneAction::None)
    }
}}

pub struct Scene2 {
    last: f64,
}

impl Scene2 {
    fn new() -> Box<dyn Scene> {
        Self{last: 0.0}.boxed()
    }
}

impl_scene! { for Scene2 {
    async fn ready(&mut self, input: Option<Box<dyn Any>>) -> anyhow::Result<SceneAction> {
        // 也可以不加这玩意
        assert_no_input!(input);
        ui_box().insert("Exit",Button::default().with_text("Exit".to_string()));
        
        Ok(SceneAction::None)
    }
    
    async fn process(&mut self) -> anyhow::Result<SceneAction> {
        if get_time() - self.last > 5. {self.last = get_time() ; info!("im~in~scene2~");}
        Ok(SceneAction::None)
    }
    
    async fn ui(&mut self) -> anyhow::Result<SceneAction> {
        let exit = ui_box().get_mut::<Button>("Exit").unwrap();
        
        let size = get_time().sin() as f32 * 30. + 40.;
        exit.size((size,size));
        
        exit.process((200.,200.));
        if exit.is_clicked(){
            return Ok(SceneAction::Pop(None));
        }
        
        exit.draw((200.,200.));
    
        Ok(SceneAction::None)
    }
    
    async fn on_death(&mut self) -> anyhow::Result<SceneAction> {
        ui_box().remove("Exit");
        Ok(SceneAction::None)
    }
}}
