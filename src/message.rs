use crate::piece::{Camp, Piece};
use crate::point::{Vector, Point};

/// 逻辑层向前端层发送的消息类型 
pub enum Message {
    /// 指示前端重置棋盘为当前设置 
    ResetChess ( Box<[Piece; 90]> ), 
    /// 指示前端处理移动事件 
    DisplayMove {
        step : Vector, 
        confirm : ( Piece , Piece ) , 
    }, 
    /// 用于解释棋盘上的棋子，面对单步询问 
    Clarify ( Point , Piece ) ,  
    /// 通知允许前端进行操作，每隔一段时间就会触发一次提示，累计 60 次则棋盘进入结算状态 
    AllowOperation ( Camp ) , 
    /// 通知前端进行结算 
    /// 
    /// 结算状态有三种状态：某一方胜利（两种），和棋。 
    EndOfGame ( Option<Camp> ) , 
    /// 告知所有移动操作 
    AllMoves ( Vec< Vector > ) , 
    /// 允许前端作为某一方进行操作 
    ConfirmPlayer ( Camp , u128 ) , 
}

/// 前端向逻辑层发送的消息类型 
pub enum Question {
    /// 询问棋盘上所有棋子的状态（会得到一个 ResetChess 的响应）
    ClarifyAllRequest, 
    /// 询问棋盘上某一点的棋子状态（会得到一个 Clarify 的响应） 
    Clarify ( Point ) , 
    /// 请求执行一步移动 
    Execute {
        /// 描述移动的向量 
        move_step : Vector, 
        /// 用于指示该移动是第几回合，用于检查是否与其他移动指示冲突 
        /// 维护移动的原子性 
        check_len : usize , 
    }, 
    /// 请求前端作为某一方进行操作 
    AsPlayer ( Camp , u128 ) , 
}