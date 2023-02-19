//! 棋子类型 
//! 
//! 两个核心定义都在此定义：
//! 
//! - 棋子种类类型：决定棋子自身的行进方式
//! 
//! - 棋子所属阵营
//! 

#[derive( Copy , Clone , Debug , PartialEq , Eq , Hash ) ] 
/// 棋子种类类型
pub enum ChessType {
    /// 帅
    King,
    /// 士
    Guard,
    /// 相
    Elephant,
    /// 马
    Horse,
    /// 车
    Chariot,
    /// 炮
    Cannon,
    /// 兵
    Soldier, 
}

#[derive( Copy , Clone , Debug , PartialEq , Eq , Hash ) ] 
pub enum Side {
    /// 红方 
    Red,
    /// 黑方 
    Black,
} 

#[derive( Clone , Debug , PartialEq , Eq , Hash , Default, Copy ) ] 
/// 棋子类型，包含棋子种类和棋子所属阵营：这是一个可空类型
pub struct ChessPiece ( pub Option < ( ChessType , Side ) > ) ; 

impl ChessPiece {
    /// 检查棋子是否同属某阵营
    pub fn same_side(&self, other: Side) -> bool {
        if let Some((_, s)) = self.0 {
            s == other 
        } else {
            false 
        } 
    }
}

impl <'a> Into<Option<char>> for &ChessPiece { 
    fn into(self) -> Option<char> {
        if let Some((c, s)) = self.0 { 
            match (c, s) {
                (ChessType::King, Side::Red) => Some('帅'), 
                (ChessType::King, Side::Black) => Some('将'), 
                (ChessType::Guard, Side::Red) => Some('仕'),
                (ChessType::Guard, Side::Black) => Some('士'), 
                (ChessType::Elephant, Side::Red) => Some('相'), 
                (ChessType::Elephant, Side::Black) => Some('象'), 
                (ChessType::Horse, Side::Red) => Some('马'), 
                (ChessType::Horse, Side::Black) => Some('馬'),
                (ChessType::Chariot, Side::Red) => Some('车'),
                (ChessType::Chariot, Side::Black) => Some('車'), 
                (ChessType::Cannon, Side::Red) => Some('炮'), 
                (ChessType::Cannon, Side::Black) => Some('砲'), 
                (ChessType::Soldier, Side::Red) => Some('兵'), 
                (ChessType::Soldier, Side::Black) => Some('卒'), 
            }
        } else {
            None 
        }
    }
}

impl ChessPiece {
    /// 将棋子转换为字符，如果棋子为空，则产生不可恢复的错误 
    pub fn as_char_unwrap(&self) -> char {
        Into::<Option<char>>::into(self).unwrap()
    } 
}