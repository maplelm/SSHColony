use std::{
    fmt::Display,
    rc::Rc,
    cell::RefCell,
    sync::{Arc, atomic::AtomicUsize}
};
use super::{Object, RenderUnitId, Layer};

pub struct RenderUnit {
    pub id: Arc<RenderUnitId>,
    pub object: Rc<RefCell<Object>>
}

impl RenderUnit {
    pub fn id(&self) -> (usize, Layer) {
        self.id.as_ref().load()
    }

    pub fn is_static(&self) -> bool {
        self.object.borrow().is_static()
    }

    pub fn is_dynamic(&self) -> bool {
        self.object.borrow().is_dynamic()
    }
}
