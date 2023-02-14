use crate::front_end::Board;
use crate::point::{Vector, Point};
use crate::piece::{Camp, Piece};

pub fn pawn_move_check ( op : Vector , camp : Camp , _chess : &Board , ) -> bool {
    let from = op.from(); 
    let to = op.to(); 
    let (x , y ) = (from.col() , from.row() ); 
    let (x1 , y1 ) = (to.col() , to.row() ); 
    let (dx , dy ) = (x1 - x , y1 - y ); 
    let (dx , dy ) = (dx.abs() , dy.abs() ); 
    if dx + dy != 1 { return false } 
    match camp {
        Camp::Red => {
            if y1 > y { return false }  
            if y1 == y && y > 4 { return false } 
        }
        Camp::Black => {
            if y1 < y { return false }   
            if y1 == y && y < 5 { return false } 
        }
    }
    true 
}

pub fn rook_move_check ( op : Vector , _camp : Camp , chess : &Board ) -> bool {
    let from = op.from(); 
    let to = op.to();
    let (x , y ) = (from.col() , from.row() ); 
    let (x1 , y1 ) = (to.col() , to.row() ); 
    let mut count = 0 ; 
    if x == x1 {
        if y == y1 { return false } 
        let min ; let max ; 
        if y < y1 { min = y ; max = y1 } else { min = y1 ; max = y } 
        for i in min + 1 .. max {
            let c = chess.0[ Point::with_row_column(i, x).unwrap().raw() ]; 
            match c {
                Piece::Exist(_) => count += 1, 
                Piece::None => (), 
            }
        } 
    } else if y == y1 {
        let min ; let max ; 
        if x < x1 { min = x ; max = x1 } else { min = x1 ; max = x } 
        for i in min + 1 .. max {
            let c = chess.0[ Point::with_row_column(y, i).unwrap().raw() ];
            match c {
                Piece::Exist(_) => count += 1, 
                Piece::None => (), 
            }
        } 
    } else {
        return false  
    } 
    count == 0 
}

pub fn knight_move_check ( op : Vector , _camp : Camp , chess : &Board ) -> bool {
    let from = op.from(); 
    let to = op.to();
    let (x , y ) = (from.col() , from.row() ); 
    let (x1 , y1 ) = (to.col() , to.row() ); 
    let (dx , dy ) = (x1 - x , y1 - y ); 
    if dx.abs() + dy.abs() != 3 { return false } 
    if dx == 2 {
        if !chess.0 [ from.right().unwrap().raw() ].is_none() {
            return false 
        }
    } else if dx == -2 {
        if !chess.0 [ from.left().unwrap().raw() ].is_none() {
            return false 
        } 
    } else if dy == 2 {
        if !chess.0 [ from.down().unwrap().raw() ].is_none() {
            return false 
        } 
    } else if dy == -2 {
        if !chess.0 [ from.up().unwrap().raw() ].is_none() {
            return false 
        } 
    } else {
        return false 
    } 
    true  
} 

pub fn bishop_move_check ( op : Vector , _camp : Camp , chess : &Board ) -> bool {
    let from = op.from(); 
    let to = op.to();
    let (x , y ) = (from.col() , from.row() ); 
    let (x1 , y1 ) = (to.col() , to.row() ); 
    let (dx , dy ) = (x1 - x , y1 - y ); 
    if dx.abs() + dy.abs() != 4 { return false } 
    match (dx, dy ) {
        (2, 2 ) => {} 
        (2, -2 ) => {} 
        (-2, 2 ) => {} 
        (-2, -2 ) => {}
        _ => return false, 
    }
    true  
}