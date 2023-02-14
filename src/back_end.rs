use std::sync::mpsc::Sender;
use std::time::SystemTime;

use crate::front_end::Board;
use crate::message::{Question, Message};
use crate::piece::{Piece, Camp, PieceExist, PieceType};
use crate::point::{Point, Vector};

pub struct BoardTracker {
    /// 棋盘内容
    pub board : Board , 
    /// 当前走棋方 / 胜利方 
    pub state : State , 
    /// 当前所有移动顺序集 
    pub moves : Vec< Vector > , 
    /// 结束游戏的时间点 
    pub end_time : Option< SystemTime > , 
}

impl BoardTracker {

}

pub enum State {
    Win ( Option< Camp > ) , 
    Go ( Camp ) , 
}

impl BoardTracker {
    /// 创建一个新的棋盘跟踪器
    pub fn new () -> BoardTracker { 
        let mut board : Board = Board ( Box::new( [Piece::None; 90] ) ); 
        // 棋盘初始化 
        board.0[ Point::with_row_column(0, 0).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Rook , Camp::Black ) ); 
        board.0[ Point::with_row_column(0, 1).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Knight , Camp::Black ) );
        board.0[ Point::with_row_column(0, 2).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Bishop , Camp::Black ) );
        board.0[ Point::with_row_column(0, 3).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Guard , Camp::Black ) );
        board.0[ Point::with_row_column(0, 4).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::King , Camp::Black ) );
        board.0[ Point::with_row_column(0, 5).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Guard , Camp::Black ) );
        board.0[ Point::with_row_column(0, 6).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Bishop , Camp::Black ) );
        board.0[ Point::with_row_column(0, 7).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Knight , Camp::Black ) );
        board.0[ Point::with_row_column(0, 8).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Rook , Camp::Black ) );
        board.0[ Point::with_row_column(2, 1).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Cannon , Camp::Black ) );
        board.0[ Point::with_row_column(2, 7).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Cannon , Camp::Black ) ); 
        board.0[ Point::with_row_column(3, 0).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Black ) );
        board.0[ Point::with_row_column(3, 2).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Black ) );
        board.0[ Point::with_row_column(3, 4).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Black ) );
        board.0[ Point::with_row_column(3, 6).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Black ) );
        board.0[ Point::with_row_column(3, 8).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Black ) );
        board.0[ Point::with_row_column(6, 0).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Red ) );
        board.0[ Point::with_row_column(6, 2).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Red ) );
        board.0[ Point::with_row_column(6, 4).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Red ) );
        board.0[ Point::with_row_column(6, 6).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Red ) );
        board.0[ Point::with_row_column(6, 8).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Pawn , Camp::Red ) );
        board.0[ Point::with_row_column(7, 1).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Cannon , Camp::Red ) );
        board.0[ Point::with_row_column(7, 7).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Cannon , Camp::Red ) );
        board.0[ Point::with_row_column(9, 0).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Rook , Camp::Red ) );
        board.0[ Point::with_row_column(9, 1).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Knight , Camp::Red ) );
        board.0[ Point::with_row_column(9, 2).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Bishop , Camp::Red ) );
        board.0[ Point::with_row_column(9, 3).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Guard , Camp::Red ) );
        board.0[ Point::with_row_column(9, 4).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::King , Camp::Red ) );
        board.0[ Point::with_row_column(9, 5).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Guard , Camp::Red ) );
        board.0[ Point::with_row_column(9, 6).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Bishop , Camp::Red ) );
        board.0[ Point::with_row_column(9, 7).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Knight , Camp::Red ) );
        board.0[ Point::with_row_column(9, 8).unwrap().raw() ] = Piece::Exist( PieceExist ( PieceType::Rook , Camp::Red ) );
        Self {
            board,  
            state : State::Go(Camp::Red), 
            moves: Vec::new() , 
            end_time : None, 
        }
    }
} 

impl BoardTracker {
    pub fn handle_message ( & mut self , message : Question , sender : & mut Sender < Message > ) {
        match message {
            Question::ClarifyAllRequest => {
                let _ = sender.send( 
                    Message::ResetChess( self.board.0.clone() )
                ); 
            }
            Question::Clarify(a) => {
                let _ = sender.send(
                    Message::Clarify(a , self.board.0[a.raw()]) 
                ); 
            }
            Question::Execute { move_step, check_len } => {

            }
            Question::AsPlayer(_, _) => todo!(),
        }
    } 
}