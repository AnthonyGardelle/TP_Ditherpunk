# Ditherpunk: Retour au Monochrome

## 👥 Équipe

- **Membres** :
  - 🧑‍💻 Ouzet Killian - 31B
  - 🧑‍💻 Gardelle Anthony - 31B

## ⚙️ Fonctionnalités à implémenter

Les fonctionnalités principales incluent :  
1️⃣ **Transformation d'images en monochrome (noir et blanc)**  
2️⃣ **Réduction d'une image à une palette limitée**  
3️⃣ **Gestion des options via une interface en ligne de commande intuitive**  
4️⃣ **Exportation des résultats dans un fichier image**

Les parties prioritaires du projet sont les **sections 1 à 4 et 7** de l'énoncé.  
Les sections **5 et 6** permettent d'aller plus loin pour obtenir une solution remarquable.

## 📅 Date de rendu

⏳ **Le projet doit être finalisé et remis au plus tard le 23 janvier.**

## Réponses aux questions

### 1. La bibliothèque image

#### Question 1

- Créer un nouveau projet Cargo, avec une dépendance sur la bibliothèque image, version 0.24.

  - Réponse :  
    Pour créer un nouveau projet Cargo et le configurer pour utiliser image, il faut exécuter les commande suivante :

    ```bash
    cargo new ditherpunk
    cd ditherpunk
    cargo add image@0.24.9
    ```

---

#### Question 2

- À quoi correspond le type DynamicImage ?

  - Réponse :  
    DynamicImage est une enum pour les formats ImageBuffer qui prend en charge les différents types d'image comme par exemple les images RGB8, RGBA8, ou Grayscale.

- Comment obtenir une image en mode rbg8 à partir de ce DynamicImage ?

  - Réponse :  
    Pour convertir une image DynamicImage en mode rgb8 il faut utiliser la méthode to_rgb8().

    ```rust
    use image::io::Reader as ImageReader;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
      let img = ImageReader::open("./static/img/iut.jpg")?.decode()?;
      let img_rgb = img.to_rgb8();
      Ok(())
    }
    ```

---

#### Question 3

- Sauver l'image obtenue au format png. Que se passe-t-il si l'image de départ avait un canal
  alpha?

  - Réponse :  
    Pour sauver l'image obtenue on peut utiliser la méthode save().

    ```rust
    use image::io::Reader as ImageReader;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
      let img = ImageReader::open("./static/img/iut.jpg")?.decode()?;
      let img_rgb = img.to_rgb8();
      img_rgb.save("./static/img/out_img_rgb.png")?;
      Ok(())
    }
    ```

    Si l'image de départ avait un canal alpha alors celui-ci est recalculé par rapport au autre canaux car la méthode to_rgb8() renvoie une image de seulement trois composante RGB (Rouge Vert Bleu).

---

#### Question 4

- Afficher dans le terminal la couleur du pixel (32, 52) de l'image de votre choix.

  - Réponse :  
    Pour afficher dans le terminal la couleur d'un pixel il faut utiliser la méthode get_pixel() puis récupérer les data du pixel.

    ```rust
    use image::{io::Reader as ImageReader, GenericImageView};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let img = ImageReader::open("./static/img/iut.jpg")?.decode()?;
        let pixel = img.get_pixel(32, 52);
        println!("Couleur : R={} G={} B={} A={}", pixel[0], pixel[1], pixel[2], pixel[3]);
        Ok(())
    }
    ```

---

#### Question 5

- Passer un pixel sur deux d'une image en blanc. Est-ce que l'image obtenue est reconnaissable ?

  - Réponse :  
    Oui, l'image est quand même reconnaissable.

    ```rust
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
    ```

---

### 2. Passage en monochrome par seuillage

#### Question 6

- Comment récupérer la luminosité d'un pixel ?

  - Réponse :  
    Pour récupérer la luminosité d'un pixel il faut calculer la luminance de celui-ci. La luminance (luma) est une grandeur correspondant à la sensation visuelle de luminosité d'une surface. On peut ensuite donc calculer luma avec une opération de matriçage des composantes RGB. On peut calculer la luminance en faisant la moyenne des trois composantes mais ce n'est pas trés réaliste car l'Humain est plus sensible au vert qu'au rouge ou au bleu. Nous allons donc utilisé la recommandations UIT-R BT 709 (Rec. 709) qui est une norme dans l'industrie audiovisuelle pour la télévision à haute définition (TVHD).
    ```math
      Y' = 0.2126 * R' + 0.7152 * G' + 0.0722 * B'
    ```
    Signification des termes :
    - Y' : La luminosité perçue du pixel (valeur en niveaux de gris).
    - R' : La composante rouge de la couleur du pixel (entre 0 et 255).
    - G' : La composante verte de la couleur du pixel (entre 0 et 255).
    - B' : La composante bleue de la couleur du pixel (entre 0 et 255).
    - Les coefficients 0.2126, 0.7152, et 0.0722 reflètent la sensibilité relative de l'œil humain au rouge, au vert et au bleu, respectivement.

---

#### Question 7

- Implémenter le traitement

  - Réponse :

    ```rust
    use image::{io::Reader as ImageReader, Rgb};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut img = ImageReader::open("./static/img/iut.jpg")?
            .decode()?
            .to_rgb8();
        for (_x, _y, pixel) in img.enumerate_pixels_mut() {
            let luma = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            if luma > 128.0 {
                *pixel = Rgb([255, 255, 255]);
            } else {
                *pixel = Rgb([0, 0, 0]);
            }
        }
        img.save("./static/img/iut_monochrome_blanc_noir.jpg")?;
        Ok(())
    }
    ```

