# RUST_BareMetal

Ce dépôt est une collection de projets et d'exercices en Rust, axés sur la programmation embarquée et le développement bare-metal, ainsi que des problèmes de programmation et des implémentations de machines virtuelles.

## Structure du dépôt

Le dépôt est organisé en plusieurs sous-répertoires, chacun représentant un projet ou un ensemble de problèmes distinct.

### `fibo`

Ce sous-répertoire contient un projet Rust pour le calcul de la suite de Fibonacci. Il utilise la bibliothèque `clap` pour la gestion des arguments en ligne de commande, ce qui suggère une application console pour calculer des nombres de Fibonacci.

*   **Fonctionnalité principale**: Calcul du n-ième nombre de Fibonacci de manière itérative avec vérification de débordement (`u32`).
*   **Utilisation**: Le programme peut être exécuté avec des arguments pour spécifier la plage de calcul et le mode verbeux.
    ```bash
    # Exemple d'utilisation
    cargo run --release -- -v --mini 0 --value 10
    ```
*   **Dépendances**: `clap` (pour l'analyse des arguments en ligne de commande).

### `problems`

Ce répertoire est destiné à contenir des solutions à divers problèmes de programmation. Il s'agit d'un projet Rust standard sans dépendances externes spécifiques listées dans son `Cargo.toml`, ce qui indique qu'il pourrait s'agir de problèmes algorithmiques ou de structures de données de base.

*   **Utilisation**: Contient des implémentations de solutions à des défis de programmation.

### `tp-led-matrix`

Ce projet est un pilote pour une matrice LED RGB 8x8, développé en Rust pour les microcontrôleurs STM32 en utilisant le framework Embassy. Il met en œuvre des fonctionnalités avancées pour le contrôle des LEDs.

*   **Fonctionnalités**: 
    *   Contrôle de matrice LED 8x8 avec multiplexage de lignes.
    *   Correction gamma pour la précision des couleurs.
    *   Streaming d'images série via UART (38400 bauds).
    *   Économiseur d'écran avec défilement de texte (
