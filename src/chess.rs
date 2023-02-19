//! 棋盘类型描述
//! 
//! ### 棋盘类型 
//! 
//! * `Board` 棋盘类型 

use crate::{piece::{ChessPiece, ChessType, Side}, point::Point};

/// 棋盘类型 
/// 
/// 一个 9 * 10 的二维数组，每个元素是一个棋子类型 
/// 
pub struct ChessBoard ( pub [ [ ChessPiece ; 10 ] ; 9 ] ) ; 

impl ChessBoard {
    pub fn get( &self, p : Point ) -> ChessPiece {
        self.0[p.x()][p.y()] 
    } 
    pub fn set ( &mut self, p : Point , piece : ChessPiece ) {
        self.0[p.x()][p.y()] = piece; 
    } 
}

impl ChessBoard {
    /// 创建一个新的棋盘 
    /// 
    /// 该方法会初始化一个新的棋盘，包含所有棋子 
    pub fn new() -> Self {
        let mut board = [ [ ChessPiece ( None ) ; 10 ] ; 9 ] ; 
        // 初始化红方 
        board [ 0 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Rook , Side :: Red ) ) ) ; 
        board [ 1 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Horse , Side :: Red ) ) ) ; 
        board [ 2 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Elephant , Side :: Red ) ) ) ; 
        board [ 3 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Guard , Side :: Red ) ) ) ;
        board [ 4 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: King , Side :: Red ) ) ) ;
        board [ 5 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Guard , Side :: Red ) ) ) ;
        board [ 6 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Elephant , Side :: Red ) ) ) ;
        board [ 7 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Horse , Side :: Red ) ) ) ;
        board [ 8 ] [ 0 ] = ChessPiece ( Some ( ( ChessType :: Rook , Side :: Red ) ) ) ;
        board [ 1 ] [ 2 ] = ChessPiece ( Some ( ( ChessType :: Cannon , Side :: Red ) ) ) ;
        board [ 7 ] [ 2 ] = ChessPiece ( Some ( ( ChessType :: Cannon , Side :: Red ) ) ) ;
        board [ 0 ] [ 3 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Red ) ) ) ;
        board [ 2 ] [ 3 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Red ) ) ) ;
        board [ 4 ] [ 3 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Red ) ) ) ;
        board [ 6 ] [ 3 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Red ) ) ) ;
        board [ 8 ] [ 3 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Red ) ) ) ;
        // 初始化黑方
        board [ 0 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Rook , Side :: Black ) ) ) ;
        board [ 1 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Horse , Side :: Black ) ) ) ;
        board [ 2 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Elephant , Side :: Black ) ) ) ;
        board [ 3 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Guard , Side :: Black ) ) ) ;
        board [ 4 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: King , Side :: Black ) ) ) ;
        board [ 5 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Guard , Side :: Black ) ) ) ;
        board [ 6 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Elephant , Side :: Black ) ) ) ;
        board [ 7 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Horse , Side :: Black ) ) ) ;
        board [ 8 ] [ 9 ] = ChessPiece ( Some ( ( ChessType :: Rook , Side :: Black ) ) ) ;
        board [ 1 ] [ 7 ] = ChessPiece ( Some ( ( ChessType :: Cannon , Side :: Black ) ) ) ;
        board [ 7 ] [ 7 ] = ChessPiece ( Some ( ( ChessType :: Cannon , Side :: Black ) ) ) ;
        board [ 0 ] [ 6 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Black ) ) ) ;
        board [ 2 ] [ 6 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Black ) ) ) ;
        board [ 4 ] [ 6 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Black ) ) ) ;
        board [ 6 ] [ 6 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Black ) ) ) ;
        board [ 8 ] [ 6 ] = ChessPiece ( Some ( ( ChessType :: Pawn , Side :: Black ) ) ) ;
        ChessBoard(board)
    } 
}

#[test]
fn test_basic_new() {
    let board = ChessBoard::new(); 
    let p_0_0 = board.0[0][0] .0.unwrap(); 
    assert! ( p_0_0 .0 == ChessType :: Rook && p_0_0 .1 == Side :: Red ); 
}