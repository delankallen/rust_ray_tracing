use color::write_color;
use vec3::Color;
pub mod vec3;
pub mod color;

fn main() {
    const IMAGE_WIDTH:i32 = 256;
    const IMAGE_HEIGHT:i32 = 256;  

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Color {
                e:[
                    (i as f32)/((IMAGE_WIDTH-1) as f32),
                    (j as f32)/((IMAGE_HEIGHT-1) as f32),
                    0.25
                    ]
                };
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
