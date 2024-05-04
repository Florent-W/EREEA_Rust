# Changelog

## 0.1
- Démarrage du projet
- Constructions des robots, de la carte, des stuctures et d'énumérations
- La simulation se lance dans la console

## 0.2
- Ajout des obstacles, des ressources sur la carte
- Utilisation du perlin noise pour génerer la carte aléatoirement

## 0.2.1
- Ajout de la base où apparaitront les robots et qui permettra de synchroniser les informations obtenues par les robots
- Spawn des robots dans la base

## 0.3
- Utilisation de Bevy pour afficher la simulation dans une fenêtre et pour avoir une interface graphique
- Ajout de sprites pour afficher les ressources, obstacles, la base et les robots

## 0.3.1
- Construction des robots améliorés et selon leurs types
- Changement du nom de la fenêtre

## 0.4
- Ajout de nouveaux sprites
- Ajout d'un état pour les cases découvertes / non découvertes
- Vitesse différente selon le type de robot, mouvement des robots pour se déplacer sur les cases

## 0.5
- Ajout de différents états pour les robots
- Demande de seed au démarrage
- Cible aléatoire à aller pour les robots
- Toutes les cases non découverte ne sont pas affichés
- Mode découverte pour voir toutes les cases même si non découvertes

## 0.6
- Changement des sprites
- Spawn des robots à la base

## 0.7
- Résolution changée
- Activation du mode plein écran
- Les robots regardent si une position n'a pas été prise avant de l'assigner en cible

## 0.8
- Correction du déplacement des robots pour éviter les obstacles
- Au démarrage, la caméra est centrée sur la base
- Demande de la résolution à l'utilisateur

## 0.9
- Compteurs d'énergie et de minerai
- Affichage des compteurs
- Correction d'un bug lors de la sélection des robots
- Ajout de la vitesse de la simulation
- Affichage de la vitesse
- Sprites retirés pour les zones géographiques pour gagner en performance
- Légende ajoutée

## 0.9.1
- Déplacement de la demande du nombre de robot au démarrage
- Possibilité de quitter la simulation

## 0.10
- Changement de l'architecture du projet avec la mise en place de différents modules pour plus de modularités

## 0.11
- Tests unitaires
- Taille de la carte demandée

## 1.0
- Possibilité d'activer les bordures
- Création de robot automatique avec l'utilisation des ressources
- Ajout de l'élément montagne
- Correction du placement de la base
- Les élements géographiques peuvent se mettent sur une case ou il y a déjà une ressource
- Ajout d'une musique de fond et de bruitage
- Ajout de tests unitaires
- Ajout d'ADR
- Correction des cases découvertes par tous les robots pas encore revenu au lieu du robot qui a été sur la case et vient de revenir 
- Sortie du projet