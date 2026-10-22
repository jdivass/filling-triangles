pub mod printer;
pub mod line;
pub mod framebuffer;
pub mod triangle;
pub mod transform;
pub mod object;
pub mod fragment;
pub mod vertex;
pub mod shaders;

use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::shaders::vertex_shader;
use crate::triangle::triangle;
use crate::vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Primer triangulo")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.0));
    framebuffer.init_texture(&mut window, &thread);

    let positions = [
        Vector3::new(300.0, 200.0, 0.0),
        Vector3::new(400.0, 400.0, 0.0),
        Vector3::new(500.0, 200.0, 0.0),
    ];

    let vertex_array: Vec<Vertex> = positions
        .into_iter()
        .map(|position| {
            let mut vertex = Vertex::new_with_color(
                position,
                Vector3::new(1.0, 1.0, 1.0),
            );
            // Las posiciones ya están en coordenadas de pantalla.
            vertex.transformed_position = position;
            vertex
        })
        .collect();

    let uniforms = Uniforms {
        model_matrix: Matrix::identity(),
    };

    while !window.window_should_close() {
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
) {
    // 1. Procesamiento de vértices: aplicar la misma matriz compartida a cada vértice.
    let processed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas: cada grupo completo de tres forma un triángulo.
    for vertices in processed_vertices.chunks_exact(3) {
        // 3. Rasterización: obtener los fragmentos del triángulo.
        let fragments = triangle(&vertices[0], &vertices[1], &vertices[2]);

        // 4. Procesamiento de fragmentos: escribirlos en el framebuffer.
        for fragment in fragments {
            framebuffer.point(
                fragment.position.x as i32,
                fragment.position.y as i32,
                fragment.color,
            );
        }
    }
}
