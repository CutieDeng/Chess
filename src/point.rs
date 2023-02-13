use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point ( u32 ) ; 

impl Point {
    pub fn with_row_column( row : i32 , col : i32 ) -> Option<Point> {
        if row < 0 || row > 9 || col < 0 || col > 8 {
            None
        } else {
            Some(Point((row * 9 + col) as u32))
        }
    }
    pub fn row( &self ) -> i32 {
        (self.0 / 9) as i32
    }
    pub fn col( &self ) -> i32 {
        (self.0 % 9) as i32
    } 
}

impl Point {
    pub fn left(self) -> Option<Point> {
        Point::with_row_column(self.row(), self.col() - 1)
    }
    pub fn right(self) -> Option<Point> {
        Point::with_row_column(self.row(), self.col() + 1)
    }
    pub fn up(self) -> Option<Point> {
        Point::with_row_column(self.row() - 1, self.col())
    }
    pub fn down(self) -> Option<Point> {
        Point::with_row_column(self.row() + 1, self.col())
    }
    pub fn raw(self) -> usize {
        self.0 as usize
    } 
}

impl Debug for Point {
    fn fmt( &self , f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!(f, "({}, {})", self.row(), self.col())
    }
} 

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector ( u32 ) ; 

impl Vector {
    pub fn with_from_to ( from : Point , to : Point ) -> Vector { 
        Vector( ( from.0 << 8 ) | to.0) 
    } 
    pub fn from ( &self ) -> Point { 
        Point( self.0 >> 8 ) 
    } 
    pub fn to ( &self ) -> Point { 
        Point( self.0 & 0xFF ) 
    } 
}

impl Debug for Vector {
    fn fmt( &self , f : &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.from(), self.to())
    }
}