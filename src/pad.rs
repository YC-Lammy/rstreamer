use std::ops::Range;
use std::sync::Arc;

use crate::traits::Element;

pub enum PadMeta{
    Audio{
        container:String,
        format:String,
        rate:Range<usize>,
        channels:Range<usize>,
        layout:String,
    },
    Video{
        container:String,
        format:String,

    },
    Application{
        container:String,
    },
    Text{
        encoding:String
    },
    Any
}


pub struct Pad{
    pub(crate) name:String,
    pub(crate) meta:Arc<PadMeta>,

    pub(crate) sink:Option<Element>
}

unsafe impl Send for Pad{}
unsafe impl Sync for Pad{}

impl Pad{
    pub fn name(&self) -> &str{
        &self.name
    }

    pub fn push(&self, buf:bytes::BytesMut){
        match &self.sink.as_ref().unwrap(){
            Element::DeMuxer(d) => {
                d.demuxer.write().chain(&d.srcPads, buf, &self.meta);
            },
            Element::Muxer(m) => {
                let mut guard = m.queue.write();
                guard.0.push(buf);
                guard.1.push(self.meta.clone());
                if guard.0.len() == m.lanes{
                    m.muxer.write().chain(m.srcPad.clone().unwrap(), guard.0.as_slice(), guard.1.as_slice());
                    guard.0.clear();
                    guard.1.clear();
                }
            }
            Element::Transformer(t) => {
                t.transformer.write().chain(t.srcPad.clone().unwrap(), buf, &self.meta);
            },
            Element::Sink(s) => {
                s.src.write().sink(buf, &self.meta);
            },
            Element::Source(s) => unimplemented!()
        }
    }

    /// meta data to pass on to next element
    pub fn set_meta(&self, meta:PadMeta){
        self.borrow_mut().meta = Arc::new(meta);
    }

    pub(crate) fn borrow_mut(&self) -> &mut Self{
        unsafe{(self as *const Self as *mut Self).as_mut().unwrap()}
    }

}