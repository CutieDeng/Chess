use std::{sync::{mpsc::{channel, Sender, Receiver}, atomic::{AtomicBool, self}, Arc, Mutex}, thread::{self, sleep}, net::{TcpListener, TcpStream}, time::Duration, io::{Read, Write}};

use crate::{track::BoardTrack, piece::Side};

/// 服务器数据 
/// 
/// 服务器有两种模式：主模式、从模式
/// 
/// 主模式下，服务器会自动处理所有的游戏逻辑，包括：
/// 下棋操作，检查是否胜利，检查是否和棋... 
/// 
/// 从模式下，服务器依旧校验游戏运行逻辑，但只负责转发消息，不做任何处理 
/// 任何一个 UI 启动时都会绑定一个服务器
pub struct Server {
    pub board : BoardTrack, 
    pub state : ServerState, 
}

/// 服务器状态 
pub enum ServerState {
    /// 服务器空闲 
    Free, 
    /// 服务器处于主模式 
    Master, 
    /// 服务器处于从模式 
    Slave, 
}

/// 客户端向服务器发送的请求 
pub enum Request {
    /// 重置棋盘 
    Reset, 
    /// 移动棋子
    Move( usize , usize ) , 
    /// 交换阵营
    Swap , 
    /// 撤回操作请求
    Back,
    /// 聊天消息
    Message ( String ) , 
    /// 结束服务器
    End, 
}

/// 服务器向客户端发送的响应 
pub enum Response {
    /// 重置棋盘
    Reset , 
    /// 下棋 ( 注释指示是否需要展示动画 )
    Move ( usize , usize , bool ) , 
    /// 胜利 
    Winner ( Option< Side > ), 
    /// 设置玩家阵营
    ActAs ( Side ), 
    /// 撤回操作～
    Back,  
    /// 聊天消息 
    Message ( String ) , 
}

pub enum ServerOpt {
    Local, 
    Network
}

pub fn new_server(opt: ServerOpt ) -> (Sender<Request>, Receiver<Response>) {
    let (sender1, receiver1) = channel(); 
    let (sender2, receiver2) = channel(); 
    let s1 = sender1.clone(); 
    let thread = thread::spawn( move || {
        let re = receiver1 ; 
        let se = sender2; 
        let opt = opt; 
        let back_se = s1; 
        let working = Arc::new ( AtomicBool::new( true ) ); 
        let w2 = working.clone(); 
        let sender_network = Arc::new ( Mutex::new ( None )); 
        let set = sender_network.clone();
        match opt {
            ServerOpt::Local => (), 
            ServerOpt::Network => {
                thread::spawn( move || {
                    let set = set; 
                    let se = back_se; 
                    let w2 = w2; 
                    let socket = TcpListener::bind("0.0.0.0:9898").expect("connect network for LAN play");
                    // 非阻塞模式运行！
                    socket.set_nonblocking(true).unwrap(); 
                    loop {
                        if ! w2.load(atomic::Ordering::Relaxed) {
                            // 关闭网络支持线程～ 
                            return ; 
                        }
                        let a = socket.accept(); 
                        match a {
                            Ok((mut tcp, _addr)) => {
                                {
                                    let mut l = set.lock().unwrap(); 
                                    *l = Some ( tcp.try_clone().unwrap() ) ; 
                                }
                                tcp.set_nonblocking(true).unwrap();
                                tcp.set_read_timeout(Some( Duration::from_secs(3) )).unwrap(); 
                                let mut buffer = Box::new ( [0u8 ; 1024] ); 
                                loop {
                                    if ! w2.load(atomic::Ordering::Relaxed) {
                                        break ; 
                                    }
                                    let r = tcp.read( buffer.as_mut_slice() ); 
                                    match r {
                                        Ok(n) => {
                                            let eff = &buffer[..n]; 
                                            dbg!(&eff);
                                            // ~ ~ ~ ~ ~ 
                                            se.send( Request::Reset ).unwrap(); 
                                        }
                                        Err(_) => (), 
                                    }
                                }
                            },
                            Err(_) => sleep(Duration::from_secs(1)), 
                        }
                    }
                }); 
                // 使用 back se 来构建网络连接支持～ 
            }
        }
        let mut buffer = Box::new ( [0u8 ; 1024] ); 
        loop {
            let r = re.recv().unwrap(); 
            match r {
                Request::Reset => todo!(),
                Request::Move(_, _) => {

                }
                Request::Swap => {

                }
                Request::Back => {

                }
                Request::Message(mes) => {
                    let len = mes.as_bytes().len(); 
                    let len = TryInto::<u8>::try_into(len); 
                    match len {
                        Ok(len) => {
                            buffer[0] = 0; 
                            buffer[1] = len as u8 ; 
                            let mut b = &mut buffer[2..]; 
                            b.write_all( mes.as_bytes() ).unwrap(); 
                            let mut l = sender_network.lock().unwrap(); 
                            if let Some ( tcp ) = l.as_mut().as_deref_mut() {
                                tcp.write_all(&buffer[..mes.as_bytes().len() + 2 ]).unwrap(); 
                            }
                        },
                        Err(_) => (), 
                    }
                }
                Request::End => {
                    working.store(false, atomic::Ordering::Relaxed); 
                }
            }
        }
    }); 
    (sender1, receiver2 )
}


//
pub fn server_run ( )