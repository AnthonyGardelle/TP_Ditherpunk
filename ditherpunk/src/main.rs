use image::{io::Reader as ImageReader, Rgb};
use rand::Rng;
use std::error::Error;
use argh::FromArgs;
use std::path::Path;


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
    /// - "randTram" applique l'algorithme de tramage aléatoire,
    /// - "ordered" applique l'algorithme du ordered dithering,
    /// - "error" applique la diffusion d'erreur avec une palette de couleurs,
    #[argh(option, short = 'm')]
    mode: Mode,

    /// sélection des couleurs utilisé, soit un couple soit une palette : [black, white, red, blue, green, yellow, cyan, magenta, gray]. Pour sélectionner plusieurs couleurs, séparez les couleurs par des virgules
    #[argh(option, short = 'c')]
    colors: Option<String>,

    /// ordre de la matrice Bayer pour l'option "ordered", par défaut 3
    #[argh(option, short = 'o', default = "3")]
    order: u32,
    
    /// choix de la diffusion d'erreur :
    /// - "simple" applique la diffusion d'erreur simple,
    /// - "simplePal" applique la diffusion d'erreur simple avec une palette de couleurs,
    /// - "floyd" applique la diffusion d'erreur de Floyd-Steinberg,
    /// - "jjn" applique la diffusion d'erreur de Jarvis-Judice-Ninke,
    /// - "atkinson" applique la diffusion d'erreur d'Atkinson,
    #[argh(option, short = 'e', default = "String::from(\"simple\")")]
    error: String,
}

/// Enumération des modes disponibles
#[derive(Debug)]
enum Mode {
    Mono,
    Pal,
    RandTram,
    Ordered,
    Error,
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mono" => Ok(Mode::Mono),
            "pal" => Ok(Mode::Pal),
            "randTram" => Ok(Mode::RandTram),
            "ordered" => Ok(Mode::Ordered),
            "error" => Ok(Mode::Error),
            _ => Err(format!("Mode invalide: {}", s)),
        }
    }
}

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

fn couleur_la_plus_proche(pixel: &Rgb<u8>, palette: &Vec<Rgb<u8>>) -> Rgb<u8> {
    let mut min_d = std::f32::MAX;
    let mut min_couleur = Rgb([0, 0, 0]);
    for couleur in palette {
        let d = euclidean_distance(pixel, couleur);
        if d < min_d {
            min_d = d;
            min_couleur = *couleur;
        }
    }
    min_couleur
}

/// Vérifie et formate un chemin de dossier pour s'assurer qu'il termine par un seul "/".
fn format_dossier(chemin_dossier: &str) -> String {
    let chemin = Path::new(chemin_dossier);
    let chemin_str = chemin.to_str().unwrap_or_default();

    if chemin_str.ends_with('/') {
        chemin_str.to_string()
    } else {
        format!("{}/", chemin_str)
    }
}

fn monochrome_par_paire(chemin_img: &str, paire: Vec<&str>, chemin_dossier: &str) -> Result<(), Box<dyn Error>> {
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

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    let output_path = format!(
        "{}iut_monochrome_{}_{}.jpg",
        chemin_dossier_formatte,
        rgb8_to_string(couleur1),
        rgb8_to_string(couleur2)
    );
    img.save(output_path)?;
    Ok(())
}

fn passage_a_une_palette(chemin_img: &str, palette: Vec<&str>, chemin_dossier: &str) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    for pixel in img.pixels_mut() {
        let couleur_la_plus_proche =
            couleur_la_plus_proche(&pixel, &palette.iter().map(|c| string_to_rgb8(c)).collect());
        *pixel = couleur_la_plus_proche;
    }
    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    let output_path = format!("{}iut_palette_{}.jpg", chemin_dossier_formatte, palette.join("_"));
    img.save(output_path)?;
    Ok(())
}

fn tramage_random(
    chemin_img: &str,
    chemin_dossier: &str
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let luma = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32)
            as f64
            / 255.0;
        if luma > rng.gen() {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    img.save(format!("{}iut_tramage_random.jpg", chemin_dossier_formatte))?;
    Ok(())
}

fn ordered_dithering(
    chemin_img: &str,
    ordre: u32,
    chemin_dossier: &str
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

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    img.save(format!("{}iut_ordered_dithering.jpg", chemin_dossier_formatte))?;
    Ok(())
}

