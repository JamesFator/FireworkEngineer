#[derive(Clone, Debug)]
pub struct RGB {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Sand,
    Explosive,
    // Fire duration is the amount of updates it has until it's extinguished.
    Fire { duration: i16, pressure: i8 },
    Pressure,
    Wood,
    Cardboard,
}

impl Material {
    pub fn rgb(&self) -> RGB {
        match *self {
            Material::Sand => RGB {
                red: 255,
                green: 255,
                blue: 255,
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
            Material::Wood => RGB {
                red: 94,
                green: 59,
                blue: 19,
            },
            Material::Cardboard => RGB {
                red: 205,
                green: 159,
                blue: 97,
            },
        }
    }

    pub fn density(&self) -> i8 {
        // Density is how susceptible a material is to a force.
        // Higher means it will go further on a push.
        // Don't know if we actually need this.
        match *self {
            Material::Sand => 1,
            Material::Explosive => 1,
            Material::Fire { .. } => 1,
            Material::Pressure => 0,
            Material::Wood => 1,
            Material::Cardboard => 1,
        }
    }
}
