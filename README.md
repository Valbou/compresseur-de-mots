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

**Version CPython** : Interprété avec Python 2.7.17, 3.6.9 et 3.8.5. Testé avec PyPy 2.7

**Version Rust** : Compilé avec Rust 1.40.

**Version PHP** : Interprété avec PHP 7.2.24

**Version C++** : Compilé avec G++ 7.5.0

**Version JS** : Interprété avec Node 12.16.1 et Deno 1.0

## Benchmark PHP vs Python2 vs PyPy vs Python3 vs Node JS vs Deno vs C++ vs Rust :
Les versions autre que Python n'ont été crée que dans le but d'évaluer grossièrement la différence de performances entre les langages.
Les codes ont été exécutés sur la même machine (PC portable Ubuntu 18.04, SSD Ext4, i7 8e génération, 8Go ram).
Le fichier dico_en.txt est le fichier texte utilisé pour le benchmark.
Les temps d'exécution fournis sont donnés à titre de comparaison, mais peuvent varier d'une exécution à l'autre, et différer sur un autre système/configuration. Enfin, le code n'est probablement pas au mieux adapté/optimisé à chaque langage.

Le temps d'exécution est en seconde. Les plus performants ont un temps d'exécution plus court. (moins c'est mieux)
Les tests sont classés du plus lent au plus rapide.

### Version CPython 2.7.17 : 
  - 50.187s de temps d'exécution
  - 45.365s (gc.disable)

### Version PHP 7.2.4 :
  - 26.722s (php-cli)
  - 10.246s (gc_disable)

### Version CPython 3.6 :
  - 18.504s
  - 15.802s (gc.disable)

### Version Node JS 12.16.1 :
  - 17.729s (node)

### Version CPython 3.8 :
  - 16.998s
  - 13.237s (gc.disable)

### Version PyPy 2.7 :
  - 12.602s

### Version Deno 1.0 :
  - 11.035s (deno run --unstable --allow-read)

### Version C++ :
  - 8.903s (g++ -o)

### Version Rust 1.40 : 
  - 10.836s (cargo run)
  - 7.881s (cargo build --release)

Je vous laisse juger des performances. Il est important de noter toutefois que ces tests ne sont pas représentatifs de cas d'usage courants.

**Attention, la désactivation du GC, peut entrainer des fuites de mémoire**
