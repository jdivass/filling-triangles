use raylib::prelude::*;

pub struct Framebuffer {
    pub height: u32,
    pub width: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width.try_into().unwrap(), height.try_into().unwrap(), background_color);
        Framebuffer { 
            height, 
            width, 
            color_buffer, 
            background_color, 
            current_color: Color::WHITE, }
    }

    pub fn clear(&mut self){
        self.color_buffer = Image::gen_image_color(self.width.try_into().unwrap(), self.height.try_into().unwrap(), self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32){

        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color){
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color){
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str){
        self.color_buffer.export_image(file_path);
    }

pub fn render_to_bmp(&self, file_path: &str) {
    use std::fs::File;
    use std::io::Write;

    let width = self.width as usize;
    let height = self.height as usize;

    // Cada fila debe ser múltiplo de 4 bytes
    let row_size = (width * 3 + 3) & !3;
    let pixel_array_size = row_size * height;
    let file_size = 54 + pixel_array_size;

    let mut file = File::create(file_path).unwrap();

    // ===== BMP HEADER =====
    file.write_all(b"BM").unwrap();
    file.write_all(&(file_size as u32).to_le_bytes()).unwrap();
    file.write_all(&[0u8; 4]).unwrap();
    file.write_all(&(54u32).to_le_bytes()).unwrap();

    // ===== DIB HEADER =====
    file.write_all(&(40u32).to_le_bytes()).unwrap();
    file.write_all(&(width as i32).to_le_bytes()).unwrap();
    file.write_all(&(height as i32).to_le_bytes()).unwrap();
    file.write_all(&(1u16).to_le_bytes()).unwrap();
    file.write_all(&(24u16).to_le_bytes()).unwrap();
    file.write_all(&(0u32).to_le_bytes()).unwrap();
    file.write_all(&(pixel_array_size as u32).to_le_bytes()).unwrap();
    file.write_all(&(0i32).to_le_bytes()).unwrap();
    file.write_all(&(0i32).to_le_bytes()).unwrap();
    file.write_all(&(0u32).to_le_bytes()).unwrap();
    file.write_all(&(0u32).to_le_bytes()).unwrap();

    // Padding por fila
    let padding = row_size - width * 3;
    let padding_bytes = vec![0u8; padding];

    // BMP guarda las filas de abajo hacia arriba
    for y in (0..height).rev() {
        for x in 0..width {

            let color = self.color_buffer.get_color(x as i32, y as i32);

            // BMP usa BGR
            file.write_all(&[
                color.b,
                color.g,
                color.r,
            ]).unwrap();
        }

        file.write_all(&padding_bytes).unwrap();
    }
}
}
