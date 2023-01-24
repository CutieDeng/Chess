use std::{num::NonZeroU128, collections::HashMap, sync::Arc};

use eframe::{egui::{CentralPanel, ScrollArea, Layout, Button, WidgetText, RichText, TextFormat}, epaint::{Galley, text::{LayoutJob, Row}, Color32, Vec2, FontId, FontFamily::{Proportional, Monospace}, self}, emath::Align};

use crate::Display;

pub struct NetworkDisplay {

}

impl Display for NetworkDisplay {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame, info: &mut crate::GlobalInfo) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::centered(eframe::egui::Direction::TopDown), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let width = ui.available_width();
                    let height = 20.0; 
                    for _ in 0..200 {
                        let mut job = LayoutJob::default(); 
                        job.halign = Align::Center; 
                        job.append(
                            "这是一个普通的房间名",
                            0.0,
                            TextFormat {
                                font_id: FontId::new(14.0, Proportional),
                                color: Color32::WHITE,
                                ..Default::default()
                            },
                        );
                        job.append("", 20.0, Default::default() ); 
                        job.append("playing", 0.0, TextFormat {
                            font_id: FontId::new(14.0, Proportional),
                            color: Color32::LIGHT_RED,
                            ..Default::default()
                        }); 
                        job.append("", 20.0, Default::default() ); 
                        job.append("观战 +4", 0.0, TextFormat {
                            font_id: FontId::new(14.0, Proportional),
                            color: Color32::LIGHT_GRAY,
                            ..Default::default()
                        }); 

                        let button = Button::new(job).min_size(Vec2 { x: width, y: height }); 
                        if ui.add(button).clicked() {} ; 
                        ui.separator(); 
                    }
                    // let b = Button::
                }); 
            }); 
        }); 
    }
}

pub struct NetworkInfoCache {
    persons: HashMap<NonZeroU128, Player>, 
    rooms: Vec<Room>, 
}

impl NetworkInfoCache {
    pub fn new() -> NetworkInfoCache {
        NetworkInfoCache { persons: Default::default(), rooms: Default::default() }
    }
}

impl NetworkInfoCache {
    fn push(&mut self, room: Room) {
        self.rooms.push(room);
    }
    pub fn update_or_push(&mut self, room: Room) {
        let mut result = self.rooms.iter_mut().filter(|r| r.name == room.name); 
        let first = result.next(); 
        if let Some(r) = first {
            *r = room;  
        } else {
            self.push(room);
        }
    }
}

/// The info of a game room 
/// 
/// It used to indicate the room state (playing | waiting | talking)
/// playing: two players are playing in the room 
/// waiting: one player or no player is waiting in this room. 
/// talking: display a game state lists and you can undo it as casual. 
pub struct Room {
    name: String, 
    state: Option<&'static str>, 
    players: [Option<NonZeroU128>; 2], 
    viewers: Vec<NonZeroU128>, 
}

impl Room {
    pub fn new(name: String, state: &str, players: [Option<NonZeroU128>; 2], viewers: Vec<NonZeroU128>) -> Option<Room> { 
        let state = match state {
            "play" => Some("play"),
            "wait" => Some("wait"),
            "talk" => Some("talk"), 
            _ => None, 
        }; 
        // check the viewer isn't the player ~ 
        if let Some(f) = players[0] {
            if viewers.contains(&f) {
                return None; 
            }
        }
        if let Some(f) = players[1] {
            if viewers.contains(&f) {
                return None; 
            }
        }
        Some( Room {
            name,
            state,
            players,
            viewers,
        } ) 
    }
}

pub struct Player {
    id: NonZeroU128, 
    name: String, 
}