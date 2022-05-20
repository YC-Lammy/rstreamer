#![feature(negative_impls)]

use std::ops::Range;

pub mod traits;
pub mod pad;
pub mod pipeline;
pub mod plugins;

pub use pad::{
    Pad, PadMeta
};
pub use traits::*;
pub use pipeline::{
    ElemId, PadId, Pipline
};


#[derive(Debug, Clone, Copy)]
pub enum State{
    Null,
    Ready,
    Paused,
    Playing,
}

pub enum StreamType{
    Stream,
    Seekable,
    RandomAccess
}

pub enum Prescence{
    Always,
    Sometimes
}

pub struct Capability{
    pub audio:Vec<AudioCapability>,
    pub video:Vec<VideoCapability>,
    pub application:Vec<ApplicationCapability>,
    pub prescence:Prescence
}

impl Capability{
    pub fn Any() -> Self{
        Self { 
            audio: Vec::new(), 
            video: Vec::new(), 
            application: Vec::new(), 
            prescence: Prescence::Always
        }
    }
}

pub struct AudioCapability{
    /// e.g. mpeg, x-raw
    pub container:String,
    pub formats:Vec<String>,
    pub rate:Range<usize>,
    pub channels:Range<usize>,
    pub layouts:Option<Vec<String>>,
}

pub struct VideoCapability{

    /// e.g. x-raw
    pub container:String,
    pub formats:Vec<String>,

    pub width:Range<usize>,
    pub height:Range<usize>,
    pub version:f64,
}

pub struct ApplicationCapability{
    pub container:String,
}


#[test]
fn test_basic(){
    use plugins::appsink::{
        AppSink, AppSinkCallback
    };
    use plugins::appsrc::AppSrc;

    struct Print{

    }
    impl AppSinkCallback for Print{
        fn on_data_avaliable(&mut self, buf:bytes::BytesMut, meta:&PadMeta) {
            println!("{}", std::str::from_utf8(buf.as_ref()).unwrap());
        }
    }

    let mut pipline = Pipline::new();
    let mut sink = AppSink::new();
    let src = AppSrc::new();

    sink.register_callback(Print{});

    let src_e = pipline.add_src(src.clone());
    let pid = pipline.elem_add_pad(src_e, "src").unwrap();
    let sink_e = pipline.add_sink(sink.clone());

    pipline.connect(pid, sink_e);
    let mut b = bytes::BytesMut::new();
    b.extend(b"hello world");
    src.push_buffer(b);

    // should print hello world
    pipline.set_state(State::Playing);
}