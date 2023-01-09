use eframe::egui;

pub fn setup_fonts(ctx : &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();  
    fonts.font_data.insert("font1".into(), egui::FontData::from_static(
        include_bytes!("/Users/cutiedeng/downloads/lxgw/LXGWWenKai-Regular.ttf")
    )); 
    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "font1".to_owned()); 
    fonts.families.entry(egui::FontFamily::Monospace).or_default().push("font1".to_owned()); 
    ctx.set_fonts(fonts); 
}

pub fn calculate_operators(chesses: &[ChessPiece; 90], index: usize ) -> Vec<usize> {
    assert!(index < 90, "Invalid index in chess : {index}"); 
    let (row, col ) = (index / 9, index % 9); 
    let mut result = Vec::new(); 
    match chesses[index] {
        ChessPiece::None => panic!("Invalid calculate operators: no piece on index {index}"), 
        ChessPiece::Chess { chess_type, black } => {
            match chess_type {
                ChessType::BING => {
                    match black {
                        false => {
                            if row < 9 {
                                let nxt = (row + 1) * 9 + col; 
                                match &chesses[nxt] {
                                    ChessPiece::Chess { chess_type: _, black: false } => (), 
                                    _ => {
                                        result.push(nxt); 
                                    }
                                }
                            }
                            if row >= 5 {
                                if col > 0 {
                                    let nxt = row * 9 + (col - 1); 
                                    match &chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: false } => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                if col < 8 {
                                    let nxt = row * 9 + (col + 1); 
                                    match &chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: false } => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                            }
                        }
                        true => {
                            if row > 0 {
                                let nxt = (row - 1) * 9 + col; 
                                match &chesses[nxt] {
                                    ChessPiece::Chess { chess_type: _, black: true } => (), 
                                    _ => {
                                        result.push(nxt); 
                                    }
                                }
                            }
                            if row <= 4 {
                                if col > 0 {
                                    let nxt = row * 9 + (col - 1); 
                                    match &chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: true } => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                if col < 8 {
                                    let nxt = row * 9 + (col + 1); 
                                    match &chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: true } => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ChessType::PAO => {
                    let mut row_increase = row + 1; 
                    while row_increase < 10 {
                        match chesses[row_increase * 9 + col] {
                            ChessPiece::None => {
                                result.push(row_increase * 9 + col);
                            }
                            ChessPiece::Chess { chess_type: _, black: _ } => {
                                break; 
                            }
                        }
                        row_increase += 1; 
                    }
                    if row_increase < 9 {
                        // go on ~ 
                        row_increase += 1; 
                        while row_increase < 10 {
                            match chesses[row_increase * 9 + col] {
                                ChessPiece::None => (), 
                                ChessPiece::Chess { chess_type: _, black: b} => {
                                    if b != black {
                                        result.push(row_increase * 9 + col); 
                                    }
                                    break; 
                                }
                            }
                            row_increase += 1; 
                        }
                    }
                    if row > 0 {
                        let mut row_decrease = row - 1; 
                        loop { 
                            match chesses[row_decrease * 9 + col] {
                                ChessPiece::None => {
                                    result.push(row_decrease * 9 + col);
                                }
                                ChessPiece::Chess { chess_type: _, black: _ } => {
                                    break; 
                                }
                            }
                            if row_decrease == 0 {
                                break; 
                            }
                            row_decrease -= 1; 
                        }
                        if row_decrease > 0 {
                            row_decrease -= 1; 
                            loop {
                                match chesses[row_decrease * 9 + col] {
                                    ChessPiece::None => (), 
                                    ChessPiece::Chess { chess_type: _, black: b} => {
                                        if b != black {
                                            result.push(row_decrease * 9 + col);
                                        }
                                        break; 
                                    }
                                }
                                if row_decrease == 0 {
                                    break; 
                                }
                                row_decrease -= 1; 
                            }
                        }
                    }
                    let mut col_increase = col + 1; 
                    while col_increase < 9 {
                        match chesses[row * 9 + col_increase] {
                            ChessPiece::None => {
                                result.push(row * 9 + col_increase); 
                            }
                            ChessPiece::Chess { chess_type: _, black: _ } => {
                                break; 
                            }
                        }
                        col_increase += 1; 
                    }
                    if col_increase < 8 {
                        col_increase += 1; 
                        while col_increase < 9 {
                            match chesses[row * 9 + col_increase] {
                                ChessPiece::None => (), 
                                ChessPiece::Chess { chess_type: _, black: b} => {
                                    if b != black {
                                        result.push(row * 9 + col_increase); 
                                    }
                                    break; 
                                }
                            }
                            col_increase += 1; 
                        }
                    }
                    let mut col_decrease = col; 
                    while col_decrease > 0 {
                        match chesses[row * 9 + col_decrease - 1] {
                            ChessPiece::None => {
                                result.push(row * 9 + col_decrease - 1);
                            }
                            ChessPiece::Chess { chess_type: _, black: _ } => {
                                break; 
                            }
                        }
                        col_decrease -= 1; 
                    }
                    if col_decrease > 1 {
                        col_decrease -= 1; 
                        while col_decrease > 0 {
                            match chesses[row * 9 + col_decrease - 1] {
                                ChessPiece::None => (), 
                                ChessPiece::Chess { chess_type: _, black: b} => {
                                    if b != black {
                                        result.push(row * 9 + col_decrease - 1); 
                                    }
                                    break; 
                                }
                            }
                            col_decrease -= 1; 
                        }
                    }
                }
                ChessType::CHE => {
                    let mut row_increase = row + 1; 
                    while row_increase < 10 {
                        match chesses[row_increase * 9 + col] {
                            ChessPiece::None => {
                                result.push(row_increase * 9 + col);
                            }
                            ChessPiece::Chess { chess_type: _, black: chess_black} => {
                                if chess_black != black {
                                    result.push(row_increase * 9 + col);
                                } 
                                break; 
                            }
                        }
                        row_increase += 1; 
                    }
                    if row > 0 {
                        let mut row_decrease = row - 1; 
                        loop { 
                            match chesses[row_decrease * 9 + col] {
                                ChessPiece::None => {
                                    result.push(row_decrease * 9 + col);
                                }
                                ChessPiece::Chess { chess_type: _, black: chess_black} => {
                                    if chess_black != black {
                                        result.push(row_decrease * 9 + col);
                                    } 
                                    break; 
                                }
                            }
                            if row_decrease == 0 {
                                break; 
                            }
                            row_decrease -= 1; 
                        }
                    }
                    let mut col_increase = col + 1; 
                    while col_increase < 9 {
                        match chesses[row * 9 + col_increase] {
                            ChessPiece::None => {
                                result.push(row * 9 + col_increase); 
                            }
                            ChessPiece::Chess { chess_type: _, black: chess_black} => {
                                if chess_black != black {
                                    result.push(row * 9 + col_increase);
                                }
                                break; 
                            }
                        }
                        col_increase += 1; 
                    }
                    let mut col_decrease = col; 
                    while col_decrease > 0 {
                        match chesses[row * 9 + col_decrease - 1] {
                            ChessPiece::None => {
                                result.push(row * 9 + col_decrease - 1);
                            }
                            ChessPiece::Chess { chess_type: _, black: chess_black} => {
                                if black != chess_black {
                                    result.push(row * 9 + col_decrease - 1); 
                                }
                                break; 
                            }
                        }
                        col_decrease -= 1; 
                    }
                }
                ChessType::MA => {
                    if row < 8 {
                        if let ChessPiece::None = chesses[(row + 1 ) * 9 + col] {
                            if col > 0 {
                                let chess_camp = black; 
                                match chesses[(row + 2) * 9 + (col - 1)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row + 2 ) * 9 + (col - 1)); 
                                    }
                                }
                            }
                            if col < 8 {
                                let chess_camp = black; 
                                match chesses[(row + 2) * 9 + (col + 1)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row + 2 ) * 9 + (col + 1)); 
                                    }
                                }
                            }
                        }
                    }
                    if row > 1 {
                        if let ChessPiece::None = chesses[(row - 1 ) * 9 + col] {
                            if col > 0 {
                                let chess_camp = black; 
                                match chesses[(row - 2) * 9 + (col - 1)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row - 2 ) * 9 + (col - 1)); 
                                    }
                                }
                            }
                            if col < 8 {
                                let chess_camp = black; 
                                match chesses[(row - 2) * 9 + (col + 1)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row - 2 ) * 9 + (col + 1)); 
                                    }
                                }
                            }
                        }
                    }
                    if col > 1 {
                        if let ChessPiece::None = chesses[row * 9 + (col - 1)] {
                            if row > 0 {
                                let chess_camp = black; 
                                match chesses[(row - 1) * 9 + (col - 2)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row - 1) * 9 + (col - 2)); 
                                    }
                                }
                            }
                            if row < 9 { 
                                let chess_camp = black; 
                                match chesses[(row + 1) * 9 + (col - 2)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row + 1) * 9 + (col - 2)); 
                                    }
                                }
                            }
                        }
                    }
                    if col < 7 {
                        if let ChessPiece::None = chesses[row * 9 + (col + 1)] {
                            if row > 0 {
                                let chess_camp = black; 
                                match chesses[(row - 1) * 9 + (col + 2)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row - 1) * 9 + (col + 2)); 
                                    }
                                }
                            }
                            if row < 9 { 
                                let chess_camp = black; 
                                match chesses[(row + 1) * 9 + (col + 2)] {
                                    ChessPiece::Chess { chess_type: _, black} if black == chess_camp => (), 
                                    _ => {
                                        result.push((row + 1) * 9 + (col + 2)); 
                                    }
                                }
                            }
                        }
                    }
                }, 
                ChessType::XIANG => {
                    let mut tmp = Vec::new(); 
                    if col < 7 {
                        match black {
                            false => {
                                if row < 3 {
                                    if let ChessPiece::None = chesses[(row + 1) * 9 + (col + 1 )] {
                                        tmp.push((row + 2) * 9 + (col + 2));  
                                    }
                                }
                                if row > 1 {
                                    if let ChessPiece::None = chesses[(row - 1) * 9 + (col + 1 )] {
                                        tmp.push((row - 2) * 9 + (col + 2));  
                                    }
                                }
                            }
                            true => {
                                if row < 8 {
                                    if let ChessPiece::None = chesses[(row + 1) * 9 + (col + 1 )] {
                                        tmp.push((row + 2) * 9 + (col + 2));  
                                    }
                                }
                                if row > 6 {
                                    if let ChessPiece::None = chesses[(row - 1) * 9 + (col + 1)] {
                                        tmp.push((row - 2) * 9 + (col + 2)); 
                                    }
                                }
                            }
                        }
                    }
                    if col > 1 {
                        match black {
                            false => {
                                if row < 3 {
                                    if let ChessPiece::None = chesses[(row + 1) * 9 + (col - 1 )] {
                                        tmp.push((row + 2) * 9 + (col - 2));  
                                    }
                                }
                                if row > 1 {
                                    if let ChessPiece::None = chesses[(row - 1) * 9 + (col - 1 )] {
                                        tmp.push((row - 2) * 9 + (col - 2));  
                                    }
                                }
                            }
                            true => {
                                if row < 8 {
                                    if let ChessPiece::None = chesses[(row + 1) * 9 + (col - 1 )] {
                                        tmp.push((row + 2) * 9 + (col - 2));  
                                    }
                                }
                                if row > 6 {
                                    if let ChessPiece::None = chesses[(row - 1) * 9 + (col - 1)] {
                                        tmp.push((row - 2) * 9 + (col - 2)); 
                                    }
                                }
                            }
                        }
                    }
                    for i in tmp {
                        match chesses[i] {
                            ChessPiece::Chess { chess_type: _, black: c} if c == black => (),
                            _ => {
                                result.push(i); 
                            }
                        }
                    }
                }
                ChessType::SHI => {
                    match black {
                        false => {
                            // red 
                            match (row, col) {
                                (0, 3) => {
                                    let nxt = 1 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (0, 5) => {
                                    let nxt = 1 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (1, 4) => {
                                    let nxts = [3, 5, 21, 23]; 
                                    for nxt in nxts {
                                        match chesses[nxt] {
                                            ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                            _ => {
                                                result.push(nxt); 
                                            }
                                        }
                                    }
                                }
                                (2, 3) => {
                                    let nxt = 1 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (2, 5) => {
                                    let nxt = 1 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                _ => panic!("Invalid position of SHI: {index}"), 
                            }
                        }
                        true => {
                            // black 
                            match (row, col) {
                                (9, 3) => {
                                    let nxt = 8 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (9, 5) => {
                                    let nxt = 8 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (8, 4) => {
                                    let nxts = [84, 86, 66, 68]; 
                                    for nxt in nxts {
                                        match chesses[nxt] {
                                            ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                            _ => {
                                                result.push(nxt); 
                                            }
                                        }
                                    }
                                }
                                (7, 3) => {
                                    let nxt = 8 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                (7, 5) => {
                                    let nxt = 8 * 9 + 4; 
                                    match chesses[nxt] {
                                        ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                        _ => {
                                            result.push(nxt); 
                                        }
                                    }
                                }
                                _ => panic!("Invalid position of SHI"), 
                            }
                        }
                    }
                }
                ChessType::JIANG => {
                    if col > 3 {
                        match chesses[row * 9 + col - 1] {
                            ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                            _ => {
                                result.push(row * 9 + col - 1);
                            }
                        }
                    }
                    if col < 5 {
                        match chesses[row * 9 + col + 1] {
                            ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                            _ => {
                                result.push(row * 9 + col + 1);
                            }
                        }
                    }
                    match black {
                        false => {
                            if row > 0 {
                                match chesses[(row - 1) * 9 + col] {
                                    ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                    _ => {
                                        result.push((row - 1) * 9 + col);
                                    }
                                }
                            }
                            if row < 2 { 
                                match chesses[(row + 1) * 9 + col] {
                                    ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                    _ => {
                                        result.push((row + 1) * 9 + col);
                                    }
                                }
                            }
                        }
                        true => {
                            if row > 7 {
                                match chesses[(row - 1) * 9 + col] {
                                    ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                    _ => {
                                        result.push((row - 1) * 9 + col);
                                    }
                                }
                            }
                            if row < 9 { 
                                match chesses[(row + 1) * 9 + col] {
                                    ChessPiece::Chess { chess_type: _, black: chess_camp} if chess_camp == black => (), 
                                    _ => {
                                        result.push((row + 1) * 9 + col);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    result 
}

#[derive(Clone, Copy)]
pub enum ChessPiece {
    /// There isn't any chess on this chess. 
    None, 
    /// A chess piece with two info: chess type and camp 
    Chess {
        chess_type : ChessType, 
        black: bool, 
    }
}

/// In this type enum, we just directly use pinyin to describe the type of the chess, in chinese. 
/// Because this is the chinese chess ) 
#[derive(Clone, Copy)]
pub enum ChessType {
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