use std::sync::mpsc::{Sender, Receiver};
use std::collections::VecDeque;
use std::io::{Write, Read, ErrorKind};
use std::time::Duration;
use std::net::{TcpStream, TcpListener, UdpSocket};

use crate::track::BoardTrack;
use crate::piece::Side;
use crate::point::{point, Point};



/// 游戏逻辑控制服务器
/// 
/// 游戏逻辑控制器有两种类型：主控制器，从控制器。
/// 
/// 主控制器将会控制游戏的运行所有细节，包括向其他联网设备发送相关的支持信息～ 
pub struct Server {
    /// 游戏当前棋盘信息
    pub board : BoardTrack , 
    /// 控制器状态
    pub state: State, 
    /// 读入缓存 
    pub buffer: VecDeque<u8>, 
}

pub enum State {
    Master {
        /// 主状态：允许网络端操作的阵营
        network : Side , 
        /// tcp 连接～ 
        tcp : Option<TcpStream>, 
        /// 服务器
        server : TcpListener , 
        /// UDP 监听 
        udp : UdpSocket , 
    }, 
    Slave {
        /// 从状态：允许 UI 操作的阵营
        local : Side ,
        /// tcp 连接～ 
        tcp : TcpStream , 
    }, 
}

/// 棋盘操作请求 
pub enum Request {
    /// 移动操作 
    MoveOperation ( Point , Point ) , 
    /// 悔棋操作    
    RollbackOperation , 
}

