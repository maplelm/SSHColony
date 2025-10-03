use super::{Layer, Object};
use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::{Arc, atomic::AtomicUsize},
};

pub const UNINITIALIZED_RENDERUNITID_VALUE: usize = 0;

pub struct RenderUnit {
    pub id: Arc<RenderUnitId>,
    pub object: Rc<RefCell<Object>>,
}

impl RenderUnit {
    pub fn id(&self) -> usize {
        self.id.as_ref().load()
    }

    pub fn is_static(&self) -> bool {
        self.object.borrow().is_static()
    }

    pub fn is_dynamic(&self) -> bool {
        self.object.borrow().is_dynamic()
    }
}

#[derive(Debug)]
pub enum RenderUnitId {
    Background(AtomicUsize),
    Middleground(AtomicUsize),
    Foreground(AtomicUsize),
    Ui(AtomicUsize),
}

impl RenderUnitId {
    pub fn new(layer: Layer) -> Arc<Self> {
        Arc::new(match layer {
            Layer::Background => {
                RenderUnitId::Background(AtomicUsize::new(UNINITIALIZED_RENDERUNITID_VALUE))
            }
            Layer::Middleground => {
                RenderUnitId::Middleground(AtomicUsize::new(UNINITIALIZED_RENDERUNITID_VALUE))
            }
            Layer::Foreground => {
                RenderUnitId::Foreground(AtomicUsize::new(UNINITIALIZED_RENDERUNITID_VALUE))
            }
            Layer::Ui => RenderUnitId::Ui(AtomicUsize::new(UNINITIALIZED_RENDERUNITID_VALUE)),
        })
    }

    pub fn from_usize(val: usize, layer: Layer) -> Self {
        let val = AtomicUsize::new(val);
        match layer {
            Layer::Background => RenderUnitId::Background(val),
            Layer::Middleground => RenderUnitId::Middleground(val),
            Layer::Foreground => RenderUnitId::Foreground(val),
            Layer::Ui => RenderUnitId::Ui(val),
        }
    }

    pub fn from_atomic(val: AtomicUsize, layer: Layer) -> Self {
        match layer {
            Layer::Background => RenderUnitId::Background(val),
            Layer::Middleground => RenderUnitId::Middleground(val),
            Layer::Foreground => RenderUnitId::Foreground(val),
            Layer::Ui => RenderUnitId::Ui(val),
        }
    }

    pub fn layer(&self) -> Layer {
        match self {
            Self::Background(_) => Layer::Background,
            Self::Middleground(_) => Layer::Middleground,
            Self::Foreground(_) => Layer::Foreground,
            Self::Ui(_) => Layer::Ui,
        }
    }

    pub fn is_bg(&self) -> bool {
        match self {
            RenderUnitId::Background(_) => true,
            _ => false,
        }
    }

    pub fn is_mg(&self) -> bool {
        match self {
            RenderUnitId::Middleground(_) => true,
            _ => false,
        }
    }

    pub fn is_fg(&self) -> bool {
        match self {
            RenderUnitId::Foreground(_) => true,
            _ => false,
        }
    }

    pub fn is_ui(&self) -> bool {
        match self {
            RenderUnitId::Ui(_) => true,
            _ => false,
        }
    }

    pub fn load(&self) -> usize {
        match self {
            Self::Background(val) => val.load(std::sync::atomic::Ordering::SeqCst),
            Self::Middleground(val) => val.load(std::sync::atomic::Ordering::SeqCst),
            Self::Foreground(val) => val.load(std::sync::atomic::Ordering::SeqCst),
            Self::Ui(val) => val.load(std::sync::atomic::Ordering::SeqCst),
        }
    }

    pub fn store(&self, val: usize) {
        match self {
            Self::Background(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
            Self::Middleground(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
            Self::Foreground(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
            Self::Ui(aval) => aval.store(val, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

