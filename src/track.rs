//! 棋盘操作记录器、回放器、追踪器
//! 
//! 用于记录棋盘操作、回放棋盘操作、追踪棋盘操作 
//! 

use crate::{chess::ChessBoard, step::Step, piece::{ChessPiece, Side}, point::Point};

pub struct BoardTrack {
    inner : ChessBoard, 
    steps: Vec<Step>, 
}

pub struct StepTransaction <'a> ( 
    &'a mut BoardTrack, 
    Step 
); 

impl <'a> StepTransaction <'a> {
    /// 执行一步棋子的移动操作
    pub fn commit (self) {
        let start_step = self.1.from; 
        let end_step = self.1.to;
        self.0.steps.push(self.1);  
        self.0.inner.set(end_step, self.0.inner.get(start_step)); 
        self.0.inner.set(start_step, ChessPiece(None)); 
        // self.0.inner.0[end_step.x()][end_step.y()] = self.0.inner.0[start_step.x()][start_step.y()]; 
        // self.0.inner.0[start_step.x()][start_step.y()] = ChessPiece(None); 
    }
}

impl BoardTrack {
    /// 无参初始化 
    pub fn new() -> Self {
        Self {
            inner: ChessBoard::new(), 
            steps: Vec::new(), 
        }
    } 
}

impl BoardTrack {
    pub fn step( &mut self, from : Point , to : Point ) -> StepTransaction {
        let eaten = self.inner.0[to.x()][to.y()]; 
        StepTransaction ( self , Step { from , to , eaten } ) 
    } 
}

impl BoardTrack {
    pub fn get_steps ( &self, from : Point , result : &mut Vec< Point > ) -> usize { 
        // 可行步数计数 
        let mut count = 0; 
        // 获取棋子 
        let piece = self.inner.0[from.x()][from.y()]; 
        // 检查其上面是否有棋子 
        let Some( (piece_type , side ) ) = piece.0 else {
            return 0; 
        }; 

        // 获取棋子类型 
        todo!()
    } 
}

/// 假设当前位置是马，获取它的可行位置 
fn horse_steps ( from : Point , camp : Side, board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 检查马的左边是否有棋子 
    let left = from.left(); 
    if let Some ( left ) = left {
        let left_chess = board.get(left).0;
        if let None = left_chess {
            // 检查马的左边的两个落点是否可以跳～ 
            if let Some ( ll ) = left.left() {
                if let Some ( l1 ) = ll.up() {
                    let c1 = board.get(l1);
                    if ! c1.same_side(camp) {
                        result.push(l1); 
                        count += 1;
                    }
                }
                if let Some ( l2 ) = ll.down() {
                    let c2 = board.get(l2);
                    if ! c2.same_side(camp) {
                        result.push(l2); 
                        count += 1;
                    } 
                } 
            }
        }
    }
    // 检查马的右边是否有棋子 
    let right = from.right(); 
    if let Some ( right ) = right {
        let right_chess = board.get(right).0;
        if let None = right_chess {
            // 检查马的右边的两个落点是否可以跳～ 
            if let Some ( rr ) = right.right() {
                if let Some ( r1 ) = rr.up() {
                    let c1 = board.get(r1);
                    if ! c1.same_side(camp) {
                        result.push(r1); 
                        count += 1;
                    }
                }
                if let Some ( r2 ) = rr.down() {
                    let c2 = board.get(r2);
                    if ! c2.same_side(camp) {
                        result.push(r2); 
                        count += 1;
                    } 
                } 
            }
        }
    } 
    // 检查马的上面是否有棋子 
    let up = from.up();
    if let Some ( up ) = up {
        let up_chess = board.get(up).0;
        if let None = up_chess {
            // 检查马的上面的两个落点是否可以跳～ 
            if let Some ( uu ) = up.up() {
                if let Some ( u1 ) = uu.left() {
                    let c1 = board.get(u1);
                    if ! c1.same_side(camp) {
                        result.push(u1); 
                        count += 1;
                    }
                }
                if let Some ( u2 ) = uu.right() {
                    let c2 = board.get(u2);
                    if ! c2.same_side(camp) {
                        result.push(u2); 
                        count += 1;
                    } 
                } 
            }
        }
    }
    // 检查马的下面是否有棋子
    let down = from.down();
    if let Some ( down ) = down {
        let down_chess = board.get(down).0;
        if let None = down_chess {
            // 检查马的下面的两个落点是否可以跳～ 
            if let Some ( dd ) = down.down() {
                if let Some ( d1 ) = dd.left() {
                    let c1 = board.get(d1);
                    if ! c1.same_side(camp) {
                        result.push(d1); 
                        count += 1;
                    }
                }
                if let Some ( d2 ) = dd.right() {
                    let c2 = board.get(d2);
                    if ! c2.same_side(camp) {
                        result.push(d2); 
                        count += 1;
                    } 
                } 
            }
        }
    } 
    count 
}