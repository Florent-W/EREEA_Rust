#  Essaim de Robots pour l'Exploration et l'Étude Astrobiologique (EREEA)

## Description du projet
Le projet consiste en une simulation d'exploration spatiale effectués par des robots autonomes. Les robots effectuent des missions pour collecter des ressources, ainsi que de l'énergie. Ils découvriront également des lieux d'interêt d'interêt scientifique. Certains robots pourront également compléter la map affichée.

## Equipe
- Florent Weltmann
- Dantin Durand
- William Girard-Reydet 
- Axel Gourdin
- Abdoula Jaiteh

## Commande pour lancer le projet
- Rendez-vous dans le répertoire du projet.
- Lancer la commande `cargo run`. Le programme va compiler et va se lancer !

## Les options
Au lancement de la simulation, différentes options vous seront proposés :
- Vous pourrez entrer un seed pour générer une map que vous avez déjà obtenue ou bien appuyer sur Entrée si vous voulez générer une map avec un seed aléatoire.
- Choisir une résolution de la fenêtre.
- Choisir le nombre de robots que vous souhaitez faire apparaître sur la carte au début de la simulation.
- Choisir la taille de la map (Exemple, vous entrez 50, la taille de la carte sera de 50*50)

## Commande pour lancer les tests unitaires
- Pour lancer les tests, rendez-vous dans le répertoire du projet et lancer la commande `cargo test`.

## Commandes
- Zoom / Dezoom : Molette de la souris
- Déplacement sur la carte : Touche directionnelle du clavier
- F1 : Ralentir la vitesse de la simulation
- F2 : Accélerer la vitesse de la simulation
- F3 : Enlever / Activer les bordures
- F11 : Passer en plein écran / Mode fenêtre
- Echap : Quitter le jeu
- Tab : Passer en mode découverte / retirer le mode découverte (Le mode découverte permet de voir toute la carte)

Une légende est également indiqué à gauche de la carte.

## Les différents robots
- Visiteur (Orange et Bleu) : Visite la carte mais ne fait pas avancer le dévoilement de la carte
- Explorateur (Vert) : Visite la carte, fait avancer le dévoilement de la carte
- Collecteur (Bleu) : Visite la carte, ne fait pas avancer le dévoilement de la carte mais collecte les ressources telles que les énergies et les minerais.

## Création d'un nouveau robot
- Pour créer un nouveau robot, il faut avoir 3 énergies et 5 minerais.

## Stack
- Rust
- Bevy

## Explications
- Les robots Explorateur vont parcourir la carte, en revenant à la base, ils enregistrent leur progression, dévoilant petit à petit la carte. 
- Les robots collecteurs vont pouvoir collecter les ressources qui vont permettre de créer des nouveaux robots à la base. Le type du robot à la création est désigné au hasard. 
- Les robots ont également plusieurs états : Quand ils sont à la base, ils vont calculer une position au hasard sur la carte qui n'a pas encore été déverouillée, ils vont essayer de ne pas rencontrer d'obstacles et une fois la cible atteinte, ils vont revenir à la base où ils attendront 5 secondes avant de repartir vers une nouvelle cible.

---

Cargo fmt && cargo clippy....

code with inconsicency in language.

a lot of test in comment.

Some test.

Integration test more than unit test.

Magic number, underuse of matches.

presence of changelog. (I feel like it's in revese order)

ADR (in french).

few dependency. but could make some optimisation of the build.

lake of builder and factory methods

wrong use of comment.

some function have a high cyclomatic complexity.

The game is not very pretty, there is sound, there is also a bug on the return of the robot.

Archi is basic but clean.

I think the ARD 3 is not well explain / suited. You should take position about `dyn` implementation and enum wrapper.


SUmmary :

 project     : EREEA_Rust
 repo age    : 2 months 
 branch:     : review
 last active : 9 seconds ago
 active on   : 14 days
 commits     : 78
 files       : 41
 uncommitted :        0
 authors     : 
    56  Florent-W      71.8%
     6  dantin-durand  7.7%
     4  WilliamGR-dev  5.1%
     3  Groot          3.8%
     3  Swann HERRERA  3.8%
     2  Abdou365       2.6%
     2  Axel Gourdin   2.6%
     2  gourdinax      2.6%

 project     : EREEA_Rust
 lines       :     8326
 authors     :
6917 Florent-W      83.1%
 745 Groot          8.9%
 497 Swann HERRERA  6.0%
 155 gourdinax      1.9%
   7 dantin-durand  0.1%
   5 Abdou365       0.1%

 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Markdown                5          243            0          181           62
 TOML                    1           22           17            1            4
-------------------------------------------------------------------------------
 Rust                   16         1984         1192          652          140
 |- Markdown             1            2            0            2            0
 (Total)                           1986         1192          654          140
===============================================================================
 Total                  22         2249         1209          834          206
