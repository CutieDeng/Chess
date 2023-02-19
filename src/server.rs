use std::{sync::{mpsc::{channel, Sender, Receiver}, atomic::{AtomicBool, self}, Arc, Mutex, Weak}, thread::{self, sleep}, net::{TcpListener, TcpStream, UdpSocket}, time::Duration, io::{Read, Write, self, ErrorKind}, collections::VecDeque};

use crate::{track::BoardTrack, piece::Side};

pub struct Registry {
    room_infos : Vec<RoomInfo>,
    udp : UdpSocket, 
}

pub struct RoomInfo {
    pub name: String, 
    pub port : u16 , 
    pub inner : Weak<()>, 
}

impl Registry {
    pub fn run(&mut self, buffer: &mut [u8], counter : u128 ) -> bool {
        let r = self.udp.recv_from(buffer); 
        if counter % 60 == 0 {
            self.room_infos.drain_filter(|r| {
                r.inner.strong_count() == 0 
            }); 
        }
        match r {
            Ok((len, addr)) => {
                let buf = & buffer[..len]; 
                if buf.len() == 4 {
                    if buf == &[ 1, 2, 3, 4 ] {
                        // reply it ! 
                        // if registry is over two, it's not implemented now! 
                        
                    }
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::TimedOut {
                    // ignore it 
                } else {
                    dbg!( &e ); 
                    return false; 
                }
            }
        }
        return true; 
    }
}