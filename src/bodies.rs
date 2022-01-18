use crate::material_map::MaterialMap;
use std::collections::HashSet;

const BODY_MIN_SIZE: usize = 50;

pub fn find_bodies(map: &MaterialMap, height: usize, width: usize) -> Vec<HashSet<(usize, usize)>> {
    let mut bodies: Vec<HashSet<(usize, usize)>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if !map.something_at_index(y, x) {
                continue;
            }
            let mut found_left = false;
            let mut left_index = 0;
            let mut found_below = false;
            let mut below_index = 0;
            for i in 0..bodies.len() {
                if x > 0 && bodies[i].contains(&(y, x - 1)) {
                    found_left = true;
                    left_index = i;
                }
                if y > 0 && bodies[i].contains(&(y - 1, x)) {
                    found_below = true;
                    below_index = i;
                }
            }
            if !found_left && !found_below {
                // New body starting
                let mut body = HashSet::new();
                body.insert((y, x));
                bodies.push(body);
            } else if found_left && !found_below {
                bodies[left_index].insert((y, x));
            } else if found_below && !found_left {
                bodies[below_index].insert((y, x));
            } else {
                if left_index == below_index {
                    // Both bodies are the same.
                    bodies[left_index].insert((y, x));
                } else if left_index < below_index {
                    // found in two different bodies, merge them.
                    bodies[left_index].insert((y, x));
                    for coord in bodies[below_index].clone() {
                        bodies[left_index].insert(coord);
                    }
                    bodies.remove(below_index);
                } else {
                    bodies[below_index].insert((y, x));
                    for coord in bodies[left_index].clone() {
                        bodies[below_index].insert(coord);
                    }
                    bodies.remove(left_index);
                }
            }
        }
    }

    // Reduce the bodies list if there aren't enough pixels in the collection.
    bodies.retain(|x| x.len() > BODY_MIN_SIZE);

    bodies
}
