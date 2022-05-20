use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::State;
use crate::traits::*;
use crate::pad::{Pad, PadMeta};

#[derive(Debug, Clone, Copy)]
pub struct ElemId(u32);

#[derive(Debug, Clone, Copy)]
pub struct PadId(u32);

pub struct Pipline{
    pads:Vec<Arc<Pad>>,
    elems:Vec<Element>
}

impl Pipline{
    pub const fn new() -> Self{
        return Self { 
            pads: Vec::new(), 
            elems: Vec::new()
        }
    }

    pub fn add_elem<E>(&mut self, elem:E) -> ElemId where E:Into<Element>{
        self.elems.push(elem.into());
        return ElemId(self.elems.len() as u32 -1)
    }

    pub fn add_src<S>(&mut self, src:S) -> ElemId where S:Source + 'static{
        self.elems.push(Element::Source(SourceWrapper{
            srcPad:None,
            src:Arc::new(RwLock::new(src))
        }));
        return ElemId(self.elems.len() as u32 -1)
    }

    pub fn add_sink<S>(&mut self, sink:S) -> ElemId where S:Sink + 'static{
        self.elems.push(Element::Sink(SinkWrapper{
            src:Arc::new(RwLock::new(sink))
        }));
        return ElemId(self.elems.len() as u32 -1)
    }

    /// only DeMuxer can have multiple pads, 
    /// if element have more then one pad, 
    /// no pads are created.
    pub fn elem_add_pad(&mut self, elem:ElemId, pad_name:&str) -> Option<PadId>{
        

        let pad = Arc::new(Pad{
            name:pad_name.to_string(),
            meta:Arc::new(PadMeta::Any),
            sink:None
        });
        self.pads.push(pad.clone());

        let elem = &mut self.elems[elem.0 as usize];

        match elem{
            Element::Source(s) => {
                s.srcPad = Some(pad);
            },
            Element::Sink(s) => return None,
            Element::Transformer(t) => {
                t.srcPad = Some(pad);
            },
            Element::DeMuxer(d) => {
                d.srcPads.push(pad);
            },
            Element::Muxer(m) => {
                m.srcPad = Some(pad);
            }
        }

        return Some(PadId(self.pads.len() as u32 -1))
    }

    pub fn elem_get_pad(&mut self, elem:ElemId, index:usize) -> Option<PadId>{
        match &self.elems[elem.0 as usize]{
            Element::Source(s) => {
                if let Some(p) = &s.srcPad{
                    let mut i = 0;
                    for p1 in &self.pads{
                        if p1.as_ref() as *const _ == p.as_ref() as *const _{
                            return Some(PadId(i))
                        }
                        i+=1;
                    }
                }
            },
            Element::Transformer(t) => if let Some(p) = &t.srcPad{
                let mut i = 0;
                for p1 in &self.pads{
                    if p1.as_ref() as *const _ == p.as_ref() as *const _{
                        return Some(PadId(i))
                    }
                    i+=1;
                }
            },

            Element::Muxer(m) => if let Some(p) = &m.srcPad{
                let mut i = 0;
                for p1 in &self.pads{
                    if p1.as_ref() as *const Pad == p.as_ref() as *const Pad{
                        return Some(PadId(i))
                    }
                    i+=1;
                }
            },

            Element::DeMuxer(d) => {
                if d.srcPads.len() < index{
                    let p = &d.srcPads[index];
                    let mut i = 0;
                    for p1 in &self.pads{
                        if p1.as_ref() as *const Pad == p.as_ref() as *const Pad{
                            return Some(PadId(i))
                        }
                        i+=1;
                    }
                }
            },

            Element::Sink(s) => {

            }
        };
        None
    }

    /// connect the pad to a element, e.g. src -> pad -> sink(element)
    pub fn connect(&mut self, pad:PadId, element:ElemId){
        let pad = &self.pads[pad.0 as usize];
        pad.borrow_mut().sink = Some(self.elems[element.0 as usize].clone())
    }

    pub fn set_state(&self, state:State){
        for e in &self.elems{
            match e{
                Element::Source(s) => {
                    
                    if let Some(p) = s.srcPad.clone(){
                        s.src.write().on_state_change(p, state);
                    }
                },
                _ => {}
            }
        }
    }
}