
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 棋子类型 
pub enum PieceType {
    /// 兵 
    Pawn , 
    /// 车
    Rook ,
    /// 马
    Knight ,
    /// 相
    Bishop ,
    /// 士
    Guard ,
    /// 帅
    King ,
    /// 炮
    Cannon ,
    /// 空，该类型用于保留实现，表示未知的棋子类型  
    Empty , 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum Camp {
    /// 红方 
    Red , 
    /// 黑方 
    Black , 
} 

#[derive( Debug , Clone , Copy , PartialEq , Eq )] 
/// 棋子存在 
pub struct PieceExist ( pub PieceType , pub Camp ); 

impl PieceExist {
    pub fn name(&self) -> char {
        match self.0 {
            PieceType::Pawn => match self.1 {
                Camp::Red => '兵',
                Camp::Black => '卒', 
            }
            PieceType::Rook => match self.1 {
                Camp::Red => '车',
                Camp::Black => '車', 
            }
            PieceType::Knight => match self.1 {
                Camp::Red => '马',
                Camp::Black => '馬', 
            }
            PieceType::Bishop => match self.1 {
                Camp::Red => '相',
                Camp::Black => '象', 
            }
            PieceType::Guard => match self.1 {
                Camp::Red => '仕',
                Camp::Black => '士', 
            }
            PieceType::King => match self.1 {
                Camp::Red => '帅',
                Camp::Black => '将', 
            }
            PieceType::Cannon => match self.1 {
                Camp::Red => '炮',
                Camp::Black => '砲', 
            }
            PieceType::Empty => ' ',
        }
    }
}

#[derive( Debug , Clone , Copy , PartialEq , Eq )] 
/// 棋子 
pub enum Piece {
    /// 棋子存在 
    Exist ( PieceExist ), 
    /// 棋子不存在 
    None ,  
}

