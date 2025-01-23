use csscolorparser::Color;
use image::{io::Reader as ImageReader, Rgb, Rgba};
use std::error::Error;
use rand::Rng;

#[derive(Debug)]
/// Représente une matrice de Bayer, avec les fonctions pour la générer
struct MatriceBayer {
    taille: usize,
    matrice: Vec<Vec<u32>>,
}

impl MatriceBayer {
    /// Génère une matrice de Bayer d'ordre `n`
    fn new_bayer_matrix(ordre: u32) -> Self {
        let taille = 2usize.pow(ordre);
        let mut matrice = vec![vec![0; taille]; taille];
        MatriceBayer::calculer_bayer(&mut matrice, 0, 0, taille, 1, 0);
        MatriceBayer { taille, matrice }
    }

    /// Génération récursive de la matrice
    fn calculer_bayer(matrice: &mut Vec<Vec<u32>>, x: usize, y: usize, taille: usize, etape: u32, valeur: u32) {
        if taille == 0 {
            matrice[y][x] = 0;
            return;
        }

        //println!("{:?}", valeur);

        if taille == 1 {
            matrice[y][x] = valeur;
            return;
        }

        //let matrice_clone = matrice.clone();
        // for row in matrice_clone {
        //     println!("{:?}", row);
        // }

        let milieu = taille / 2;

        //Calcule haut gauche
        Self::calculer_bayer(matrice, x, y, milieu, etape*4, valeur+(etape*0));
        //Calcule bas droite
        Self::calculer_bayer(matrice, x+milieu, y+milieu, milieu, etape*4, valeur+(etape*1));
        //Calcule haut droit
        Self::calculer_bayer(matrice, x+milieu, y, milieu, etape *4, valeur+(etape*2));
        //Calcule bas gauche
        Self::calculer_bayer(matrice, x, y+milieu, milieu, etape *4, valeur+(etape*3));

    }
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

fn ordered_dithering(
    chemin_img: &str,
    ordre: u32,
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();

    let bayer = MatriceBayer::new_bayer_matrix(ordre);

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let luma = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as f64 / 255.0;
        let seuil = bayer.matrice[(_y % bayer.taille as u32) as usize][(_x % bayer.taille as u32) as usize] as f32 * 1.0 / (bayer.taille * bayer.taille) as f32;
        if luma > seuil.into() {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }

    img.save("./static/img/iut_ordered_dithering.jpg")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let chemin_img = "./static/img/iut.jpg";
    let paire = vec!["red", "blue"];
    monochrome_par_paire(chemin_img, &paire)?;
    tramage_random(chemin_img)?;
    ordered_dithering(chemin_img, 3)?;

    Ok(())
}
