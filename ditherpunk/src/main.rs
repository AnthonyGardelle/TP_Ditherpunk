use image::{io::Reader as ImageReader, Rgb};
use std::error::Error;

fn rgb8_to_string(composantes: Rgb<u8>) -> String {
    match composantes {
        Rgb([255, 0, 0]) => "red".to_string(),
        Rgb([0, 255, 0]) => "green".to_string(),
        Rgb([0, 0, 255]) => "blue".to_string(),
        Rgb([255, 255, 0]) => "yellow".to_string(),
        Rgb([0, 255, 255]) => "cyan".to_string(),
        Rgb([255, 0, 255]) => "magenta".to_string(),
        Rgb([0, 0, 0]) => "black".to_string(),
        Rgb([255, 255, 255]) => "white".to_string(),
        Rgb([160, 82, 45]) => "sienna".to_string(),
        Rgb([128, 0, 128]) => "purple".to_string(),
        _ => panic!("Couleur non supportée"),
    }
}

fn string_to_rgb8(couleur: &str) -> Rgb<u8> {
    match couleur {
        "red" => Rgb([255, 0, 0]),
        "green" => Rgb([0, 255, 0]),
        "blue" => Rgb([0, 0, 255]),
        "yellow" => Rgb([255, 255, 0]),
        "cyan" => Rgb([0, 255, 255]),
        "magenta" => Rgb([255, 0, 255]),
        "black" => Rgb([0, 0, 0]),
        "white" => Rgb([255, 255, 255]),
        "sienna" => Rgb([160, 82, 45]),
        "purple" => Rgb([128, 0, 128]),
        _ => panic!("Couleur non supportée"),
    }
}

fn euclidean_distance(color1: &Rgb<u8>, color2: &Rgb<u8>) -> f32 {
    let r1 = color1[0] as f32;
    let r2 = color2[0] as f32;
    let g1 = color1[1] as f32;
    let g2 = color2[1] as f32;
    let b1 = color1[2] as f32;
    let b2 = color2[2] as f32;
    ((r2 - r1).powf(2.0) + (g2 - g1).powf(2.0) + (b2 - b1).powf(2.0)).sqrt()
}

fn monochrome_par_paire(chemin_img: &str, paire: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let couleur1 = string_to_rgb8(paire[0]);
    let couleur2 = string_to_rgb8(paire[1]);
    for pixel in img.pixels_mut() {
        let luma = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
        if luma > 128.0 {
            *pixel = couleur1;
        } else {
            *pixel = couleur2;
        }
    }
    let output_path = format!(
        "./static/img/iut_monochrome_{}_{}.jpg",
        rgb8_to_string(couleur1),
        rgb8_to_string(couleur2)
    );
    img.save(output_path)?;
    Ok(())
}

fn passage_a_une_palette(chemin_img: &str, palette: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    for pixel in img.pixels_mut() {
        let mut min_d = std::f32::MAX;
        let mut min_couleur = Rgb([0, 0, 0]);
        for couleur_str in &palette {
            let couleur_rgb = string_to_rgb8(couleur_str);
            let d = euclidean_distance(pixel, &couleur_rgb);
            if d < min_d {
                min_d = d;
                min_couleur = couleur_rgb;
            }
        }
        *pixel = min_couleur;
    }
    let output_path = format!("./static/img/iut_palette_{}.jpg", palette.join("_"));
    img.save(output_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let chemin_img = "./static/img/iut.jpg";
    let paire = vec!["white", "black"];
    let palette2 = vec![
        "red", "green", "blue", "yellow", "cyan", "magenta", "black", "white",
    ];
    monochrome_par_paire(chemin_img, paire)?;
    passage_a_une_palette(chemin_img, palette2)?;
    Ok(())
}
