pub struct GameApp {
    screen: AppScreen, 
    info: GlobalInfo, 
}

impl GameApp {
    pub fn new() -> GameApp {
        GameApp { screen: AppScreen::Home(home::HomeState(None)), info: GlobalInfo {
            network: network::NetworkInfoCache::new(), 
        } }
    }
}

pub struct GlobalInfo {
    network: NetworkInfoCache, 
}

pub enum AppScreen {
    Home(home::HomeState), 
    NetworkDisplay(NetworkDisplay), 
}

impl App for GameApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut Frame) {
        let GameApp { ref mut screen, ref mut info, } = *self; 
        match screen {
            AppScreen::Home(h) => {
                h.update(ctx, frame, info);
                let t = h.0.take(); 
                if let Some(a) = t {
                    *screen = *a;
                }
                return ; 
            }
            AppScreen::NetworkDisplay(n) => {
                n.update(ctx, frame, info);
            }
        }
    }
}

pub trait Display {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut Frame, info: &mut GlobalInfo); 
}

use network::{NetworkInfoCache, NetworkDisplay};

use super::*; 

mod home; 
mod network;