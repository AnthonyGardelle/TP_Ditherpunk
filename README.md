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

##  Réponses aux questions 

### 1. La bibliothèque image

### Question 1

- Créer un nouveau projet Cargo, avec une dépendance sur la bibliothèque image, version 0.24.

  - Réponse :  
    Pour créer un nouveau projet Cargo et le configurer pour utiliser image, il faut exécuter les commande suivante :  

    ```bash
    cargo new ditherpunk
    cd ditherpunk
    cargo add image@0.24.9
    ```

---

### Question 2

- À quoi correspond le type DynamicImage ?

  - Réponse :  
    DynamicImage est une enum pour les formats ImageBuffer qui prend en charge les différents types d'image comme par exemple les images RGB8, RGBA8, ou Grayscale.

-  Comment obtenir une image en mode rbg8 à partir de ce DynamicImage ?

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

### Question 3

- Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal
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

### Question 4

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

### Question 5

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