fn diffusion_d_erreur_simple(chemin_img: &str, chemin_dossier: &str) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let largeur = img.width();
    let hauteur = img.height();

    for x in 0..largeur {
        for y in 0..hauteur {
            let pixel = img.get_pixel(x, y);
            let luma =
                0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            let nouvelle_valeur = if luma > 128.0 { 255.0 } else { 0.0 };
            let erreur = luma - nouvelle_valeur;

            img.put_pixel(
                x,
                y,
                Rgb([
                    nouvelle_valeur as u8,
                    nouvelle_valeur as u8,
                    nouvelle_valeur as u8,
                ]),
            );

            if x + 1 < largeur {
                let voisin = img.get_pixel(x + 1, y);
                let voisin_luma = 0.2126 * voisin[0] as f32
                    + 0.7152 * voisin[1] as f32
                    + 0.0722 * voisin[2] as f32;
                let valeur_mise_a_jour = voisin_luma + 0.5 * erreur;
                img.put_pixel(
                    x + 1,
                    y,
                    Rgb([
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                    ]),
                );
            }
            if y + 1 < hauteur {
                let voisin = img.get_pixel(x, y + 1);
                let voisin_luma = 0.2126 * voisin[0] as f32
                    + 0.7152 * voisin[1] as f32
                    + 0.0722 * voisin[2] as f32;
                let valeur_mise_a_jour = voisin_luma + 0.5 * erreur;
                img.put_pixel(
                    x,
                    y + 1,
                    Rgb([
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                        (valeur_mise_a_jour.clamp(0.0, 255.0)) as u8,
                    ]),
                );
            }
        }
    }

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    img.save(format!("{}iut_diffusion_d_erreur_simple.jpg", chemin_dossier_formatte))?;
    Ok(())
}

fn diffusion_d_erreur_simple_palette(
    chemin_img: &str,
    palette: &Vec<&str>,
    chemin_dossier: &str
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let largeur = img.width();
    let hauteur = img.height();

    let palette_rgb: Vec<Rgb<u8>> = palette.iter().map(|&c| string_to_rgb8(c)).collect();

    for y in 0..hauteur {
        for x in 0..largeur {
            let current_pixel = img.get_pixel(x, y);
            let closest = couleur_la_plus_proche(current_pixel, &palette_rgb);

            let error = [
                current_pixel[0] as i16 - closest[0] as i16,
                current_pixel[1] as i16 - closest[1] as i16,
                current_pixel[2] as i16 - closest[2] as i16,
            ];

            img.put_pixel(x, y, closest);

            if x + 1 < largeur {
                for c in 0..3 {
                    let neighbor = img.get_pixel_mut(x + 1, y);
                    let value = neighbor[c] as i16 + (error[c] as f32 * 0.5) as i16;
                    neighbor[c] = value.clamp(0, 255) as u8;
                }
            }
            if y + 1 < hauteur {
                for c in 0..3 {
                    let neighbor = img.get_pixel_mut(x, y + 1);
                    let value = neighbor[c] as i16 + (error[c] as f32 * 0.5) as i16;
                    neighbor[c] = value.clamp(0, 255) as u8;
                }
            }
        }
    }

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    let output_path = format!("{}iut_diffusion_d_erreur_simple_palette_{}.jpg", chemin_dossier_formatte, palette.join("_"));

    img.save(output_path)?;
    Ok(())
}

fn diffusion_d_erreur_floyd_steinberg_palette(
    chemin_img: &str,
    palette: &Vec<&str>,
    chemin_dossier: &str
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let largeur = img.width();
    let hauteur = img.height();

    let palette_rgb: Vec<Rgb<u8>> = palette.iter().map(|&c| string_to_rgb8(c)).collect();

    for y in 0..hauteur {
        for x in 0..largeur {
            let current_pixel = img.get_pixel(x, y);
            let closest = couleur_la_plus_proche(current_pixel, &palette_rgb);

            let error = [
                current_pixel[0] as i16 - closest[0] as i16,
                current_pixel[1] as i16 - closest[1] as i16,
                current_pixel[2] as i16 - closest[2] as i16,
            ];

            img.put_pixel(x, y, closest);

            if x + 1 < largeur {
                for c in 0..3 {
                    let neighbor = img.get_pixel_mut(x + 1, y);
                    let value = neighbor[c] as i16 + (error[c] * 7 / 16);
                    neighbor[c] = value.clamp(0, 255) as u8;
                }
            }
            if y + 1 < hauteur {
                if x > 0 {
                    for c in 0..3 {
                        let neighbor = img.get_pixel_mut(x - 1, y + 1);
                        let value = neighbor[c] as i16 + (error[c] * 3 / 16);
                        neighbor[c] = value.clamp(0, 255) as u8;
                    }
                }
                for c in 0..3 {
                    let neighbor = img.get_pixel_mut(x, y + 1);
                    let value = neighbor[c] as i16 + (error[c] * 5 / 16);
                    neighbor[c] = value.clamp(0, 255) as u8;
                }
                if x + 1 < largeur {
                    for c in 0..3 {
                        let neighbor = img.get_pixel_mut(x + 1, y + 1);
                        let value = neighbor[c] as i16 + (error[c] * 1 / 16);
                        neighbor[c] = value.clamp(0, 255) as u8;
                    }
                }
            }
        }
    }

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    let output_path = format!("{}iut_diffusion_d_erreur_floyd_steinberg_palette_{}.jpg", chemin_dossier_formatte, palette.join("_"));

    img.save(output_path)?;
    Ok(())
}

