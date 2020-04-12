# Compresseur de Dictionnaire de Mots

Aucune dépendance nécessaire.

Prend un fichier texte sans sauts de ligne en entrée pour construire un arbre/tas optimisé.
C'est une base simple, mais performante pour un système d'auto complétion.
Cette version n'étant réalisée qu'à titre ludique, elle n'a pas vocation à être prête à l'emploi en prodution.

Le script indique le nombre de caractères reçus, de mots, et le nombre de noeuds créés.
Donnant ainsi une indication sur la compression du texte fourni.
Il chronomètre également le temps d'exécution.

La plupart des listes de mots utilisant le saut de ligne comme séparateur, le script preformatter_txt.py converti les sauts de ligne en espaces. Ainsi, les listes sont traités comme un paragraphe de texte lambda.

Le fichier texte.txt est un texte Lorem Ipsum généré par : https://fr.lipsum.com/

Le fichier dico_fr.txt est un fichier reformaté de : http://www.pallier.org/liste-de-mots-francais.html

Le fichier dico_en.txt est un fichier reformaté de : https://github.com/dwyl/english-words

**Version CPython** : Interprété avec Python 2.7.17 et 3.6.9. Testé avec PyPy 2.7

**Version Rust** : Compilé avec Rust 1.40.

**Version PHP** : Interprété avec PHP 7.2.24

**Version C++** : Compilé avec G++ 7.5.0

**Version JS** : Interprété avec Node 12.16.1

## Benchmark PHP vs Python2 vs PyPy vs Python3 vs Node JS vs C++ vs Rust :
Les versions autre que Python n'ont été crée que dans le but d'évaluer grossièrement la différence de performances entre les langages.
Les codes ont été exécutés sur la même machine (PC portable Ubuntu 18.04, SSD, i7 8e génération, 8Go ram).
Le fichier dico_en.txt est le fichier texte utilisé pour le benchmark.
Les temps d'exécution fournis sont donné à titre de comparaison, mais peuvent varier d'une exécution à l'autre, et différer sur un autre système/configuration. Enfin, le code n'est probablement pas au mieux adapté/optimisé à chaque langage.

**NB** : Les caractères composés en UTF-8 posent des difficultés sur les slices notamment en C++ et en Rust. C'est le cas pour les caractères accentués notamment. C++ crée un noeud pour l'accent. Rust n'est pas en mesure de traiter un accent seul et ne traite donc pas le reste du mot. Des adaptations du code supplémentaires sont nécessaires. Testé avec le fichier dico_fr.txt.

Le temps d'exécution est en seconde. Les plus performants ont un temps d'exécution plus court. (moins c'est mieux)

### Version PHP 7.2.4 :
  - 26.722s (php-cli)

### Version CPython 2.7.17 : 
  - 50.187s de temps d'exécution
  - 45.365s (gc.disable)

### Version PyPy 2.7 :
  - 12.602s

### Version CPython 3.6 :
  - 18.504s
  - 15.802s (gc.disable)

### Version Node JS 12.16.1 :
  - 17.729s (node)

### Version C++ :
  - 8.903s (g++ -o)

### Version Rust 1.40 : 
  - 10.836s (cargo run)
  - 7.881s (cargo build --release)


Sans surprise les langages compilés dominent. Rust brille particulièrement face au C++ (-11%).

Je suis surpris de voir Python 2.7 aussi lent, il est souvent évoqué comme plus rapide que Python3 sur la toile. De même PHP souffre beaucoup ici (+44% par rappor à Python3). Sur de petits scripts, PHP est souvent plus rapide que Python3.

La performance tant vantée de Node sur la toile ne ressort pas franchement sur ce test (-4% par rapport à Python3 avec gc, et +10% par rapport à Python 3 en optimisant le gc). Node est beaucoup moins performant que PyPy (+40% par rapport à Pypy).
