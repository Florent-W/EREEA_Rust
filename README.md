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
- Pour créer un nouveau robot, il faut avoir 3 énergies et 5 mminerais.

## Stack
- Rust
- Bevy

## Explications
- Les robots Explorateur vont parcourir la carte, en revenant à la base, ils enregistrent leur progression, dévoilant petit à petit la carte. Les robots collecteurs vont pouvoir collecter les ressources qui vont permettre de créer des nouveaux robots à la base. Le type du robot à la création est désigné au hasard. Les robots ont également plusieurs états : Quand ils sont à la base, ils vont calculer une position au hasard sur la carte qui n'a pas encore été déverouillée, ils vont essayer de ne pas rencontrer d'obstacles et une fois la cible atteinte, ils vont revenir à la base où ils attendront 5 secondes avant de repartir vers une nouvelle cible.