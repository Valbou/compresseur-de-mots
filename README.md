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

**Version CPython** : Interprété avec Python 2.7.17, 3.6.9 et 3.8.5. Testé avec PyPy 2.7 et PyPy 3.6. Testé avec le compilateur pour Python Nuitka

**Version Rust** : Compilé avec Rust (Rustc 1.40 et 1.46).

**Version PHP** : Interprété avec PHP 7.2.4, 7.4.3 et 8.0.3, 

**Version C++** : Compilé avec G++ 7.5.0 et 9.3.0

**Version JS** : Interprété avec Node 12.16.1 ainsi que Deno 1.0 et 1.8.3

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

### Nuitka 0.6.14.4 / CPython 3.8.5 :
  - 18.282s
  - 14.759s (gc.disable)

### Version Node JS 12.16.1 :
  - 17.729s (node)

### Version CPython 3 :
  - v3.6.9
    - 18.504s
    - 15.802s (gc.disable)
  - v3.8.5 :
    - 16.889s
    - 13.237s (gc.disable)

### Version Deno :
  - v1.0 :
    - 11.035s (deno run --unstable --allow-read)
  - v1.8.3 :
    - 10.643s (deno run --allow-read)

### Version PyPy :
  - v2.7 :
    - 12.602s
  - v3.6
    - 10.411s

### Version PHP :
  - v7.2.4 :
    - 26.722s (php-cli)
    - 10.246s (gc_disable)
  - v7.4.3 :
    - 10.053s (php-cli)
    - 8.869s (gc_disable)
  - v8.0.3 :
    - 9.670s (php-cli)
    - 8.109s (gc_disable)

### Version C++ :
  - g++ v7.5.0 :
    - 8.903s (g++ -o)
  - g++ v9.3.0 :
    - 7.742s (g++ -o)

### Version Rust :
  - v1.40 :
    - 10.836s (cargo run)
    - 7.881s (cargo build --release)
  - v1.46 :
    - 9.568s (cargo run)
    - 6.440s (cargo build --release)

Je vous laisse juger des performances. Il est important de noter toutefois que ces tests ne sont pas représentatifs de cas d'usage courants.

**Attention, la désactivation du GC, peut entrainer des fuites de mémoire**
