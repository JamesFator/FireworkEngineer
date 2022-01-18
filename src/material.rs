#[derive(Clone, Debug)]
pub struct RGB {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Free,
    Set,
    Calc,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Sand,
    Stone,
    Pressure,
}

impl Material {
    pub fn rgb(&self) -> RGB {
        match *self {
            Material::Sand => RGB {
                red: 255,
                green: 255,
                blue: 255,
            },
            Material::Stone => RGB {
                red: 102,
                green: 102,
                blue: 153,
            },
            Material::Pressure => RGB {
                red: 253,
                green: 208,
                blue: 23,
            },
        }
    }

    pub fn speed(&self) -> f32 {
        match *self {
            Material::Sand => 0.1f32,
            Material::Stone => 0.1f32,
            Material::Pressure => 0.1f32,
        }
    }
}
