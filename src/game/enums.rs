#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Creatures {
    Dwarf,
    Human,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CreatureState {
    Idle,
    Combat,
    Moving,
    Dead,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Objects {
    Door,
    Bin,
    Chair,
    Table,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ObjectState {
    Normal,
    Damaged,
    Broken,
}
