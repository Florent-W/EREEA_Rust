# ADR 0003: Utilisation d'`enum` et `struct` pour les Types de Robots

## Status
**Validé**

## Contexte
Dans le développement de notre système de robots, nous avons besoin de représenter différents types de robots, chacun avec ses propres caractéristiques et comportements. Nous avons réfléchi à deux approches : 
- Utiliser des traits et des implémentations spécifiques pour chaque type de robot
- Utiliser une structure avec une énumération pour les types. 

## Décision
Nous avons décidé d'utiliser une combinaison de `enum` pour définir les types de robots et une `struct` pour représenter un robot, au lieu d'utiliser des traits et des implémentations spécifiques pour chaque type de robot.

## Raisonnement

### Avantages de l'utilisation d'`enum` et `struct`
- **Simplicité de la structure de données** : L'utilisation d'une `enum` pour les types de robots simplifie la définition des différents types, en regroupant tous les types possibles en un seul endroit, facilitant leur gestion et leur extension.
- **Facilité d'intégration avec ECS** : En ECS, utiliser une `struct` pour les robots permet d'intégrer facilement ces entités dans l'architecture ECS, avec les types de robots facilement modifiables en tant que composant.
- **Maintenabilité** : Cette approche réduit la complexité en évitant la prolifération de traits et de classes pour chaque nouveau type de robot.
- **Gain de temps** : Utiliser un struct pour les robots permet de gagner du temps pour l'implémenter. 

### Inconvénients de l'utilisation d'`enum` et `struct`
- **Risque de surcharge dans `enum`** : Si les différences de comportement entre les types de robots deviennent trop complexes, l'énumération peut devenir surchargée de cas spéciaux, ce qui pourrait nuire à la clarté du code. Nous n\'avons cependant pas prévu énormément de type de robots différents pour l'instant.'
- **A l\'utilisation** : Chaque fonction utilisée seulement par certains types de robots devront vérifier le type du robot avant de se lancer. 

## Conséquences
En adoptant une structuration basée sur `enum` et `struct`, cette décision devrait être révisée si le nombre de types de robots ou la complexité de leurs comportements augmente significativement.
