use std::{net::{TcpStream, TcpListener, UdpSocket}, time::Duration, io::{Write, Read, ErrorKind}, collections::VecDeque};

use crate::{track::BoardTrack, piece::Side, point::point};



/// 游戏逻辑控制服务器
/// 
/// 游戏逻辑控制器有两种类型：主控制器，从控制器。
/// 
/// 主控制器将会控制游戏的运行所有细节，包括向其他联网设备发送相关的支持信息～ 
pub struct Server {
    /// 游戏当前棋盘信息
    pub board : BoardTrack , 
    /// 当前允许操作的阵营
    pub now_attempt : Option < Side > , 
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

impl Server {
    pub fn run(&mut self, buffer: &mut [u8]) {
        match &mut self.state {
            State::Master { network, tcp, server, udp } => {
                // 主控制器
                // 1. 接收网络端的操作
                // 2. 向 UI 发送当前棋盘信息
                // 3. 向网络端发送当前棋盘信息

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
                            let r = stream.write_all(&buffer[0..1]); 
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
                            }
                        },
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
                // 从控制器
                // 1. 接收 UI 的操作
                // 2. 向网络端发送当前棋盘信息
                // 3. 向 UI 发送当前棋盘信息
            }, 
        } 
    }
}