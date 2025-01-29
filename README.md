# Contenu du fichier /rust-dashboard-app/rust-dashboard-app/README.md

# Rust Dashboard App

Ce projet est une application de tableau de bord utilisant Eframe, Egui et Puffin. Il est structuré selon le modèle MVC (Modèle-Vue-Contrôleur) et permet de visualiser des données à l'aide d'un tableau de bord et d'un diagramme de Gantt.

## Structure du projet

- `src/main.rs`: Point d'entrée de l'application, initialise Eframe et configure la fenêtre principale.
- `src/app.rs`: Contient la structure `App` qui gère l'état de l'application et les composants de l'interface utilisateur.
- `src/models/mod.rs`: Exporte les structures de données utilisées, comme `Dashboard` et `GanttChart`.
- `src/views/mod.rs`: Exporte les fonctions de rendu pour les différentes vues.
- `src/views/dashboard.rs`: Fonction `render_dashboard` pour dessiner le tableau de bord.
- `src/views/gantt.rs`: Fonction `render_gantt` pour dessiner le diagramme de Gantt.
- `src/controllers/mod.rs`: Gère la logique de l'application et les interactions utilisateur.
- `Cargo.toml`: Fichier de configuration pour Cargo, spécifiant les dépendances et les métadonnées.
- `.gitignore`: Liste des fichiers à ignorer par Git.

## Installation

Pour exécuter ce projet, assurez-vous d'avoir Rust et Cargo installés sur votre machine. Clonez le dépôt et exécutez les commandes suivantes :

```bash
cargo build
cargo run
```

## Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à soumettre des problèmes ou des demandes de tirage.