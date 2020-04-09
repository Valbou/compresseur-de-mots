var fs = require('fs');

// node -e 'require("./arbre").init()'

class Noeud {
  constructor(mot) {
    this.val = mot[0];
    this.enfants = [];
    if(mot.length > 1) {
      this.ajoutNoeud(mot)
    }
    console.log("Ajout Noeud ("+ this.val +") "+mot)
  }

  ajoutNoeud(mot) {
    if(mot.length > 1) {
      mot = mot.slice(1)
      let indice = this.noeudEnfantExiste(mot)
      if(indice >= 0) {
        this.enfants[indice].ajoutNoeud(mot)
        return true
      }
      this.enfants[this.enfants.length] = new Noeud(mot)
    }
    return true
  }

  noeudEnfantExiste(mot) {
    let c = mot[0]
    let taille = this.enfants.length
    for(let i=0; i<taille; i++) {
      if(c == this.enfants[i].val) {
        return i
      }
    }
    return -1
  }

  decompte() {
    let total = 0
    let taille = this.enfants.length
    for(let i=0; i<taille; i++) {
        total += this.enfants[i].decompte()
    }
    total += taille
    return total
  }
}

function texteVersMots(texte) {
  return texte.split(" ")
}

function comptageNoeuds(mots) {
  let arbre = new Noeud(' ')
  let taille = mots.length
  for(let i=0; i<taille; i++) {
    console.log("-> "+mots[i])
    arbre.ajoutNoeud(" "+mots[i])
  }
  return arbre.decompte()
}

function script(texte) {
  let liste = texteVersMots(texte)
  let nbNoeuds = comptageNoeuds(liste)
  console.log("### Bench JS Arbre ###")
  console.log("Texte : "+ texte.length +" caractères, "+ liste.length +" mots, "+ nbNoeuds +" noeuds")
  return nbNoeuds
}

function main () {
  let debut = Date.now()
  let repo = "/home/user/"
  let texte = fs.readFileSync(repo+"dico_fr.txt", 'utf8')
  script(texte)
  let fin = Date.now()
  console.log("Temps : "+ new Intl.NumberFormat('fr-FR', { maximumFractionDigits: 2}).format(fin-debut) +"s")
}

main()

