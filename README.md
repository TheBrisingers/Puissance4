# Puissance4

### Règles du jeu

“Le but du jeu est d'aligner 4 pions sur une grille comptant 6 rangées et 7 colonnes. 
Chaque joueur dispose de 21 pions d'une couleur (par convention, en général jaune ou rouge). 
Tour à tour les deux joueurs placent un pion dans la colonne de leur choix, le pion coulisse 
alors jusqu'à la position la plus basse possible dans ladite colonne à la suite de quoi 
c'est à l'adversaire de jouer. Le vainqueur est le joueur qui réalise le premier un 
alignement (horizontal, vertical ou diagonal) d'au moins quatre pions de sa couleur. 
Si, alors que toutes les cases de la grille de jeu sont remplies, aucun des deux joueurs 
n'a réalisé un tel alignement, la partie est déclarée nulle.” (Wikipédia)

### Démarrer une nouvelle partie 

Pour démarrer il faut faire un cargo run dans le dossier puissance4_server et deux fois cargo run dans le dossier puissance4_client pour avoir 2 clients qui se connectent.

Une partie nécessite la mise en place d’une grille de jeu de 7 colonnes par 6 lignes générée sur la partie serveur.
Et de deux clients connectés, qui représentent les joueurs. 
Les utilisateurs doivent pouvoir choisir la colonne de leur choix à chaque avec A Z E R T Y U. 

Nous représentons la partie comme une succession de coups, qui pourrons être effectués 
tantôt par un joueur (client 1), tantôt par l’autre(client 2). Etant donné que lorsqu’un joueur 
joue un pion, celui-ci coulisse jusqu’à la position la plus basse dans une colonne, 
jouer un coup, ou placer un pion, revient à choisir une colonne comme dit plus tôt.

### Mettre à jour la grille de jeu

A chaque action du joueur ou de l’adversaire, une position de la grille est modifiée. 
On attribut à chaque position une valeur pour définir si elle a été jouée par le joueur.


### Détecter la fin de la partie

Une partie de puissance quatre se termine soit quand un des deux joueurs a réalisé 
un alignement de quatre pions concomitants soit quand toutes les positions de la 
grille ont été jouées sans vainqueur.