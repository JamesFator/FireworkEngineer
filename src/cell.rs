use crate::material::Material;
use crate::material::State;

#[derive(Copy, Clone, Debug)]
pub struct MaterialRecord {
    pub mat: Material,
    pub state: State
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub contents: Option<MaterialRecord>
}

impl Cell {
    pub fn default() -> Cell {
        Cell { contents: None }
    }
}
