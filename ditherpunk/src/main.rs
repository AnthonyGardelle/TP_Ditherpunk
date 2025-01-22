use csscolorparser::Color;
use image::{io::Reader as ImageReader, Rgb, Rgba};
use std::error::Error;
use rand::Rng;
use argh::FromArgs;


#[derive(FromArgs, Debug)]
/// Traitement d'image en ligne de commande
struct DitherOptions {
    /// indique l'emplacement de 'image, par défault dans le dossier actuel
    #[argh(option, short = 'r', default = "String::from(\"./\")")]
    read_image: String,

    /// indique le dossier où écrire l'image, par défault dans le dossier actuel
    #[argh(option, short = 'w', default = "String::from(\"./\")")]
    write_to_dir: String,

    /// choix du mode de filtre d'image :
    /// - "mono" utilise un filtre monochrome avec un couple de couleurs,
    /// - "pal" utilise une palette précise,
    /// - "tram" applique l'algorithme de tramage aléatoire,
    #[argh(option, short = 'm')]
    mode: Option<String>,

    /// sélection des couleurs utilisé, soit un couple soit une palette
    #[argh(option, short = 'c')]
    colors: Option<String>,
}

fn rgba8_to_string(composantes: Rgba<u8>) -> String {
    match composantes {
        Rgba([255, 0, 0, 255]) => "red".to_string(),
        Rgba([0, 255, 0, 255]) => "green".to_string(),
        Rgba([0, 0, 255, 255]) => "blue".to_string(),
        Rgba([255, 255, 0, 255]) => "yellow".to_string(),
        Rgba([0, 255, 255, 255]) => "cyan".to_string(),
        Rgba([255, 0, 255, 255]) => "magenta".to_string(),
        Rgba([0, 0, 0, 255]) => "black".to_string(),
        Rgba([255, 255, 255, 255]) => "white".to_string(),
        Rgba([160, 82, 45, 255]) => "sienna".to_string(),
        Rgba([128, 0, 128, 255]) => "purple".to_string(),
        _ => panic!("Couleur non supportée"),
    }
}

fn parse_color(color_str: &str, default: Color) -> [u8; 4] {
    color_str
        .parse::<Color>()
        .unwrap_or_else(|_| {
            println!(
                "Erreur : '{}' n'est pas une couleur valide. Utilisation de la couleur par défaut.",
                color_str
            );
            default.clone()
        })
        .to_rgba8()
}

fn monochrome_par_paire(
    chemin_img: &str,
    paire: &[&str],
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let couleur1 = parse_color(paire[0], Color::new(1.0, 1.0, 1.0, 1.0));
    let couleur2 = parse_color(paire[1], Color::new(0.0, 0.0, 0.0, 1.0));

    for pixel in img.pixels_mut() {
        let luma = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
        if luma > 128.0 {
            *pixel = Rgb([couleur1[0], couleur1[1], couleur1[2]]);
        } else {
            *pixel = Rgb([couleur2[0], couleur2[1], couleur2[2]]);
        }
    }

    let output_path = format!(
        "./static/img/iut_monochrome_{}_{}.jpg",
        rgba8_to_string(Rgba(couleur1)),
        rgba8_to_string(Rgba(couleur2))
    );
    img.save(output_path)?;
    Ok(())
}

fn tramage_random(
    chemin_img: &str,
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let luma = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as f64 / 255.0;
        if luma > rng.gen() {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }

    img.save("./static/img/iut_tramage_random.jpg")?;
    Ok(())
}

fn main() {
    let options: DitherOptions = argh::from_env();

    //println!("{:?}", options);
    // let chemin_img = "./static/img/iut.jpg";
    // let paire = vec!["red", "blue"];
    // monochrome_par_paire(chemin_img, &paire)?;
    // tramage_random(chemin_img)?;
    // Ok(())
}
