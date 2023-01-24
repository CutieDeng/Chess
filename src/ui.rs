use eframe::{epaint::Color32, App, egui::CentralPanel};

use crate::MyApp;

use self::room::Room;

pub struct AnimationResponse {
    pub repaint: bool, 
}

impl AnimationResponse {
    pub fn new() -> Self {
        Self {
            repaint: false, 
        }
    }
}

pub struct ColorDisplay {
    pub foreground: Color32, 
    pub background: Color32, 
}

pub struct CursorContext {
    pub black: bool, 
    pub white: bool, 
}

pub trait ColorGetter {
    fn get(&mut self, context: &CursorContext) -> (ColorDisplay, AnimationResponse);
}

pub struct GameApp {
    user: User,
    frame: Frame,
}

impl GameApp {
    pub fn new() -> Self {
        GameApp {
            user: User::create_with_null(), 
            frame: Frame::init(), 
        }
    }
}

pub struct User {
    private: Option<PrivateUser>,
    achievement: Achievement, 
}

impl User {
    pub fn name(&self) -> &str {
        match self.private {
            Some(ref p ) => {
                p.name.as_str()
            },
            None => {
                "匿名用户"
            },
        }
    } 
}

impl User {
    pub fn create_with_null() -> Self {
        User {
            private: None, 
            achievement: Default::default(), 
        }
    }
}

pub struct PrivateUser {
    name: String, 
}

pub struct Achievement {
    winning_games: usize, 
    losing_games: usize, 
    total_games: usize, 
}

impl Default for Achievement {
    fn default() -> Self {
        Self { winning_games: Default::default(), losing_games: Default::default(), total_games: Default::default() }
    }
}

impl Achievement {
    pub fn winning_rates(&self) -> f32 {
        if self.total_games == 0 {
            return 0.0 
        }
        let wins = self.winning_games as f32; 
        let totals = self.total_games as f32; 
        wins / totals
    }
    pub fn total_games(&self) -> usize {
        self.total_games
    }
    pub fn winning_games(&self) -> usize {
        self.winning_games
    }
}

impl Achievement {
    pub fn win(&mut self) {
        self.total_games += 1; 
        self.winning_games += 1; 
    }
    pub fn loss(&mut self) {
        self.total_games += 1; 
        self.losing_games += 1; 
    }
    pub fn tie(&mut self) {
        self.total_games += 1; 
    }
}

pub enum Frame {
    MainPage, 
    NormalGame {
        // 若干属性
        app: MyApp, 
    }, 
    RemoteGame(RemoteFrame),  
}

mod room {
    pub struct Room {
        
    }
}

pub enum RemoteFrame {
    Search {
        list: Vec<Room>, 
    },
    Host {

    },
    Game {

    },
}

impl Frame {
    pub fn init() -> Self {
        Frame::MainPage 
    }
}

impl App for GameApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        match self.frame {
            Frame::MainPage => {
                // draw the main title on the center ~ 
                CentralPanel::default().show(ctx, |ui| {
                    let page_ret = main_page::show(self, ui);
                    match page_ret {
                        main_page::Feedback::None => (), 
                        main_page::Feedback::Exit => {
                            frame.close(); 
                        }
                        // todo 
                        main_page::Feedback::HostGame => (), 
                        main_page::Feedback::ClientGame => (), 
                    }
                }); 
            },
            Frame::NormalGame { ref mut app } => {
                app.update(ctx, frame); 
            }
            Frame::RemoteGame(ref mut rg) => {
                match rg {
                    RemoteFrame::Search { list }=> {
                        CentralPanel::default().show(ctx, |ui| {
                            
                        }); 
                    }
                    RemoteFrame::Host {  } => todo!(),
                    RemoteFrame::Game {  } => todo!(),
                }
            }
        }
    }
}

pub mod main_page {

    use eframe::{egui::{Ui, RichText, Button}, epaint::vec2};

    use crate::MyApp;

    use super::{GameApp, Frame};

    type MyGame = GameApp; 

    pub enum Feedback {
        None, 
        Exit, 
        HostGame, 
        ClientGame,
    }

    pub fn show(game: &mut MyGame, ui: &mut Ui) -> Feedback {
        let mut result = Feedback::None; 
        ui.vertical_centered(|ui| {
            ui.add_space(35.0); 
            let title = RichText::new("中国象棋")
                .size(60.0)
                .heading(); 
            ui.label(title); 
            let text = RichText::new("双人热座").size(40.0);
            let button = Button::new(text).min_size(vec2(210.0, 55.0)); 
            ui.add_space(40.0);
            if ui.add(button).clicked() {
                // balabalabala ~  
                game.frame = Frame::NormalGame { app: MyApp::new() }; 
                return ; 
            }
            let text = RichText::new("主机").size(40.0);
            let button = Button::new(text).min_size(vec2(210.0, 55.0)); 
            ui.add_space(40.0);
            if ui.add(button).clicked() {
                result = Feedback::HostGame; 
                return ; 
            }
            let text = RichText::new("客机").size(40.0);
            let button = Button::new(text).min_size(vec2(210.0, 55.0)); 
            ui.add_space(40.0);
            if ui.add(button).clicked() {
                result = Feedback::ClientGame; 
                return ; 
            }
            let text = RichText::new("退出游戏").size(40.0);
            let button = Button::new(text).min_size(vec2(210.0, 55.0)); 
            ui.add_space(40.0);
            if ui.add(button).clicked() {
                result = Feedback::Exit; 
                return ; 
            }
        }); 
        result 
    }

}