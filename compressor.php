<?php
    class Noeud {
        private $val = "";
        private $enfants = array();

        public function __construct($mot) {
            $this->val = $mot[0];
            if(strlen($mot) > 1) {
                $this->ajoutNoeud($mot);
            }
            echo "Ajout Noeud ({$this->val}) $mot\n";
        }

        public function ajoutNoeud($mot) {
            if(1 < strlen($mot)) {
                $mot = substr($mot, 1);
                $indice = $this->noeudEnfantExiste($mot);
                if($indice >= 0) {
                    $this->enfants[$indice]->ajoutNoeud($mot);
                    return TRUE;
                }
                $this->enfants[] = new Noeud($mot);
            }
            return FALSE;
        }

        public function noeudEnfantExiste($mot) {
            foreach($this->enfants as $k => $n) {
                if($n->val == $mot[0]) {
                    return $k;
                }
            }
            return -1;
        }

        public function decompte() {
            $total = 0;
            foreach($this->enfants as $k => $n) {
                $total += $n->decompte();
            }
            $total += count($this->enfants);
            return $total;
        }
    }

    function texteVersMots($texte) {
        echo "Préparation\n";
        return explode(' ', $texte);
    }

    function comptageNoeuds($mots) {
        $arbre = new Noeud(' ');
        foreach($mots as $mot) {
            echo "-> $mot\n";
            $arbre->ajoutNoeud(" ".$mot);
        }
        return $arbre->decompte();
    }

    function script($texte) {
        $listeMots = texteVersMots($texte);
        $nbrNoeuds = comptageNoeuds($listeMots);
        echo "### Bench PHP Arbre ###\n";
        echo "Texte : ".strlen($texte)." caractères, ".count($listeMots)." mots, ".$nbrNoeuds." noeuds\n";
    }

    $debut = microtime(TRUE);

    $repo = "/home/user/";
    $texte = file_get_contents($repo."dico_fr.txt");
    script($texte);

    $fin = microtime(TRUE);
    $delai = $fin - $debut;
    echo "Temps : ".number_format($delai*1000, 2)."ms\n";
