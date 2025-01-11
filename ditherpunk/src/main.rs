use image::{io::Reader as ImageReader, Rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageReader::open("./static/img/iut.jpg")?
        .decode()?
        .to_rgb8();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
    }
    img.save("./static/img/iut_blanc_un_sur_deux.jpg")?;
    Ok(())
}
