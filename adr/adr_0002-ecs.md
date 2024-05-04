# ADR 000X: Utilisation de l'Architecture ECS dans le Projet

## Status
**Accepté**

## Contexte
Notre projet nécessite la gestion efficace de nombreuses entités et interactions complexes, ce qui est typique dans les simulations interactives. Pour répondre à ce besoin, nous avons choisi une architecture basée sur le système Entity Component System. Cette architecture est mise en œuvre à travers les dossiers `components` et `systems`.

## Décision
Nous avons décidé d'adopter et de maintenir une architecture ECS pour le développement de la simulation.

## Raisonnement

### Avantages de l'ECS
- **Modularité**: L'ECS permet une séparation claire entre la logique de traitement (systèmes) et les données des entités (composants), favorisant ainsi un code plus modulaire.
- **Performance**: Cette architecture est bien connue pour sa capacité à faciliter le traitement parallèle et à optimiser les performances qui pourra éventuellement gérer le rôle des robots indépendamment.
- **Maintenabilité**: Le découplage entre les données et les comportements simplifie l'évolution du code et permettra d'éviter les conflits.

### Inconvénients de l'ECS
- **Complexité de conception**: La mise en place initiale peut être plus complexe comparée aux architectures traditionnelles, surtout que nous ne partons pas de zéro puisque nous avons déjà commencé à développer l'application.

### Fichiers et Modules Concernés
- **Composants (`components`)**: Contient des définitions de composants tels que `audio`, `map`, `resource`, `robot`, `ui`.
- **Systèmes (`systems`)**: Implémente la logique opérationnelle des systèmes dans des fichiers comme `camera`, `movement`, `ui`, `utilities`.

## Conséquences
L'adoption de l'ECS prendra un peu de temps et nécessitera une refonte du code. A terme, elle permettra d'améliorer les performances, la clarté et la maintenabilité du code, ce qui permettra de gagner du temps.
