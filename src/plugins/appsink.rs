use std::sync::Arc;

use parking_lot::RwLock;

use crate::{traits::Sink, pad::PadMeta, Capability};


pub trait AppSinkCallback{
    fn on_data_avaliable(&mut self, buf:bytes::BytesMut, meta:&PadMeta);
}

#[derive(Clone)]
pub struct AppSink{
    pub(crate) callback:Option<Arc<RwLock<dyn AppSinkCallback>>>
}

impl AppSink{
    pub fn new() -> Self{
        Self { 
            callback: None 
        }
    }

    pub fn register_callback<C>(&mut self, callback:C) where C:AppSinkCallback+'static{
        self.callback = Some(Arc::new(RwLock::new(callback)));
    }
}

impl Sink for AppSink{
    
    fn sink_capability(&self) -> crate::Capability {
        return Capability::Any()
    }

    fn sink(&mut self, buf:bytes::BytesMut, meta:&PadMeta) {
        if let Some(c) = &self.callback{
            c.write().on_data_avaliable(buf, meta);
        }
    }

    fn on_state_change(&mut self, state:crate::State) {
        
    }
}