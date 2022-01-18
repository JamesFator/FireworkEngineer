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
    Ground,
    Explosive,
    // Fire duration is the amount of updates it has until it's extinguished.
    Fire { duration: i16, pressure: i8 },
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
            Material::Ground => RGB {
                red: 102,
                green: 102,
                blue: 153,
            },
            Material::Explosive => RGB {
                red: 255,
                green: 255,
                blue: 0,
            },
            Material::Fire { .. } => RGB {
                red: 255,
                green: 0,
                blue: 0,
            },
            Material::Pressure => RGB {
                red: 128,
                green: 128,
                blue: 128,
            },
        }
    }
}