impl Server {
    pub fn run(&mut self, buffer: &mut [u8], ui_sender : Sender<Request> , ui_receiver : Receiver<Request> ) {
        match &mut self.state {
            State::Master { network, tcp, server, udp } => {
                // 指示当前网络端支持的额外连接～
                let r = udp.recv_from(buffer); 
                match r {
                    Ok((_, reply)) => {
                        let port = server.local_addr().unwrap().port(); 
                        buffer[0] = port as u8; 
                        buffer[1] = (port >> 8) as u8; 
                        udp.send_to(&buffer[0..2], reply).unwrap();
                    },
                    Err(_) => {},
                }
                // 尝试获得新的 tcp 连接 
                if tcp.is_none() {
                    let r = server.accept(); 
                    match r {
                        Ok((mut stream, _)) => {
                            stream.set_read_timeout(Some(Duration::from_secs(0))).unwrap();
                            stream.set_nonblocking(true).unwrap(); 
                            // 连接完成后，接受到的第一个字节将会是对方的阵营信息～ 
                            // 0x00 为红方，0x01 为黑方 
                            match network {
                                Side::Red => buffer[0] = 0x00,
                                Side::Black => buffer[0] = 0x01, 
                            }
                            let r = stream.write_all(&buffer[..1]); 
                            if r.is_ok() {
                                *tcp = Some(stream); 
                            }
                        },
                        Err(_) => {},
                    }
                } 
                // 尝试处理一般的操作请求 
                if let Some ( raw_tcp ) = tcp {
                    let size = raw_tcp.read( buffer ); 
                    match size {
                        Ok(len) => {
                            // 写入缓存 
                            self.buffer.write_all(&buffer[..len]).unwrap(); 
                        },
                        Err(e) => {
                            dbg!(&e); 
                            // 连接已经断开～... 应该吧... 
                            if e.kind() == ErrorKind::TimedOut {
                                // 超时，不做处理～
                            } else {
                                *tcp = None; 
                                eprintln!("哎，连接断开了噢 😯");
                            }
                        },
                    }
                }
                'ui_operator: 
                {
                    if let Ok(tr) = ui_receiver.try_recv() {
                        match tr {
                            Request::MoveOperation(from, to) => {
                                // 检查一下现在是不是在联机？ 
                                if tcp.is_some() {
                                    // 在联机！所以检查一下 UI 操作是不是合法的捏～ 
                                    let op_chess = self.board.chess_board().get(from); 
                                    if let Some((_, side)) = op_chess.0 {                                    
                                        if side == *network {
                                            // 搞什么鬼呀，不要动人家的棋呀 
                                            eprintln!( "😠 不要动人家的棋子！" ); 
                                            break 'ui_operator; 
                                        }
                                    } else {
                                        break 'ui_operator; 
                                    }
                                }
                                let step = self.board.attempt_step(from, to); 
                                if let Some(st) = step {
                                    // 广播操作啦！ 
                                    ui_sender.send( Request::MoveOperation(from, to) ).unwrap(); 
                                    if let Some(raw_tcp) = tcp {
                                        buffer[0] = from.raw() as u8 ; 
                                        buffer[1] = to.raw() as u8 ; 
                                        let r = raw_tcp.write_all(&buffer[..2]); 
                                        if r.is_err() {
                                            eprintln!("💭 我就想下一步棋，咋就断网了呢");
                                            *tcp = None; 
                                        }
                                    }
                                    st.commit(); 
                                }
                            }
                            Request::RollbackOperation => {
                                // 不想做～ 
                                eprintln! ( "这个功能（撤销）不想做捏，不要为难我 😡" ); 
                            }
                        }
                    }
                }
                // 单步操作处理
                // 检查缓存中是否有操作请求 
                if self.buffer.len() >= 2 {
                    // 有操作请求～
                    let first_byte = self.buffer.pop_front().unwrap();
                    let second_byte = self.buffer.pop_front().unwrap(); 
                    // 检查这是什么操作！
                    if first_byte < 90 && second_byte < 90 { 
                        // 移动操作～
                        let from = (first_byte / 10, first_byte % 10); 
                        let to = (second_byte / 10, second_byte % 10); 
                        let from = point(from.0.into(), from.1.into()).unwrap(); 
                        let to = point(to.0.into(), to.1.into()).unwrap(); 
                        // 处理移动操作请求 ～ 
                        let side = self.board.chess_board().get(from); 
                        match side.0 {
                            Some((_, side)) => {
                                if side == *network { 
                                    // 获得操作许可～ 
                                    let s = self.board.attempt_step(from, to); 
                                    match s {
                                        Some(s) => {
                                            // 操作成功～ 
                                            s.commit(); 
                                            if let Some (raw_tcp) = tcp {
                                                // 回显操作～ 
                                                let r = raw_tcp.write_all(&buffer[0..2]);  
                                                match r {
                                                    Ok(_) => {},
                                                    Err(_) => {
                                                        // 操作失败～ 
                                                        eprintln!( "😢 连接断开，发消息发不出去了捏。" ); 
                                                        *tcp = None; 
                                                    },
                                                }
                                            } 
                                        },
                                        None => {
                                            // 操作失败～ 
                                            // 忽略吧 
                                            eprintln! ( "操作失败：{:?} -> {:?}" , from , to ) ; 
                                        },
                                    }
                                } 
                            },
                            None => {
                                // what the hell ? you cheat on me ? 
                            },
                        }
                    } else if first_byte == 0x10 {
                        // 检查是否是悔棋操作～
                        if second_byte == 0x00 {
                            // 悔棋操作～
                            eprintln!("请求悔棋：还没实现呢小蠢猪 🐷"); 
                        } 
                    } 
                }
            },
            State::Slave { local, tcp } => {
                // 这下有的搞了 ... 我啥也不知道，咋写程序呢 ... 
                // 先收消息吧 emmm 
                let r = tcp.read(buffer); 
                match r {
                    Ok(len) => {
                        // 我在自己的缓冲区里写呢，怎么可能出事呢啊哈哈哈！
                        self.buffer.write_all( &buffer[..len] ).unwrap(); 
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::TimedOut {
                            // 超时，就算了吧
                        } else {
                            // 啊呀，这可咋办呀，断网了吗？
                            eprintln!("😨 从机断网了好像，咋整呀？"); 
                        }
                    },
                }
                // 先收一下 UI 消息吧 emmm 
                let r = ui_receiver.try_recv(); 
                match r {
                    Ok(r) => {
                        match r {
                            Request::MoveOperation(from, to) => {
                                // 是否是有效操作 
                                let success = self.board.attempt_step(from, to).is_some(); 
                                // 是否这是我的阵营的棋子操作～ 
                                let local = self.board.chess_board().get(from).same_side(*local); 
                                if success && local {
                                    // 发送消息给对方啦！
                                    buffer[0] = from.raw() as u8 ; 
                                    buffer[1] = to.raw() as u8 ; 
                                    tcp.write_all(&buffer[..2]).expect( "从服务器发送操作给主服务器" ); 
                                }
                            }
                            Request::RollbackOperation => {
                                eprintln!("🈚️ 真没写，我真不会写这个！"); 
                            }
                        }
                    },
                    Err(_) => {},
                }
                // 听听对面说了啥吧，咱可是从机～
                if self.buffer.len() >= 2 { 
                    let first = self.buffer.pop_front().unwrap() as usize ;
                    let second = self.buffer.pop_front().unwrap() as usize ; 
                    // 适当检查一下这是不是一个合法 Point ~ 
                    let first = point( first / 10 , first % 10 ).unwrap(); 
                    let second = point ( second / 10 , second % 10 ).unwrap(); 
                    self.board.attempt_step(first, second).unwrap(); 
                    ui_sender.send( Request::MoveOperation(first, second) ).unwrap(); 
                }
            }, 
        } 
    }
}