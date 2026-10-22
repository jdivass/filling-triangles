pub mod printer;
pub mod line;
pub mod framebuffer;
pub mod triangle;
pub mod transform;
pub mod object;
use std::path::Path;

use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::object::{load_obj, Obj};
use crate::triangle::draw_triangle;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("model/object.obj");
    let obj = load_obj(path.to_str().unwrap()).expect("No se pudo cargar el OBJ");
    
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, Color::BLACK);
    framebuffer.set_current_color(Color::WHITE);

    render(&mut framebuffer, &obj);

    framebuffer.render_to_file("render.png");
    println!(
        "Modelo renderizado: {} vértices, {} triángulos -> render.png",
        obj.vertices.len(),
        obj.indices.len() / 3
    );
}

fn render(framebuffer: &mut Framebuffer, obj: &Obj) {
    framebuffer.clear();

    if obj.vertices.is_empty() {
        return;
    }

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for v in &obj.vertices {
        min_x = min_x.min(v.x);
        max_x = max_x.max(v.x);
        min_y = min_y.min(v.y);
        max_y = max_y.max(v.y);
    }

    let model_width = (max_x - min_x).max(1e-6);
    let model_height = (max_y - min_y).max(1e-6);
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    // Escala para que quepa en la ventana dejando un margen del 10%
    let margin = 0.9;
    let scale = (framebuffer.width as f32 * margin / model_width)
        .min(framebuffer.height as f32 * margin / model_height);

    let half_w = framebuffer.width as f32 / 2.0;
    let half_h = framebuffer.height as f32 / 2.0;

    // Centra el modelo y voltea y (en OBJ y crece hacia arriba, en pantalla hacia abajo)
    let to_screen = |v: Vector3| -> Vector2 {
        Vector2::new(
            (v.x - center_x) * scale + half_w,
            half_h - (v.y - center_y) * scale,
        )
    };

    // Cada triángulo son 3 índices consecutivos
    for tri in obj.indices.chunks_exact(3) {
        let a = to_screen(obj.vertices[tri[0]]);
        let b = to_screen(obj.vertices[tri[1]]);
        let c = to_screen(obj.vertices[tri[2]]);
        draw_triangle(framebuffer, a, b, c);
    }
}
