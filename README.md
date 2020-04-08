# compresseur-de-mots

Aucune dépendance nécessaire.

**Version Python** : Testé avec Python 2.7.17 et 3.6.9.

**Version Rust** : Compilé avec Rust 1.40.

Prend un fichier texte sans sauts de ligne en entrée pour construire un arbre/tas optimisé.
C'est une base simple, mais performante pour un système d'auto complétion.
Cette version n'étant réalisée qu'à titre ludique, elle n'a pas vocation à être prête à l'emploi en prodution.

Le script indique le nombre de caractères reçus, et le nombre de noeuds créés.
Donnant ainsi une indication sur la compression du texte fourni.

## Benchmark Python vs Rust :
La version Rust n'a été crée que dans le but d'évaluer grossièrement la différence de performances entre les deux langages.
Les deux codes ont été exécutés sur la même machine (PC portable Ubuntu 18.04, SSD, i7 8e génération, 8Go ram).
Le fichier texte.txt est le fichier texte utilisé pour le benchmark (Lorem Ipsum).

### Version Python 2.7.17 : 
  - 100ms à 140ms de temps d'exécution
  
### Version Python 3.6.9 : 
  - 48ms à 52ms de temps d'exécution (venv)
  
### Version Rust 1.40 : 
  - 30ms à 35ms (cargo run)
  - 18ms à 25ms (cargo build --release)
