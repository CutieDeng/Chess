//! 棋盘操作记录器、回放器、追踪器
//! 
//! 用于记录棋盘操作、回放棋盘操作、追踪棋盘操作 
//! 

use crate::chess::ChessBoard;
use crate::point::{Point, point};
use crate::piece::{ChessPiece, Side, ChessType};
use crate::step::Step;

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
    fn step( &mut self, from : Point , to : Point ) -> StepTransaction {
        let eaten = self.inner.0[to.x()][to.y()]; 
        StepTransaction ( self , Step { from , to , eaten } ) 
    } 
}

impl BoardTrack {
    pub fn get_steps ( &self, from : Point , result : &mut Vec< Point > ) -> usize { 
        // 获取棋子 
        let piece = self.inner.0[from.x()][from.y()]; 
        // 检查其上面是否有棋子 
        let Some( (piece_type , side ) ) = piece.0 else {
            return 0; 
        }; 
        match piece_type {
            ChessType::King =>
                king_steps(from, side, &self.inner, result), 
            ChessType::Guard => 
                guard_steps(from, side, &self.inner, result),
            ChessType::Elephant => 
                elephant_steps(from, side, &self.inner, result),
            ChessType::Horse =>
                horse_steps(from, side, &self.inner, result),
            ChessType::Rook =>
                rook_steps(from, side, &self.inner, result),
            ChessType::Cannon =>
                cannon_steps(from, side, &self.inner, result),
            ChessType::Pawn =>
                pawn_steps(from, side, &self.inner, result),
        }
    } 
    pub fn attempt_step( &mut self, from : Point , to : Point ) -> Option<StepTransaction> {
        let mut steps = Vec::new(); 
        self.get_steps(from, &mut steps); 
        for step in steps {
            if step == to {
                return Some ( self.step(from, to) ); 
            } 
        } 
        None 
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

/// 假设当前位置是车，获取它的可行位置 
fn rook_steps ( from : Point , camp : Side, board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 检查车的左边是否有棋子 
    let mut raw_left = from.left(); 
    while let Some ( left ) = raw_left { 
        let left_chess = board.get(left); 
        if let None = left_chess.0 {
            result.push(left); 
            count += 1;
        } else {
            if ! left_chess.same_side(camp) {
                result.push(left); 
                count += 1;
            }
            break;
        }
        raw_left = left.left();
    } 
    // 检查车的右边是否有棋子 
    let mut raw_right = from.right();
    while let Some ( right ) = raw_right { 
        let right_chess = board.get(right); 
        if let None = right_chess.0 {
            result.push(right); 
            count += 1;
        } else {
            if ! right_chess.same_side(camp) {
                result.push(right); 
                count += 1;
            }
            break;
        }
        raw_right = right.right();
    } 
    // 检查车的上面是否有棋子
    let mut raw_up = from.up();
    while let Some ( up ) = raw_up { 
        let up_chess = board.get(up); 
        if let None = up_chess.0 {
            result.push(up); 
            count += 1;
        } else {
            if ! up_chess.same_side(camp) {
                result.push(up); 
                count += 1;
            }
            break;
        }
        raw_up = up.up();
    }
    // 检查车的下面是否有棋子
    let mut raw_down = from.down();
    while let Some ( down ) = raw_down { 
        let down_chess = board.get(down); 
        if let None = down_chess.0 {
            result.push(down); 
            count += 1;
        } else {
            if ! down_chess.same_side(camp) {
                result.push(down); 
                count += 1;
            }
            break;
        }
        raw_down = down.down();
    } 
    count 
}

/// 假设当前位置是炮，获取它的可行位置 
fn cannon_steps ( from : Point , camp : Side , board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 检查炮的左边是否有棋子 
    let mut raw_left = from.left(); 
    let mut left_chess_meet = false;
    while let Some ( left ) = raw_left { 
        let left_chess = board.get(left); 
        if let None = left_chess.0 {
            if ! left_chess_meet { 
                result.push(left); 
                count += 1;
            }
        } else {
            if left_chess_meet {
                if ! left_chess.same_side(camp) {
                    result.push(left); 
                    count += 1;
                }
                break;
            } else {
                left_chess_meet = true; 
            }
        }
        raw_left = left.left();
    } 
    // 检查炮的右边是否有棋子 
    let mut raw_right = from.right();
    let mut right_chess_meet = false;
    while let Some ( right ) = raw_right { 
        let right_chess = board.get(right); 
        if let None = right_chess.0 {
            if ! right_chess_meet { 
                result.push(right); 
                count += 1;
            }
        } else {
            if right_chess_meet {
                if ! right_chess.same_side(camp) {
                    result.push(right); 
                    count += 1;
                }
                break;
            } else {
                right_chess_meet = true; 
            }
        }
        raw_right = right.right();
    }
    // 检查炮的上面是否有棋子
    let mut raw_up = from.up();
    let mut up_chess_meet = false;
    while let Some ( up ) = raw_up { 
        let up_chess = board.get(up); 
        if let None = up_chess.0 {
            if ! up_chess_meet { 
                result.push(up); 
                count += 1;
            }
        } else {
            if up_chess_meet {
                if ! up_chess.same_side(camp) {
                    result.push(up); 
                    count += 1;
                }
                break;
            } else {
                up_chess_meet = true; 
            }
        }
        raw_up = up.up();
    }
    // 检查炮的下面是否有棋子
    let mut raw_down = from.down();
    let mut down_chess_meet = false;
    while let Some ( down ) = raw_down { 
        let down_chess = board.get(down); 
        if let None = down_chess.0 {
            if ! down_chess_meet { 
                result.push(down); 
                count += 1;
            }
        } else {
            if down_chess_meet {
                if ! down_chess.same_side(camp) {
                    result.push(down); 
                    count += 1;
                }
                break;
            } else {
                down_chess_meet = true; 
            }
        }
        raw_down = down.down();
    }
    count 
} 

/// 假设当前位置是象，获取它的可行位置
fn elephant_steps ( from : Point , camp : Side , board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 检查象的左上方是否有棋子 
    let raw_left_up = from.left_up(); 
    // 如果左上方的点位有效
    if let Some ( left_up ) = raw_left_up { 
        // 检查该点位是否有棋子 
        let left_up_chess = board.get(left_up); 
        // 如果没有棋子 
        if let None = left_up_chess.0 {
            // 则检查该点位的左上方是否有棋子 
            if let Some ( left_up_up ) = left_up.left_up() {
                let left_up_up_chess = board.get(left_up_up); 
                // 只要其不与当前象同一方，就可以走到该点位 
                if ! left_up_up_chess.same_side(camp) {
                    result.push(left_up_up); 
                    count += 1;
                } 
            }
        }
    } 
    // 检查象的右上方是否有棋子 
    let raw_right_up = from.right_up();
    // 如果右上方的点位有效
    if let Some ( right_up ) = raw_right_up { 
        // 检查该点位是否有棋子 
        let right_up_chess = board.get(right_up); 
        // 如果没有棋子 
        if let None = right_up_chess.0 {
            // 则检查该点位的右上方是否有棋子 
            if let Some ( right_up_up ) = right_up.right_up() {
                let right_up_up_chess = board.get(right_up_up); 
                // 只要其不与当前象同一方，就可以走到该点位 
                if ! right_up_up_chess.same_side(camp) {
                    result.push(right_up_up); 
                    count += 1;
                } 
            }
        }
    }
    // 检查象的左下方是否有棋子
    let raw_left_down = from.left_down();
    // 如果左下方的点位有效
    if let Some ( left_down ) = raw_left_down { 
        // 检查该点位是否有棋子 
        let left_down_chess = board.get(left_down); 
        // 如果没有棋子 
        if let None = left_down_chess.0 {
            // 则检查该点位的左下方是否有棋子 
            if let Some ( left_down_down ) = left_down.left_down() {
                let left_down_down_chess = board.get(left_down_down); 
                // 只要其不与当前象同一方，就可以走到该点位 
                if ! left_down_down_chess.same_side(camp) {
                    result.push(left_down_down); 
                    count += 1;
                } 
            }
        }
    }
    // 检查象的右下方是否有棋子
    let raw_right_down = from.right_down();
    // 如果右下方的点位有效
    if let Some ( right_down ) = raw_right_down { 
        // 检查该点位是否有棋子 
        let right_down_chess = board.get(right_down); 
        // 如果没有棋子 
        if let None = right_down_chess.0 {
            // 则检查该点位的右下方是否有棋子 
            if let Some ( right_down_down ) = right_down.right_down() {
                let right_down_down_chess = board.get(right_down_down); 
                // 只要其不与当前象同一方，就可以走到该点位 
                if ! right_down_down_chess.same_side(camp) {
                    result.push(right_down_down); 
                    count += 1;
                } 
            }
        }
    }
    count
}

/// 假设当前位置是士，获取它的可行位置 
fn guard_steps( from : Point , camp : Side , board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 获得当前的九宫中心位置 
    let king_pos = match camp {
        Side::Red => {
            point( 4 , 1 ) 
        }
        Side::Black => {
            point( 4 , 8 )
        }
    } .unwrap(); 
    // 检查当前节点的左上方
    let raw_left_up = from.left_up(); 
    if let Some ( left_up ) = raw_left_up { 
        let left_up_chess = board.get(left_up); 
        if ! left_up_chess.same_side(camp) && king_pos.dis(&left_up) <= 2 {
            result.push(left_up); 
            count += 1; 
        }
    } 
    // 检查当前节点的右上方 
    let raw_right_up = from.right_up(); 
    if let Some ( right_up ) = raw_right_up { 
        let right_up_chess = board.get(right_up); 
        if ! right_up_chess.same_side(camp) && king_pos.dis(&right_up) <= 2 {
            result.push(right_up); 
            count += 1; 
        }
    } 
    // 检查当前节点的左下方 
    let raw_left_down = from.left_down(); 
    if let Some ( left_down ) = raw_left_down { 
        let left_down_chess = board.get(left_down); 
        if ! left_down_chess.same_side(camp) && king_pos.dis(&left_down) <= 2 {
            result.push(left_down); 
            count += 1; 
        }
    } 
    // 检查当前节点的右下方 
    let raw_right_down = from.right_down();
    if let Some ( right_down ) = raw_right_down { 
        let right_down_chess = board.get(right_down); 
        if ! right_down_chess.same_side(camp) && king_pos.dis(&right_down) <= 2 {
            result.push(right_down); 
            count += 1; 
        }
    }
    count 
}

/// 假设当前位置是将，获取它的可行位置 
fn king_steps( from : Point , camp : Side , board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    // 获得当前的九宫中心位置
    let king_pos = match camp {
        Side::Red => {
            point( 4 , 1 ) 
        }
        Side::Black => {
            point( 4 , 8 )
        }
    } .unwrap();
    // 检查当前节点的左方
    let raw_left = from.left(); 
    if let Some ( left ) = raw_left { 
        let left_chess = board.get(left); 
        if ! left_chess.same_side(camp) && left.special_area() && king_pos.dis(&left) <= 2 {
            result.push(left); 
            count += 1; 
        }
    } 
    // 检查当前节点的右方 
    let raw_right = from.right();
    if let Some ( right ) = raw_right { 
        let right_chess = board.get(right); 
        if ! right_chess.same_side(camp) && right.special_area() && king_pos.dis(&right) <= 2 {
            result.push(right); 
            count += 1; 
        }
    } 
    // 检查当前节点的上方
    let raw_up = from.up();
    if let Some ( up ) = raw_up { 
        let up_chess = board.get(up); 
        if ! up_chess.same_side(camp) && up.special_area() && king_pos.dis(&up) <= 2 {
            result.push(up); 
            count += 1; 
        }
    }
    // 检查当前节点的下方
    let raw_down = from.down();
    if let Some ( down ) = raw_down { 
        let down_chess = board.get(down); 
        if ! down_chess.same_side(camp) && down.special_area() && king_pos.dis(&down) <= 2 {
            result.push(down); 
            count += 1; 
        }
    }
    // 检查是否王王相见
    let nxt = match camp {
        Side::Red => Point::down,  
        Side::Black => Point::up, 
    }; 
    let mut pos = from; 
    while let Some ( next ) = nxt(&pos) {
        let chess = board.get(next); 
        match chess.0 {
            Some(p) => {
                if p.0 == ChessType::King {
                    result.push(next); 
                    count += 1; 
                }
                break; 
            },
            None => (),
        }
        pos = next; 
    } 
    count 
}

/// 假设当前位置是兵，获取它的可行位置 
fn pawn_steps ( from : Point , camp : Side , board : &ChessBoard , result : &mut Vec < Point > ) -> usize {
    let mut count = 0; 
    let nxt = match camp {
        Side::Red => Point::up, 
        Side::Black => Point::down, 
    }; 
    let raw_up = nxt(&from); 
    if let Some ( up ) = raw_up {
        let up_chess = board.get(up); 
        if ! up_chess.same_side(camp) {
            result.push(up); 
            count += 1; 
        }
    }
    let left_right_support = match camp {
        Side::Red => from.is_black_camp(), 
        Side::Black => from.is_red_camp(), 
    }; 
    if left_right_support {
        let raw_left = from.left(); 
        if let Some ( left ) = raw_left {
            let left_chess = board.get(left); 
            if ! left_chess.same_side(camp) {
                result.push(left); 
                count += 1; 
            }
        }
        let raw_right = from.right(); 
        if let Some ( right ) = raw_right {
            let right_chess = board.get(right); 
            if ! right_chess.same_side(camp) {
                result.push(right); 
                count += 1; 
            }
        } 
    }
    count 
} 