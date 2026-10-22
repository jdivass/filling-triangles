use crate::fragment::Fragment;
use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}

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

fn wireframe_triangle(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    append_edge(&mut fragments, a, b);
    append_edge(&mut fragments, b, c);
    append_edge(&mut fragments, c, a);
    fragments
}

fn mix_color(a: raylib::math::Vector3, b: raylib::math::Vector3, t: f32) -> raylib::math::Vector3 {
    raylib::math::Vector3::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
    )
}

fn filled_barycentric(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let p0 = a.transformed_position;
    let p1 = b.transformed_position;
    let p2 = c.transformed_position;
    let area = (p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x);
    if area.abs() < f32::EPSILON {
        return Vec::new();
    }

    let min_x = p0.x.min(p1.x).min(p2.x).floor() as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).ceil() as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).floor() as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).ceil() as i32;
    let mut fragments = Vec::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32;
            let py = y as f32;
            let w0 = ((p1.x - px) * (p2.y - py) - (p1.y - py) * (p2.x - px)) / area;
            let w1 = ((p2.x - px) * (p0.y - py) - (p2.y - py) * (p0.x - px)) / area;
            let w2 = 1.0 - w0 - w1;

            if w0 >= -1e-6 && w1 >= -1e-6 && w2 >= -1e-6 {
                let color = raylib::math::Vector3::new(
                    a.color.x * w0 + b.color.x * w1 + c.color.x * w2,
                    a.color.y * w0 + b.color.y * w1 + c.color.y * w2,
                    a.color.z * w0 + b.color.z * w1 + c.color.z * w2,
                );
                let depth = p0.z * w0 + p1.z * w1 + p2.z * w2;
                fragments.push(Fragment::new(px, py, color, depth));
            }
        }
    }
    fragments
}

fn filled_scanline(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let vertices = [a, b, c];
    let min_y = vertices
        .iter()
        .map(|v| v.transformed_position.y)
        .fold(f32::INFINITY, f32::min)
        .ceil() as i32;
    let max_y = vertices
        .iter()
        .map(|v| v.transformed_position.y)
        .fold(f32::NEG_INFINITY, f32::max)
        .floor() as i32;
    let mut fragments = Vec::new();

    for y in min_y..=max_y {
        let scan_y = y as f32;
        let mut intersections: Vec<(f32, raylib::math::Vector3, f32)> = Vec::with_capacity(3);
        for (start, end) in [(a, b), (b, c), (c, a)] {
            let p0 = start.transformed_position;
            let p1 = end.transformed_position;
            if (scan_y >= p0.y.min(p1.y))
                && (scan_y < p0.y.max(p1.y))
                && (p0.y - p1.y).abs() > f32::EPSILON
            {
                let t = (scan_y - p0.y) / (p1.y - p0.y);
                let x = p0.x + (p1.x - p0.x) * t;
                let color = mix_color(start.color, end.color, t);
                let depth = p0.z + (p1.z - p0.z) * t;
                intersections.push((x, color, depth));
            }
        }
        if intersections.len() < 2 {
            continue;
        }
        intersections.sort_by(|left, right| left.0.total_cmp(&right.0));
        let (left_x, left_color, left_depth) = intersections[0];
        let (right_x, right_color, right_depth) = intersections[intersections.len() - 1];
        let first_x = left_x.ceil() as i32;
        let last_x = right_x.floor() as i32;
        for x in first_x..=last_x {
            let t = if (right_x - left_x).abs() < f32::EPSILON {
                0.0
            } else {
                (x as f32 - left_x) / (right_x - left_x)
            };
            fragments.push(Fragment::new(
                x as f32,
                scan_y,
                mix_color(left_color, right_color, t),
                left_depth + (right_depth - left_depth) * t,
            ));
        }
    }
    fragments
}

pub fn triangle(a: &Vertex, b: &Vertex, c: &Vertex, mode: TriangleMode) -> Vec<Fragment> {
    match mode {
        TriangleMode::Wireframe => wireframe_triangle(a, b, c),
        TriangleMode::FilledBarycentric => filled_barycentric(a, b, c),
        TriangleMode::FilledScanline => filled_scanline(a, b, c),
    }
}
