use crate::{front_end, piece::Camp};

type Board = front_end::Board; 

pub struct BoardTracker {
    /// 棋盘内容
    pub board : Board , 
    /// 当前走棋方 / 胜利方 
    pub camp : Camp, 
    /// 棋盘是否胜利
    pub win : bool , 
}