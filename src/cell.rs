use crate::material::Material;

#[derive(Copy, Clone, Debug)]
pub struct MaterialRecord {
    pub mat: Material,
    pub force_y: i8,
    pub force_x: i8,
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
