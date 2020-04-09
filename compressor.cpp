#include <iostream>
#include <vector>
#include <string>
#include <sys/time.h>
#include <fstream>
#include <sstream>

using namespace std;

class Noeud {
    private:
        char val;
        vector<Noeud> enfants;

    public:
        Noeud(string mot) {
            this->val = mot[0];
            if(mot.length() > 1) {
                this->ajoutNoeud(mot);
            }
            cout << "Ajout Noeud (" << this->val << ") " << mot << endl;
        }

        bool ajoutNoeud(string mot) {
            mot = mot.substr(1);
            if(mot.length() > 1) {
                int indice = this->noeudEnfantExiste(mot);

                if(indice >= 0) {
                    this->enfants[indice].ajoutNoeud(mot);
                    return true;
                }
                this->enfants.push_back(Noeud(mot));
            }
            return true;
        }

        int noeudEnfantExiste(string mot) {
            char c = mot[0];
            int taille = this->enfants.size();
            for(int i=0; i < taille; i++) {
                if(this->enfants[i].val == c) {
                    return i;
                }
            }
            return -1;
        }

        int decompte(void) {
            int total = 0;
            int taille = this->enfants.size();
            for(int i=0; i < taille; i++) {
                total += this->enfants[i].decompte();
            }
            total += taille;
            return total;
        }
};

vector<string> texteVersMots(string texte) {
    cout << "Préparation" << endl;
    string temp; 
    stringstream flux(texte);
    vector<string> mots;
    while(getline(flux, temp, ' ')){
        mots.push_back(temp);
    }
    return mots;
}

int comptageNoeuds(vector<string> mots) {
	Noeud arbre = Noeud(" ");
    int taille = mots.size();
	for(int i=0; i < taille; i++) {
		cout << "-> " << mots[i] << endl;
		arbre.ajoutNoeud(" "+mots[i]);
	}
	return arbre.decompte();
}

int script(string texte) {
	vector<string> listeMots = texteVersMots(texte);
	int nbrNoeuds = comptageNoeuds(listeMots);
	cout << "### Bench C++ Arbre ###" << endl;
	cout << "Texte : " << texte.length() << " caractères, " << listeMots.size() << " mots, " << nbrNoeuds << " noeuds" << endl;
	return nbrNoeuds;
}

int main() {
    struct timeval time;
    gettimeofday(&time, NULL);
    long depart = (( long long)time.tv_sec * 1000000) + time.tv_usec;

	string repo = "/home/user/";
    fstream fichier;
    fichier.open(repo+"dico_fr.txt", ios::in);
    string texte;
    if(fichier.is_open()) {
        getline(fichier, texte);
        fichier.close();
    }

	script(texte);

    gettimeofday(&time, NULL);
    long fin = (( long long)time.tv_sec * 1000000) + time.tv_usec;

    cout.precision(2);
	cout << "Temps : " << (fin-depart)/1000 << "ms" << endl;

    return 0;
}
