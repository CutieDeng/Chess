use eframe::{App, egui::{CentralPanel, Context, Ui, Layout}, Frame};

use crate::{AppScreen, Display, GlobalInfo};

/// Home screen display state
/// 
/// It only contains a pointer, to describe the next instant, the page would changes or not. 
pub struct HomeState(pub(super) Option<Box<AppScreen>>); 

impl Display for HomeState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut Frame, info: &mut GlobalInfo) {
        CentralPanel::default().show(ctx, |ui| {
            let home_layout = Layout::top_down(eframe::emath::Align::Center); 
            ui.with_layout(home_layout, |ui| {

                // single play, with extensional artificial intelligence 
                if ui.button("单人游戏").clicked() {
                    
                }

                // two players in host, I'd like to give the special support for keyboard 
                if ui.button("双人游戏").clicked() {

                }

                // play online 
                if ui.button("联网游戏").clicked() {
                    self.0 = Some(Box::new(AppScreen::NetworkDisplay(super::network::NetworkDisplay {  })));
                }

                // global configures for displaying 
                if ui.button("显示设置").clicked() {

                }

                // configures for keyboard, and other things maybe 
                if ui.button("游戏设置").clicked() {

                }

            } ); 
        }); 
    }
}