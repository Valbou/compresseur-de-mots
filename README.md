# Compresseur de Dictionnaire de Mots

Aucune dépendance nécessaire.

**Version CPython** : Interprété avec Python 2.7.17 et 3.6.9. Testé avec PyPy 2.7

**Version Rust** : Compilé avec Rust 1.40.

**Version PHP** : Interprété avec PHP 7.2.24

**Version C++** : Compilé avec G++ 7.5.0

Prend un fichier texte sans sauts de ligne en entrée pour construire un arbre/tas optimisé.
C'est une base simple, mais performante pour un système d'auto complétion.
Cette version n'étant réalisée qu'à titre ludique, elle n'a pas vocation à être prête à l'emploi en prodution.

Le script indique le nombre de caractères reçus, et le nombre de noeuds créés.
Donnant ainsi une indication sur la compression du texte fourni.

Le fichier texte.txt est un texte Lorem Ipsum généré par : https://fr.lipsum.com/

Le fichier dico_fr.txt est un fichier reformaté de : http://www.pallier.org/liste-de-mots-francais.html

## Benchmark Python2 vs PyPy vs Python3 vs Rust vs PHP vs C++ :
Les versions autre que Python n'ont été crée que dans le but d'évaluer grossièrement la différence de performances entre les langages.
Les codes ont été exécutés sur la même machine (PC portable Ubuntu 18.04, SSD, i7 8e génération, 8Go ram).
Le fichier dico_fr.txt est le fichier texte utilisé pour le benchmark.
Les temps d'exécution fournis sont donné à titre de comparaison, mais peuvent varier d'une exécution à l'autre, et différer sur un autre système/configuration. Enfin, le code n'est probablement pas au mieux adapté/optimisé à chaque langage.

**NB** : Certains langages ne prennent pas en charge nativement l'UTF-8 ce qui ajoute un biais au code testé générant des noeuds supplémentaires pour les accents notamment.

### Version CPython 2.7.17 : 
  - environ 22s de temps d'exécution

### Version PyPy 2.7 :
  - environ 6.4s
  
### Version CPython 3.6.9 : 
  - environ 10s
  
### Version Rust 1.40 : 
  - environ 4.5s (cargo run)
  - environ 3.2s (cargo build --release)

### Version PHP 7.2.4 :
  - environ 10s (php-cli)

### Version C++ :
  - environ 4.5s (g++ -o)
