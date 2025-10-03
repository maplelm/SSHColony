
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Inventory {
    items: Vec<usize>,
    max_weight: u32,
}

impl Inventory {
    pub fn new(weight: u32) -> Option<Self> {
        Some(Self {
            items: vec![],
            max_weight: weight,
        })
    }
}
