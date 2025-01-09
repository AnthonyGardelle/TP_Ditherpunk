# 🎨 **Ditherpunk: Retour au Monochrome** 🖤

## ✨ Introduction

Bienvenue dans ce projet intitulé **Ditherpunk : Retour au Monochrome**.  
L'objectif de ce TP est de manipuler des images à l'aide de la bibliothèque Rust `image` afin de les transformer en versions simplifiées, utilisant un nombre réduit de couleurs :  
- 🖤 **Monochrome (noir et blanc)**  
- 🎨 **Palette limitée**  

Ce travail pratique s'inspire de la page [Ditherpunk de Surma](https://surma.dev/things/ditherpunk/), que vous pourrez consulter pour des explications approfondies sur les concepts de dithering et de réduction de palette.

---

## 🎯 Objectifs

Vous développerez une **application en ligne de commande** qui permet d'effectuer divers traitements d'image. Cette application devra :  

✅ **Sélectionner une image d’entrée**  
✅ **Choisir un nom pour le fichier de sortie** *(par défaut : `out.png`)*  
✅ **Appliquer un traitement spécifique** parmi ceux que vous aurez implémentés, avec des options configurables  

Les choix et les commandes seront gérés à l'aide de la bibliothèque [argh](https://crates.io/crates/argh).

---

## 👥 Équipe

- **Membres** :  
  - 🧑‍💻 Ouzet Killian - 31A
  - 🧑‍💻 Gardelle Anthony - 31A  

---

## ⚙️ Fonctionnalités à implémenter

Les fonctionnalités principales incluent :  
1️⃣ **Transformation d’images en monochrome (noir et blanc)**  
2️⃣ **Réduction d’une image à une palette limitée**  
3️⃣ **Gestion des options via une interface en ligne de commande intuitive**  
4️⃣ **Exportation des résultats dans un fichier image**  

Les parties prioritaires du projet sont les **sections 1 à 4 et 7** de l'énoncé.  
Les sections **5 et 6** permettent d’aller plus loin pour obtenir une solution remarquable.  

---

## 🛠️ Contraintes techniques

La bibliothèque `image` sera utilisée dans sa version **0.24** :  
📄 Documentation : [image 0.24.9](https://docs.rs/image/0.24.9/image/index.html)  

Cela garantit la compatibilité avec le compilateur Rust installé sur les machines de l’IUT.  

---

## 📅 Date de rendu

⏳ **Le projet doit être finalisé et remis au plus tard le 22 janvier.**  

Bonne chance et amusez-vous bien avec le **ditherpunk** ! 🚀