fn diffusion_d_erreur_palette_matrice(
    chemin_img: &str,
    palette: &Vec<&str>,
    diffusion_matrix: &[&[i32]],
    factor: i32,
    chemin_dossier: &str
) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    let (width, height) = img.dimensions();
    let palette_rgb: Vec<Rgb<u8>> = palette.iter().map(|&c| string_to_rgb8(c)).collect();

    for y in 0..height as usize {
        for x in 0..width as usize {
            let old_pixel = img.get_pixel(x as u32, y as u32);
            let old_pixel = Rgb([old_pixel[0] as u8, old_pixel[1] as u8, old_pixel[2] as u8]);

            let new_pixel = couleur_la_plus_proche(&old_pixel, &palette_rgb);
            img.put_pixel(x as u32, y as u32, new_pixel);

            let error = [
                old_pixel[0] as f32 - new_pixel[0] as f32,
                old_pixel[1] as f32 - new_pixel[1] as f32,
                old_pixel[2] as f32 - new_pixel[2] as f32,
            ];

            for (dy, row) in diffusion_matrix.iter().enumerate() {
                for (dx, weight) in row.iter().enumerate() {
                    let nx = x as i32 + dx as i32 - (row.len() / 2) as i32;
                    let ny = y as i32 + dy as i32;

                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let neighbor = img.get_pixel(nx as u32, ny as u32);
                        let mut neighbor_pixel =
                            [neighbor[0] as f32, neighbor[1] as f32, neighbor[2] as f32];

                        for i in 0..3 {
                            neighbor_pixel[i] += error[i] * (*weight as f32 / factor as f32);
                            neighbor_pixel[i] = neighbor_pixel[i].clamp(0.0, 255.0);
                        }

                        img.put_pixel(
                            nx as u32,
                            ny as u32,
                            Rgb([
                                neighbor_pixel[0] as u8,
                                neighbor_pixel[1] as u8,
                                neighbor_pixel[2] as u8,
                            ]),
                        );
                    }
                }
            }
        }
    }

    let chemin_dossier_formatte = format_dossier(chemin_dossier);
    let output_path = format!("{}iut_diffusion_d_erreur_palette_matrice_{}.jpg", chemin_dossier_formatte, palette.join("_"));

    img.save(output_path)?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let options: DitherOptions = argh::from_env();

    let chemin_img = options.read_image;
    let dossier_ecriture = format_dossier(&options.write_to_dir); // Formatage du chemin de dossier
    let mode = options.mode;
    let couleurs = options.colors.unwrap_or_else(|| String::from(""));
    let ordre = options.order;
    let erreur_diffuse = options.error.as_str();

    let jarvis_judice_ninke: &[&[i32]] = &[&[0, 0, 0, 7, 5], &[3, 5, 7, 5, 3], &[1, 3, 5, 3, 1]];
    let atkinson: &[&[i32]] = &[&[0, 0, 1, 1], &[1, 1, 1, 0], &[0, 1, 0, 0]];

    match mode {
        Mode::Mono => {
            let paire: Vec<&str> = couleurs.split(',').collect();
            if paire.len() != 2 {
                return Err("Pour le mode 'mono', fournissez une paire de couleurs (ex: 'white,black')".into());
            }
            monochrome_par_paire(&chemin_img, paire, &dossier_ecriture)?;
        }
        Mode::Pal => {
            let palette: Vec<&str> = couleurs.split(',').collect();
            if palette.iter().all(|&color| color.is_empty()) {
                return Err("Pour le mode 'pal', fournissez une palette de couleurs (ex: 'cyan,green,yellow')".into());
            }
            passage_a_une_palette(&chemin_img, palette, &dossier_ecriture)?;
        }
        Mode::RandTram => {
            tramage_random(&chemin_img, &dossier_ecriture)?;
        }
        Mode::Ordered => {
            ordered_dithering(&chemin_img, ordre, &dossier_ecriture)?;
        }
        Mode::Error => {
            let palette: Vec<&str> = couleurs.split(',').collect();
            println!("{:?}", palette);

            if !["simple"].contains(&erreur_diffuse) {
                if palette.iter().all(|&color| color.is_empty()) {
                    return Err("Pour le mode 'error', fournissez une palette de couleurs (ex: 'cyan,green,yellow')".into());
                }
            }

            println!("{:?}", palette);
            match erreur_diffuse {
                "simple" => diffusion_d_erreur_simple(&chemin_img, &dossier_ecriture)?,
                "simplePal" => diffusion_d_erreur_simple_palette(&chemin_img, &palette, &dossier_ecriture)?,
                "floyd" => diffusion_d_erreur_floyd_steinberg_palette(&chemin_img, &palette, &dossier_ecriture)?,
                "jjn" => diffusion_d_erreur_palette_matrice(&chemin_img, &palette, &jarvis_judice_ninke, 48, &dossier_ecriture)?,
                "atkinson" => diffusion_d_erreur_palette_matrice(&chemin_img, &palette, &atkinson, 8, &dossier_ecriture)?,
                _ => return Err("Diffusion d'erreur non supportée".into()),
            };
        },
    }

    println!(
        "Traitement terminé avec succès. Les images ont été enregistrées dans le dossier : {}",
        dossier_ecriture
    );
    Ok(())
}
