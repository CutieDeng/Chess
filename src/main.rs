#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::VecDeque;

use chess::setup_fonts;
use eframe::epaint::{Color32, Vec2};
use eframe::{App, NativeOptions};
use eframe::egui::{self, RichText}; 

fn main() {
    let app = MyApp::new(); 
    let native_options = NativeOptions::default(); 
    eframe::run_native("中国象棋", native_options, Box::new(|ctx | {
        setup_fonts(&ctx.egui_ctx);
        Box::new(MyApp::new())
    }) ); 
}

struct MyApp {
    chess : ChessInfo, 
    animations: VecDeque<Box<dyn FontAnimation>>, 
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            chess : ChessInfo {
                info : Box::new([ChessPiece::None; 9 * 10]), 
            }, 
            animations: VecDeque::default(), 
        }
    }
}

trait FontAnimation {
    fn change(&mut self) -> Option<(usize, Color32)>; 
}

// every color displays on 400 ms, one update happens with 17 ms. 24 clicks for a tip 
// three rounds, every thing done, the last one is 24 * 6 = 144 
struct ClickFontAnimation {
    position : usize, 
    origin : Color32, 
    target : Color32, 
    val : i32, 
}

impl FontAnimation for ClickFontAnimation {
    fn change(&mut self) -> Option<(usize, Color32)> {
        self.val += 1; 
        if self.val >= 144 {
            self.val = 144; 
            None 
        } else {
            let p = self.val / 24; 
            Some((self.position, 
                if p % 2 == 0 {
                    self.target
                } else {
                    self.origin
                })) 
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut m = Vec::new(); 
        let mut removes = Vec::new(); 
        let ref mut a = self.animations; 
        let ai = a.iter_mut().enumerate(); 
        for i in ai {
            let c = (i.1).change();
            match c {
                Some(v) => {
                    m.push(v);
                },
                None => {
                    removes.push(i.0);
                },
            }
        }
        for r in removes.into_iter().rev() {
            a.swap_remove_back(r);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui | 
            {
                egui::Grid::new("chess board").num_columns(9).show(ui, |ui| {
                    for i in 0..10 {
                        for j in 0..9 {
                            let val = self.chess.info[i * 9 + j]; 
                            let word; 
                            match val {
                                ChessPiece::None => {
                                    word = "";  
                                }
                                ChessPiece::Chess { chess_type, black } => {
                                    word = match chess_type {
                                        ChessType::BING => match black {
                                            true => "卒", 
                                            false => "兵", 
                                        }
                                        ChessType::PAO => match black {
                                            true => "砲", 
                                            false => "炮", 
                                        }
                                        ChessType::CHE => match black {
                                            true => "車",
                                            false => "车",  
                                        }
                                        ChessType::MA => match black {
                                            true => "馬", 
                                            false => "马", 
                                        }
                                        ChessType::XIANG => match black {
                                            true => "象", 
                                            false => "相", 
                                        }
                                        ChessType::SHI => match black {
                                            true => "仕", 
                                            false => "士", 
                                        }
                                        ChessType::JIANG => match black {
                                            true => "将", 
                                            false => "帅", 
                                        }
                                    }; 
                                }
                            }
                            let index = i * 9 + j; 
                            let size = 50.0; 
                            let mut word = RichText::new(word).size(size * 0.7); 
                            if let ChessPiece::Chess { chess_type : _, black } = val {
                                word = if black {
                                    word.color(Color32::from_rgb(200, 50, 30)) 
                                } else {
                                    word.color(Color32::BLACK)
                                }
                            }
                            for &ii in m.iter() {
                                if ii.0 == index {
                                    word = word.color(ii.1); 
                                    ctx.request_repaint(); 
                                }
                            }
                            // use the word , with the proper font 
                            let mut widget = egui::Button::new(word); 
                            if j >= 3 && j < 6 && ( i < 3 || i > 6 ) {
                                widget = widget.fill(Color32::from_gray(140)); 
                            }
                            if ui.add_sized( [size, size], widget ).clicked() {
                                // println!("You click {word}. "); 
                                a.push_back(Box::new(
                                    ClickFontAnimation {
                                        position: index, 
                                        origin: Color32::GREEN, 
                                        target: Color32::from_rgb(130, 12, 144), 
                                        val: 0, 
                                    }
                                )); 
                            }; 
                        }
                        ui.end_row(); 
                    }
                }); 

                ui.separator(); 

                if ui.button("重置棋局").clicked() {
                    self.chess.reset(); 
                    ctx.request_repaint(); 
                }

            } ); 
        }); 
    }
}

