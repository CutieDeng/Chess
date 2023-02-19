//! 坐标描述
//! 
//! 使用 [`Point`] 类型描述象棋上的任意位置的坐标关系
//! 

/// 游戏坐标类型，坐标为：(x, y)
/// 特别地，x 用于描述坐标列，而 y 用于描述坐标行
/// 这与程序中 GUI 的坐标关系具有一致性。 
#[derive( PartialEq, Eq, Debug, Hash, Clone, Copy )]
pub struct Point ( usize, usize ); 

/// 根据 Point 的 x 和 y 属性构建 Point 类型 
pub fn point ( x : usize , y : usize ) -> Option< Point > {
    if x >= 9 {
        return None 
    }
    if y >= 10 {
        return None 
    }
    Some ( Point ( x , y ) ) 
}

impl Point {
    pub fn x(&self) -> usize {
        self.0 
    }
    pub fn y(&self) -> usize {
        self.1 
    }
    pub fn raw(&self) -> usize {
        self.0 * 10 + self.1 
    }
}

impl Point {
    pub fn up(&self) -> Option < Point >  {
        if self.1 == 0 {
            None 
        } else {
            Some(Point( self.0, self.1 - 1 )) 
        }
    }
    pub fn down(&self) -> Option < Point > {
        if self.1 == 9 {
            None 
        } else {
            Some(Point( self.0, self.1 + 1 )) 
        }
    }
    pub fn left(&self) -> Option < Point > {
        if self.0 == 0 {
            None 
        } else {
            Some(Point( self.0 - 1 , self.1 ))
        }
    }
    pub fn right(&self) -> Option < Point > {
        if self.0 == 8 {
            None 
        } else {
            Some( Point ( self.0 + 1, self.1 ))
        }
    }
}

#[test]
fn test_point_1_2() {
    let point = point(1, 2); 
    let point = point.unwrap(); 
    assert_eq! ( point.x() , 1 ); 
    assert_eq! ( point.y() , 2 ); 
}

#[test] 
fn test_point_9_1() {
    let point = point(9, 1); 
    assert!( point.is_none() ); 
}

#[test] 
fn test_point_8_1() {
    let point = point( 8 , 1 ) ;
    let point = point.unwrap(); 
    assert_eq! ( point.x() , 8 ); 
    assert_eq! ( point.y() , 1 ); 
    assert_eq! ( point.raw() , 81 ); 
}

#[test]
fn test_point_8_right() {
    let point = point( 8 , 1 ); 
    let point = point.unwrap(); 
    let right_point = point.right(); 
    assert! ( right_point.is_none() ); 
}

#[test] 
fn test_point_upword_2() {
    let point = point ( 0 , 2 ) ; 
    let point = point.unwrap().up().unwrap().up().unwrap().up(); 
    assert! ( point.is_none() ); 
}

impl Point {
    /// 判断是否为红方势力点位
    pub fn is_red_camp(&self) -> bool {
        self.1 <= 4 
    }
    /// 判断是否为黑方势力点位 
    pub fn is_black_camp(&self) -> bool {
        self.1 >= 5 
    }
}

impl Point {
    /// 左上方的点位 
    pub fn left_up( &self ) -> Option < Point > {
        self.left().as_ref().map(Point::up).flatten() 
    } 
    /// 右上方的点位 
    pub fn right_up( &self ) -> Option < Point > {
        self.right().as_ref().map(Point::up).flatten() 
    } 
    /// 左下方的点位 
    pub fn left_down( &self ) -> Option < Point > {
        self.left().as_ref().map(Point::down).flatten() 
    } 
    /// 右下方的点位 
    pub fn right_down( &self ) -> Option < Point > {
        self.right().as_ref().map(Point::down).flatten() 
    } 
}

impl Point {
    pub fn dis( &self, other: &Point ) -> usize {
        let x = self.0 as isize - other.0 as isize; 
        let y = self.1 as isize - other.1 as isize; 
        ( x.abs() + y.abs() ) as usize 
    } 
    pub fn special_area( &self ) -> bool {
        self.0 >= 3 && self.0 <= 5 
    }
}