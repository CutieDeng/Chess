#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::VecDeque;

use chess::{setup_fonts, ChessPiece, ChessType, calculate_operators};
use eframe::epaint::{Color32, Stroke};
use eframe::{App, NativeOptions};
use eframe::egui;
use eframe::egui::RichText; 

fn main() {
    let app = MyApp::new(); 
    let native_options = NativeOptions::default(); 
    eframe::run_native("中国象棋", native_options, Box::new(|ctx | {
        setup_fonts(&ctx.egui_ctx);
        Box::new(app)
    }) ); 
}

struct MyApp {
    chess : ChessInfo, 
    animations: VecDeque<Box<dyn FontAnimation>>, 
    game: GameController, 
}

struct GameController {
    state: GameState, 
    operators : Vec<MoveOperator>, 
    /// 0: black; 1: red 
    cursors : [Cursor; 2], 
}

struct Cursor {
    position: Option<usize>, 
}

struct MoveOperator {
    from_chess: ChessPiece, 
    to_chess: ChessPiece, 
    from_index: usize, 
    to_index: usize, 
}

enum GameState {
    RedTurn(Option<Select>), 
    BlackTurn(Option<Select>), 
    Win {
        black: bool, 
    }, 
}

struct Select {
    select_id: usize, 
    move_support: Vec<usize>, 
}

impl GameController {
    pub fn reset(&mut self) {
        self.state = GameState::BlackTurn(None);
        self.operators.clear(); 
        self.cursors = [Cursor {
            position: Some(4), 
        }, Cursor {
            position: Some(85), 
        }]; 
    }
}

