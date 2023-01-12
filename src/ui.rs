use eframe::epaint::Color32;

pub struct AnimationResponse {
    pub repaint: bool, 
}

impl AnimationResponse {
    pub fn new() -> Self {
        Self {
            repaint: false, 
        }
    }
}

pub struct ColorDisplay {
    pub foreground: Color32, 
    pub background: Color32, 
}

pub struct CursorContext {
    pub black: bool, 
    pub white: bool, 
}

pub trait ColorGetter {
    fn get(&mut self, context: &CursorContext) -> (ColorDisplay, AnimationResponse);
}