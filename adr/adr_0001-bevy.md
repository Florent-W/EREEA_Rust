# ADR 0001: Utilisation de Bevy pour le Développement de l'Interface de visualisation de la carte

## Status
**Accepté**

## Contexte
Nous envisageons le développement d'une application qui nécessite une interface pour visualiser la carte sur lequel il y a des robots parcourant une planète constitués de ressources et d'éléments géologiques. Deux principales technologies en RUST sont considérées: 
- Le terminal
- Bevy, un moteur de jeu ECS. Bevy offre des capacités graphiques étendues et une architecture basée sur les systèmes et les composants, tandis que l'interface en terminal offre une simplicité et une compatibilité étendue avec les systèmes à faible ressource.

## Décision
Nous avons décidé d'utiliser **Bevy** pour le développement de notre interface.

## Raisonnement

### Avantages de Bevy
- **Interactivité**: Bevy permet de bien visualiser les robots, leurs mouvements, les différentes ressources avec des couleurs et des sprites.
- **Architecture ECS**: Bevy utilise l'architecture Entity Component System qui sera bien adapté pour décomposer notre projet en différents modules.
- **Simplicité de visualisation**: Dans le terminal, nous sommes obligés de rafraichir la carte, ce qui n'est pas pratique, l'utilisation de Bevy permettra d'effectuer la simulation en temps réel.

### Inconvénients de Bevy
- **Temps de développement**: Etant un moteur 2D/3D, la simulation prendra plus de temps à développer. Au lieu du Rust natif, nous utilisons un moteur encore très jeune avec une documentation pas toujours à jour.
- **Exigences système**: L'utilisation de Bevy prend plus de ressources et il faut également une carte graphique comme nous avons pu le constater.

## Conséquences
En utilisant Bevy, nous devons nous préparer bien consulter la documentation ou les différents forums. Nous devrons également utiliser des sprites pour la visualisation.
Cependant, cette décision nous permettra de créer une application bien plus plus belle et interactive de la simulation.