---

#### Question 8

- Permettre à l'utilisateurice de remplacer "noir" et "blanc" par une paire de couleurs au choix.

  - Réponse :

    Lors de l'appel de la fonction **monochrome_par_paire** il faut lui mettre en paramètre le chemin de l'image source et un vecteur contenant les deux couleur de la paire.

    ```rust
    use csscolorparser::Color;
    use image::{io::Reader as ImageReader, Rgb, Rgba};
    use std::error::Error;

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

    fn main() -> Result<(), Box<dyn Error>> {
        let chemin_img = "./static/img/iut.jpg";
        let paire = vec!["red", "blue"];
        monochrome_par_paire(chemin_img, &paire)?;
        Ok(())
    }
    ```

---

### 4 Tramage aléatoire (dithering)

#### Question 12

- Implémenter le tramage aléatoire des images.

  - Réponse :

    ```rust
    use image::{io::Reader as ImageReader, Rgb};
    use std::rand;
    use std::rand::Rng;

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


    fn main() -> Result<(), Box<dyn Error>> {
        let chemin_img = "./static/img/iut.jpg";
        let paire = vec!["red", "blue"];
        monochrome_par_paire(chemin_img, &paire)?;
        tramage_random(chemin_img)?;
        Ok(())
    }
    ```

  - Résultat :

  ![image](ditherpunk/static/img/iut_tramage_random.jpg)

---

### 5 Utilisation de la matrice de Bayer comme trame

#### Question 13

- Déterminer B3

  - Réponse :

    - U2 :

    $$
    \left(\begin{array}{cc}
    1 & 1 & 1 & 1\\
    1 & 1 & 1 & 1\\
    1 & 1 & 1 & 1\\
    1 & 1 & 1 & 1
    \end{array}\right)
    $$

    - B3 :

    $$
    1/64.
    \left(\begin{array}{cc}
    \left(\begin{array}{cc}
    0 & 32 &8 & 40 \\
    48 & 16 & 56 & 24\\
    12 & 44 & 4	& 36\\
    60 & 28 & 52 & 20
    \end{array}\right)
    \left(\begin{array}{cc}
    2 & 34 & 10 &	42\\
    50 & 18 & 58 & 26\\
    14 & 46 & 6 & 38\\
    62 & 30 & 54 & 22\\
    \end{array}\right)
    \\
    \left(\begin{array}{cc}
    3 & 35 & 11	& 43\\
    51 & 19	& 59 & 27\\
    15 & 47 & 7 & 39\\
    63 & 31	& 55 & 23\\
    \end{array}\right)
    \left(\begin{array}{cc}
    1 & 33 & 9 & 41\\
    49 & 17	& 57 & 25\\
    13 & 45 & 5	& 37\\
    61 & 29	& 53 & 21\\
    \end{array}\right)
    \\
    \end{array}\right)
    $$

- Résultat :

  $$
  1/64.
  \left(\begin{array}{cc}
  0 & 32 &8 & 40 & 2 & 34 & 10 &	42 \\
  48 & 16 & 56 & 24 & 50 & 18 & 58 & 26\\
  12 & 44 & 4	& 36 & 14 & 46 & 6 & 38\\
  60 & 28 & 52 & 20 & 62 & 30 & 54 & 22\\
  3 & 35 & 11	& 43 & 1 & 33 & 9 & 41\\
  51 & 19	& 59 & 27 & 49 & 17	& 57 & 25\\
  15 & 47 & 7 & 39 & 13 & 45 & 5	& 37\\
  63 & 31	& 55 & 23 & 61 & 29	& 53 & 21\\
  \end{array}\right)
  $$

#### Question 14

- Quel type de données utiliser pour représenter la matrice de Bayer? Comment créer une matrice de Bayer d’ordre arbitraire?

  - Réponse :

    Pour implémenter la matrice de bayer nous avons pensé évidemment à l'utilisation des tableaux et plus particuièrement d'un tableau contenant d'autres tableaux, afin de pouvoir mieux représenter les lignes et colonnes. Pour ce faire nous avons donc créer un struct "MatriceBayer" contenant la matrice de type "Vec<Vec<u32>>" et la taille de celle-ci en "usize".

    ```rust
    /// Représente une matrice de Bayer, avec les fonctions pour la générer
    struct MatriceBayer {
        taille: usize,
        matrice: Vec<Vec<u32>>,
    }
    ```

#### Question 15

- Implémenter le tramage par matrice de Bayer.

  - Réponse :

    L'implémentation du tramage par matrice de Bayer, dit Ordered Dithering, se sépare en deux partie. La première partie est la création et le calcul de l'algorithme et la deuxième partie est l'utilisation de cette matrice pour filter l'image.

    Les fonctions de générations sont implémenté directement dans le struct. Celles-ci sont au nombre de deux :

    - new_bayer_matrix(...), la fonction à appeler pour générer une matrice de bayer
    - calculer_bayer(...), une fonction récursive qui calcule chaque partie de la matrice.

  ```rust
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
  ```

  Et enfin la fonction permettant de faire le tramage. 
  Dans les tramages précédents, nous utilisions un seuil pour comparer avec la luminostié du pixel. 
  Ici le seuil est l'une des valeurs de la matrice. 
  En effet, le but du ordered dithering est de placer la matrice comme un motif sur l'image et pour chaque pixel nous prenons comme seuil la valeur superposé. 
  Si jamais l'image à une plus grande résolution que la taille de la matrice alors la matrice est réutilisé à la chaine comme une mosaïque.

  ```rust
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
  ```

  - Résultat :

  ![image](ditherpunk/static/img/iut_ordered_dithering.jpg)
