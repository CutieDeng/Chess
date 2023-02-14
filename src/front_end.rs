use std::sync::mpsc::Receiver;

use eframe::{NativeOptions, CreationContext, Frame};
use eframe::epaint::{FontFamily, Color32};
use eframe::egui::{FontDefinitions, FontData, Context};

use crate::{point::Point, piece::Piece, message::Message};

/// 前端显示操作，阻塞调用，在绘图过程被调用完成后返回 —— 一般不再返回。
pub fn main ( app_name : &str,  native_options : NativeOptions ) { 
    let _ = eframe::run_native(app_name, native_options, Box::new ( app_creator ) ); 
}

fn app_creator ( creation_context : &CreationContext ) -> Box < dyn eframe::App > { 
    font_config_once(&creation_context.egui_ctx); 
    todo!()
}

/// 配置字体
/// 
/// 一共设置了两种中文字体。
/// monospace ：隶书
/// proportional : 楷体
fn font_config_once ( ctx : &Context ) {
    let mut fonts = FontDefinitions::default();  
    fonts.font_data.insert("cutiefont".to_owned(), FontData::from_static(
        include_bytes!("../assets/fonts/LXGWWenKai-Regular.ttf") 
    )); 
    fonts.font_data.insert("lyshu".to_owned(), FontData::from_static(
        include_bytes!("../assets/fonts/RuiZiYunZiKuLiShuTiGBK.ttf") 
    )); 
    fonts.families.entry(FontFamily::Proportional).or_default().push("cutiefont".to_owned()); 
    fonts.families.entry( FontFamily::Monospace ) .or_default().push("lyshu".to_owned()); 
    ctx.set_fonts(fonts); 
}

pub struct MyApp {
    /// 时钟 / 计时器 
    pub clock : u128 , 
    /// 棋盘展示配置 
    pub color_config : ColorConfig , 
    /// 棋盘数据展示
    pub board : BoardTracker, 
    /// 棋盘接受控制器的输入 
    pub input : Receiver<Message>, 
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        todo!()
    }
}

/// 描述棋盘相关的所有属性
pub struct ChessBoard {
    /// 棋盘基础属性 
    pub base : Base , 
    /// 棋盘颜色配置（日间模式）
    pub light : ColorConfig , 
    /// 棋盘颜色配置（夜间模式）
    pub dark : ColorConfig , 
}

pub struct Base {
    /// 一个格子的大小
    pub grid_size : f32 , 
    /// 棋子所占格子比例 
    pub piece_ratio : f32 , 
    /// 光标大小
    pub cursor_ratio : f32 , 
}

/// 描述棋子在某一阵营下的相关变换
pub struct CampDisplay {
    /// 棋子背景色
    pub background : BackGround , 
    /// 棋子边框颜色 
    pub border_color : Color32 ,  
    /// 棋子上文字颜色 
    pub text_color : Color32 , 
}

pub struct ChessGround {
    /// 棋盘背景色
    pub background : BackGround , 
    /// 棋盘边框颜色 
    pub border_color : Color32 , 
    /// 棋盘文字颜色
    pub text_color : Color32 , 
}

/// 描述了基本的棋盘颜色配置 
pub struct ColorConfig {
    /// 棋盘
    pub chess : ChessGround , 
    /// 红方 
    pub red : CampDisplay , 
    /// 黑方 
    pub black : CampDisplay ,
}

/// 棋子背景色 
/// 
/// 目前只实现了单色背景 
/// 
pub enum BackGround {
    /// 单色背景 
    SingleColor ( Color32 ) , 
}

/// 交互控制器
pub struct Interactive {
    /// 红方光标 
    pub red : Cursor , 
    /// 黑方光标 
    pub black : Cursor , 
}

pub enum Animation {
    /// 无动画 
    Nothing, 
    /// 选中动画
    Selected ( Point ) , 
    /// 走棋动画 
    Move ( LineAnimation ) , 
}

pub struct LineAnimation {
    /// 起点 
    pub start : Point , 
    /// 终点 
    pub end : Point , 
    /// 动画起始时间
    pub start_time : u128 ,
    /// 移动结束时间 
    pub move_end_time : u128 ,
    /// 变换结束时间
    pub scale_end_time : u128 , 
    /// 终点棋子留存
    pub end_piece : Piece, 
}

pub struct Cursor {
    /// 光标位置 
    pub position : Point, 
    /// 光标可见性、可达性
    pub visible : bool , 
}

/// 棋盘内容
pub struct Board ( pub Box::<[ Piece ; 90 ]> ); 

pub struct BoardTracker {
    /// 棋盘内容
    pub board : Board , 
    /// 棋盘交互控制器
    pub interactive : Interactive , 
    /// 棋盘动画
    pub animation : Animation , 
}