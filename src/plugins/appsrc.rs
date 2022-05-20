use std::{sync::Arc, thread};

use parking_lot::RwLock;
use lock_api::*;

use crossbeam::channel::{
    unbounded,
    Sender, Receiver
};

use crate::{traits::{Sink, Source}, Capability};


#[derive(Clone)]
pub struct AppSrc{
    sender:Sender<bytes::BytesMut>,
    reciever:Receiver<bytes::BytesMut>,

    callback:Option<Arc<dyn AppSrcCallback>>,

    bytes_queued:Arc<RwLock<u64>>,

    max_bytes:Arc<RwLock<u64>>,
}

pub trait AppSrcCallback{
    fn on_need_data(&self, appsrc:&AppSrc, length:usize);
    fn on_enough_data(&self, appsrc:&AppSrc);
    fn on_seek_data(&self, appsrc:&AppSrc, offset:usize);
}

impl AppSrc{
    pub fn new() -> AppSrc{
        let (s, r) = unbounded();
        return AppSrc {
            sender:s,
            reciever:r,

            callback:None,

            bytes_queued:Arc::new(RwLock::new(0)),

            max_bytes:Arc::new(RwLock::new(200000)),
        }
    }

    pub fn register_callback<T>(&mut self, callback:T) where T:AppSrcCallback + 'static{
        self.callback = Some(Arc::new(callback))
    }

    pub fn push_buffer(&self, buf:bytes::BytesMut){
        let mut guard = self.bytes_queued.write();
        *guard += buf.len() as u64;
        if *guard > *self.max_bytes.read(){
            if let Some(v) = &self.callback{
                v.on_enough_data(self);
            }
        }
        self.sender.send(buf);
    }
}

impl Source for AppSrc{
    fn src_capability(&self) -> crate::Capability {
        return Capability::Any();
    }
    
    fn on_state_change(&mut self, nextPad:Arc<crate::pad::Pad>, state:crate::State) {
        let recv = self.reciever.clone();
        loop{
            let b = recv.recv().unwrap();
            nextPad.push(b);
        }
    }
}