impl MyApp {
    pub fn new() -> Self {
        let mut s = Self {
            chess : ChessInfo {
                info : Box::new([ChessPiece::None; 9 * 10]), 
            }, 
            animations: VecDeque::default(), 
            game: GameController {
                state : GameState::BlackTurn(None), 
                operators : Vec::new(), 
                cursors: [
                Cursor {
                    position: Some(4), 
                }, 
                Cursor {
                    position: Some(85),  
                }], 
            }
        }; 
        s.chess.reset(); 
        s 
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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
        let mut click = None;  
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::global_dark_light_mode_buttons(ui);
                }); 
            }); 
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
                                    word.color(Color32::BLACK)
                                } else {
                                    word.color(Color32::from_rgb(200, 50, 30)) 
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
                            let black_cursor = self.game.cursors[0].position.as_ref().map(|&v| v == index).unwrap_or(false); 
                            let red_cursor = self.game.cursors[1].position.as_ref().map(|&v| v == index).unwrap_or(false); 
                            match (red_cursor, black_cursor) {
                                (true, true) => widget = widget.fill(Color32::from_rgb(134, 197, 202)), 
                                (false, true) => widget = widget.fill(Color32::from_rgb(67, 194, 204)), 
                                (true, false) => widget = widget.fill(Color32::from_gray(200)), 
                                (false, false) => (), 
                            }
                            // widget = widget.stroke(Stroke::new(1.0, Color32::GREEN)); 
                            match &self.game.state {
                                &GameState::RedTurn(Some(ref v)) => {
                                    if v.select_id == index {
                                        widget = widget.stroke(Stroke::new(1.3, Color32::RED)); 
                                    } else if v.move_support.contains(&index) {
                                        widget = widget.stroke(Stroke::new(1.3, Color32::GREEN)); 
                                    }
                                }, 
                                &GameState::BlackTurn(Some(ref v)) => {
                                    if v.select_id == index {
                                        widget = widget.stroke(Stroke::new(1.3, Color32::RED)); 
                                    } else if v.move_support.contains(&index) {
                                        widget = widget.stroke(Stroke::new(1.3, Color32::GREEN)); 
                                    }
                                }, 
                                _ => (), 
                            }
                            if ui.add_sized( [size, size], widget ).clicked() {
                                a.push_back(Box::new(
                                    ClickFontAnimation {
                                        position: index, 
                                        origin: Color32::WHITE, 
                                        target: Color32::from_rgb(130, 12, 144), 
                                        val: 0, 
                                    }
                                )); 
                                click = Some(index); 
                            }; 
                        }
                        ui.end_row(); 
                    }
                }); 

                ui.separator(); 

                if ui.button("重置棋局").clicked() {
                    self.chess.reset(); 
                    self.game.reset(); 
                    ctx.request_repaint(); 
                }

            } ); 
        }); 

        {
            let input = ctx.input();
            if input.key_released(egui::Key::PageUp) {
                // roll the game operator ~ 
                let p = self.game.operators.pop();  
                if let Some(m) = p {
                    self.chess.info[m.from_index] = m.from_chess; 
                    self.chess.info[m.to_index] = m.to_chess; 
                    match self.game.state {
                        GameState::RedTurn(_) => {
                            self.game.state = GameState::BlackTurn(None);
                        }
                        GameState::BlackTurn(_) => {
                            self.game.state = GameState::RedTurn(None);
                        }
                        _ => (), 
                    }
                    return ; 
                }
            }
            if input.key_released(egui::Key::Space) {
                // input enter for red 
                let index = self.game.cursors[1].position; 
                match index {
                    Some(index) => {
                        if let GameState::RedTurn(ref mut v) = self.game.state {
                            match v {
                                Some(Select {
                                    select_id,
                                    move_support,
                                }) => {
                                    if *select_id == index {
                                        *v = None; 
                                    } else if move_support.contains(&index) {

                                        // great, commit this operator ~ 
                                        self.game.operators.push(MoveOperator {
                                            from_chess: self.chess.info[*select_id], 
                                            to_chess: self.chess.info[index], 
                                            from_index: *select_id, 
                                            to_index: index, 
                                        }); 
                                        self.chess.info[index] = self.chess.info[*select_id]; 
                                        self.chess.info[*select_id] = ChessPiece::None; 

                                        self.game.state = GameState::BlackTurn(None); 
                                        
                                    } else {
                                        if let ChessPiece::Chess { chess_type: _, black } = self.chess.info[index] { 
                                            if black {
                                                *v = Some(Select {
                                                    select_id: index, 
                                                    move_support: calculate_operators(&self.chess.info, index), 
                                                }); 
                                            }
                                        }
                                    }
                                },
                                None => {
                                    if let ChessPiece::Chess { chess_type: _, black } = self.chess.info[index] { 
                                        if black {
                                            *v = Some(Select {
                                                select_id: index, 
                                                move_support: calculate_operators(&self.chess.info, index), 
                                            }); 
                                        }
                                    }
                                },
                            }
                        };
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::W) {
                // input for red 
                let ref mut c = self.game.cursors[1]; 
                match &mut c.position {
                    Some(val) => {
                        let mut row = *val / 9; 
                        let col = *val % 9; 
                        if row > 0 {
                            row -= 1;  
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::S) {
                // input for red 
                let ref mut c = self.game.cursors[1]; 
                match &mut c.position {
                    Some(val) => {
                        let mut row = *val / 9; 
                        let col = *val % 9; 
                        if row < 9 { 
                            row += 1;  
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::A) {
                // input for red 
                let ref mut c = self.game.cursors[1]; 
                match &mut c.position {
                    Some(val) => {
                        let row = *val / 9; 
                        let mut col = *val % 9; 
                        if col > 0 {
                            col -= 1; 
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::D) {
                // input for red 
                let ref mut c = self.game.cursors[1]; 
                match &mut c.position {
                    Some(val) => {
                        let row = *val / 9; 
                        let mut col = *val % 9; 
                        if col < 8 { 
                            col += 1; 
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_released(egui::Key::Enter) {
                // input enter for black (actually red )
                let index = self.game.cursors[0].position; 
                match index {
                    Some(index) => {
                        if let GameState::BlackTurn(ref mut v) = self.game.state {
                            match v {
                                Some(Select {
                                    select_id,
                                    move_support,
                                }) => {
                                    if *select_id == index {
                                        *v = None; 
                                    } else if move_support.contains(&index) {

                                        // great, commit this operator ~ 
                                        self.game.operators.push(MoveOperator {
                                            from_chess: self.chess.info[*select_id], 
                                            to_chess: self.chess.info[index], 
                                            from_index: *select_id, 
                                            to_index: index, 
                                        }); 
                                        self.chess.info[index] = self.chess.info[*select_id]; 
                                        self.chess.info[*select_id] = ChessPiece::None; 

                                        self.game.state = GameState::RedTurn(None); 
                                        
                                    } else {
                                        if let ChessPiece::Chess { chess_type: _, black } = self.chess.info[index] { 
                                            if !black {
                                                *v = Some(Select {
                                                    select_id: index, 
                                                    move_support: calculate_operators(&self.chess.info, index), 
                                                }); 
                                            }
                                        }
                                    }
                                },
                                None => {
                                    if let ChessPiece::Chess { chess_type: _, black } = self.chess.info[index] { 
                                        if !black {
                                            *v = Some(Select {
                                                select_id: index, 
                                                move_support: calculate_operators(&self.chess.info, index), 
                                            }); 
                                        }
                                    }
                                },
                            }
                        };
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                // input for black 
                let ref mut c = self.game.cursors[0]; 
                match &mut c.position {
                    Some(val) => {
                        let mut row = *val / 9; 
                        let col = *val % 9; 
                        if row > 0 {
                            row -= 1;  
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::ArrowDown) {
                // input for black 
                let ref mut c = self.game.cursors[0]; 
                match &mut c.position {
                    Some(val) => {
                        let mut row = *val / 9; 
                        let col = *val % 9; 
                        if row < 9 { 
                            row += 1;  
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::ArrowLeft) {
                // input for black 
                let ref mut c = self.game.cursors[0]; 
                match &mut c.position {
                    Some(val) => {
                        let row = *val / 9; 
                        let mut col = *val % 9; 
                        if col > 0 {
                            col -= 1; 
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
            if input.key_pressed(egui::Key::ArrowRight) {
                // input for black 
                let ref mut c = self.game.cursors[0]; 
                match &mut c.position {
                    Some(val) => {
                        let row = *val / 9; 
                        let mut col = *val % 9; 
                        if col < 8 { 
                            col += 1; 
                        }
                        *val = row * 9 + col; 
                    }
                    None => (), 
                }
            }
        }
        
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
