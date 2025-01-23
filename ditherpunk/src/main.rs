use image::{io::Reader as ImageReader, Rgb};
use rand::Rng;
use std::error::Error;

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

fn passage_a_une_palette(chemin_img: &str, palette: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut img = ImageReader::open(chemin_img)?.decode()?.to_rgb8();
    for pixel in img.pixels_mut() {
        let couleur_la_plus_proche =
            couleur_la_plus_proche(&pixel, &palette.iter().map(|c| string_to_rgb8(c)).collect());
        *pixel = couleur_la_plus_proche;
    }
    let output_path = format!("./static/img/iut_palette_{}.jpg", palette.join("_"));
    img.save(output_path)?;
    Ok(())
}

fn tramage_random(chemin_img: &str) -> Result<(), Box<dyn Error>> {
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

fn diffusion_d_erreur_simple(chemin_img: &str) -> Result<(), Box<dyn Error>> {
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

    img.save("./static/img/iut_diffusion_d_erreur_simple.jpg")?;
    Ok(())
}

fn diffusion_d_erreur_simple_palette(
    chemin_img: &str,
    palette: &Vec<&str>,
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

    let output_path = format!(
        "./static/img/iut_diffusion_d_erreur_simple_palette_{}.jpg",
        palette.join("_")
    );
    img.save(output_path)?;
    Ok(())
}

fn diffusion_d_erreur_floyd_steinberg_palette(
    chemin_img: &str,
    palette: &Vec<&str>,
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

    let output_path = format!(
        "./static/img/iut_diffusion_d_erreur_floyd_steinberg_palette_{}.jpg",
        palette.join("_")
    );
    img.save(output_path)?;
    Ok(())
}

fn diffusion_d_erreur_palette_matrice(
    chemin_img: &str,
    palette: &Vec<&str>,
    diffusion_matrix: &[&[i32]],
    factor: i32,
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

    let output_path = format!(
        "./static/img/iut_diffusion_d_erreur_palette_matrice_{}.jpg",
        palette.join("_")
    );
    img.save(output_path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let chemin_img = "./static/img/iut.jpg";
    let paire = vec!["white", "black"];
    let palette = vec![
        "red", "green", "blue", "yellow", "cyan", "magenta", "black", "white",
    ];
    let palette2 = vec!["red", "green", "blue", "white", "black"];

    let jarvis_judice_ninke: &[&[i32]] = &[&[0, 0, 0, 7, 5], &[3, 5, 7, 5, 3], &[1, 3, 5, 3, 1]];

    let atkinson: &[&[i32]] = &[&[0, 0, 1, 1], &[1, 1, 1, 0], &[0, 1, 0, 0]];

    monochrome_par_paire(chemin_img, paire)?;
    passage_a_une_palette(chemin_img, &palette)?;
    tramage_random(chemin_img)?;
    ordered_dithering(chemin_img, 3)?;

    diffusion_d_erreur_simple(chemin_img)?;
    diffusion_d_erreur_simple_palette(chemin_img, &palette2)?;
    diffusion_d_erreur_floyd_steinberg_palette(chemin_img, &palette2)?;
    diffusion_d_erreur_palette_matrice(chemin_img, &palette2, jarvis_judice_ninke, 48)?;
    diffusion_d_erreur_palette_matrice(chemin_img, &palette2, atkinson, 8)?;
    Ok(())
}