struct ChessInfo {
    info: Box<[ChessPiece; 9 * 10]>, 
}

impl ChessInfo {
    pub fn reset(&mut self) {
        let ref mut b = self.info; 
        b.fill(ChessPiece::None);
        use ChessType::*; 

        let base = 0; 
        let black = false; 
        b[base + 0] = ChessPiece::Chess { chess_type: CHE, black }; 
        b[base + 8] = b[base + 0]; 
        b[base + 1] = ChessPiece::Chess { chess_type: MA, black};
        b[base + 7] = b[base + 1]; 
        b[base + 2] = ChessPiece::Chess { chess_type: XIANG, black}; 
        b[base + 6] = b[base + 2]; 
        b[base + 3] = ChessPiece::Chess { chess_type: SHI, black};
        b[base + 5] = b[base + 3]; 
        b[base + 4] = ChessPiece::Chess { chess_type: JIANG, black};

        let base = 81; 
        let black = true; 
        b[base + 0] = ChessPiece::Chess { chess_type: CHE, black }; 
        b[base + 8] = b[base + 0]; 
        b[base + 1] = ChessPiece::Chess { chess_type: MA, black};
        b[base + 7] = b[base + 1]; 
        b[base + 2] = ChessPiece::Chess { chess_type: XIANG, black}; 
        b[base + 6] = b[base + 2]; 
        b[base + 3] = ChessPiece::Chess { chess_type: SHI, black};
        b[base + 5] = b[base + 3]; 
        b[base + 4] = ChessPiece::Chess { chess_type: JIANG, black};

        let base = 18; 
        let black = false; 
        b[base + 1] = ChessPiece::Chess { chess_type: PAO, black };
        b[base + 7] = b[base + 1];

        let base = 63;  
        let black = true; 
        b[base + 1] = ChessPiece::Chess { chess_type: PAO, black };
        b[base + 7] = b[base + 1];

        let base = 27; 
        let black = false; 
        b[base + 0] = ChessPiece::Chess { chess_type: BING, black };
        b[base + 2] = b[base]; 
        b[base + 4] = b[base]; 
        b[base + 6] = b[base]; 
        b[base + 8] = b[base]; 

        let base = 54; 
        let black = true; 
        b[base + 0] = ChessPiece::Chess { chess_type: BING, black };
        b[base + 2] = b[base]; 
        b[base + 4] = b[base]; 
        b[base + 6] = b[base]; 
        b[base + 8] = b[base]; 
    }
}

/// In this type enum, we just directly use pinyin to describe the type of the chess, in chinese. 
/// Because this is the chinese chess ) 
#[derive(Clone, Copy)]
enum ChessType {
    /// 兵 / 卒
    BING, 
    /// 炮 / 砲
    PAO,  
    /// 车 / 車
    CHE, 
    /// 马 / 馬
    MA, 
    /// 象 / 相
    XIANG, 
    /// 士 / 仕
    SHI, 
    /// 帅 / 将
    JIANG, 
}

#[derive(Clone, Copy)]
enum ChessPiece {
    /// There isn't any chess on this chess. 
    None, 
    /// A chess piece with two info: chess type and camp 
    Chess {
        chess_type : ChessType, 
        black: bool, 
    }
}