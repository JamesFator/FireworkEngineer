use rand;
use rand::Rng;

pub fn circle(
    r: f32,
    y: i32,
    x: i32,
    height: usize,
    width: usize,
    opacity: f32,
) -> Vec<(usize, usize)> {
    let mut v = Vec::new();

    let uwidth = width as i32;
    let uheight = height as i32;
    let _r = r as i32;
    for _y in (y - _r)..(y + _r) {
        let b = (r.powi(2) - ((y - _y) as f32).powi(2)).sqrt().floor() as i32;
        for _x in (x - b)..(x + b) {
            if rand::thread_rng().gen_range(0f32..1.) > opacity
                && _x >= 0
                && _y >= 0
                && _x < uwidth
                && _y < uheight
            {
                v.push((_y as usize, _x as usize));
            }
        }
    }

    return v;
}
