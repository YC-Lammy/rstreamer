

use std::{sync::Arc, cell::Cell};
use std::sync::atomic::AtomicUsize;
use atomic::Atomic;

use parking_lot::RwLock;

use crate::pad::PadMeta;
use crate::{pad::Pad, Capability, State};

pub trait Source{
    fn src_capability(&self) -> Capability;
    fn on_state_change(&mut self, nextPad:Arc<Pad>, state:State);
}

pub trait Muxer{
    fn sink_capability(&self) -> Capability;
    fn src_capability(&self) -> Capability;

    fn chain(&mut self, nextPads:Arc<Pad>, buf:&[bytes::BytesMut], meta:&[Arc<PadMeta>]);
}

pub trait DeMuxer{
    fn sink_capability(&self) -> Capability;
    fn src_capability(&self) -> Capability;

    fn chain(&mut self, nextPad:&[Arc<Pad>], buf:bytes::BytesMut, meta:&PadMeta);
}

pub trait Transformer{
    fn sink_capability(&self) -> Capability;
    fn src_capability(&self) -> Capability;

    fn set_property(&mut self, name:&str, value:i64);
    fn chain(&mut self, nextPad:Arc<Pad>, buf:bytes::BytesMut, meta:&PadMeta);

    fn on_state_change(&mut self, state:State);
}

pub trait Sink {
    fn sink_capability(&self) -> Capability;
    fn sink(&mut self, buf:bytes::BytesMut, meta:&PadMeta);
    fn on_state_change(&mut self, state:State);
}

#[derive(Clone)]
pub enum Element{
    Source(SourceWrapper),
    Muxer(MuxerWrapper),
    DeMuxer(DeMuxerWrapper),
    Transformer(TransformerWrapper),
    Sink(SinkWrapper),
}

#[derive(Clone)]
pub struct SourceWrapper{
    pub(crate) srcPad:Option<Arc<Pad>>,
    pub(crate) src:Arc<RwLock<dyn Source>>,
}

#[derive(Clone)]
pub struct SinkWrapper{
    pub(crate) src:Arc<RwLock<dyn Sink>>
}

#[derive(Clone)]
pub struct MuxerWrapper{
    pub(crate) srcPad:Option<Arc<Pad>>,
    pub(crate) queue:Arc<RwLock<(Vec<bytes::BytesMut>, Vec<Arc<PadMeta>>)>>,
    pub(crate) lanes:usize,
    pub(crate) muxer:Arc<RwLock<dyn Muxer>>,
}

#[derive(Clone)]
pub struct DeMuxerWrapper{
    pub(crate) srcPads:Vec<Arc<Pad>>,
    pub(crate) demuxer:Arc<RwLock<dyn DeMuxer>>
}

#[derive(Clone)]
pub struct TransformerWrapper{
    pub(crate) srcPad:Option<Arc<Pad>>,
    pub(crate) transformer:Arc<RwLock<dyn Transformer>>
}

