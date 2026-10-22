use crate::fragment::Fragment;
use crate::vertex::Vertex;

fn append_edge(fragments: &mut Vec<Fragment>, start: &Vertex, end: &Vertex) {
    let mut x = start.transformed_position.x as i32;
    let mut y = start.transformed_position.y as i32;
    let x1 = end.transformed_position.x as i32;
    let y1 = end.transformed_position.y as i32;
    let dx = (x1 - x).abs();
    let dy = -(y1 - y).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let sy = if y < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        fragments.push(Fragment::new(x as f32, y as f32, start.color, 0.0));
        if x == x1 && y == y1 {
            break;
        }
        let twice_error = 2 * error;
        if twice_error >= dy {
            error += dy;
            x += sx;
        }
        if twice_error <= dx {
            error += dx;
            y += sy;
        }
    }
}

pub fn triangle(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    append_edge(&mut fragments, a, b);
    append_edge(&mut fragments, b, c);
    append_edge(&mut fragments, c, a);
    fragments
}
