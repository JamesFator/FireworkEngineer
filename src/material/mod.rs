#[derive(Clone, Debug)]
pub struct RGB  { pub red: usize, pub green: usize, pub blue: usize  }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State  {
    Free,
    Set,
    Calc,
    Dead
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Sand,
    Water,
    Stone
}

impl Material{
    pub fn rgb(&self) -> RGB {
        match *self {
            Material::Sand => RGB{ red: 255, green: 255, blue: 255 },
            Material::Water => RGB{ red: 255, green: 255, blue: 255 },
            Material::Stone => RGB{ red: 255, green: 255, blue: 255 }
        }
    }

    pub fn  speed(&self) -> f32 {
        match *self {
            Material::Sand => 0.1f32,
            Material::Water => 0.1f32,
            Material::Stone => 0.1f32
        }
    }

    pub fn def_sand() -> Material {
        Material::Sand
    }
}
