use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    vertex1: Vector2,
    vertex2: Vector2,
    vertex3: Vector2
    ) {
        line(framebuffer, vertex1, vertex2);
        line(framebuffer, vertex2, vertex3);
        line(framebuffer, vertex3, vertex1);
}
