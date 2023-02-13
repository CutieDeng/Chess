use crate::{piece::Piece, point::Point};

pub enum Message {
    ResetChess ( Box<[Piece; 90]> ), 
    DisplayMove {
        step : ( Point , Point ) , 
        confirm : ( Piece , Piece ) , 
    }
}