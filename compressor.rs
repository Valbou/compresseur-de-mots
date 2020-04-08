use std::str::FromStr;
use std::time::Instant;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn premier_caractere(mot: &str) -> char {
	match mot.get(..1) {
		Some(v) => char::from_str(v).unwrap(),
		None => char::from_str(" ").unwrap(),
	}
}

struct Noeud {
	val: char,
	enfants: Vec<Noeud>
}

impl Noeud {
	fn new(mot: &str) -> Noeud {
		let c = premier_caractere(mot);
		let mut n = Noeud{val:c, enfants:vec!()};
		if mot.len() > 1 {
			n.ajout_noeud(mot);
		}
		println!("Ajout Noeud ({}) {}", n.val, mot);

		n
	}

	fn ajout_noeud(&mut self, mot: &str) -> bool {
		if 1 < mot.len() {
			let mot2 = match mot.get(1..) {
				Some(v) => v,
				None => "",
			};

			let indice: isize = self.noeud_enfant_existe(mot2);
			if indice >= 0 {
				self.enfants[indice as usize].ajout_noeud(mot2);
				return true;
			}

			self.enfants.push(Noeud::new(mot2));
		}

		true
	}

	fn noeud_enfant_existe(&self, mot: &str) -> isize {
		let c = premier_caractere(mot);
		let mut i: isize = 0;
		for n in &self.enfants {
			if n.val == c {
				return i
			}
			i += 1;
		}

		-1
	}

	fn decompte(&self) -> usize {
		let mut total: usize = 0;
		for n in &self.enfants {
			total += n.decompte();
		}
		total += self.enfants.len();

		total
	}
}

fn texte_vers_mots(texte: &str) -> Vec<&str> {
	println!("Préparation");

	texte.split(' ').collect()
}

fn comptage_noeuds(mots: Vec<&str>) -> usize {
	let mut arbre = Noeud::new(" ");
	for mot in mots {
		println!("-> {}", mot);
		let concat = &format!("{}{}", " ", mot.to_lowercase())[..];
		arbre.ajout_noeud(concat);
	}

	arbre.decompte()
}

fn script(texte: String) -> usize {
	let liste_mots = texte_vers_mots(&texte[..]);
	let taille = liste_mots.len();
	let nbr_noeuds = comptage_noeuds(liste_mots);
	println!("### Bench Rust Arbre ###");
	println!("Texte : {} caractères, {} mots, {} noeuds", texte.len(), taille, nbr_noeuds);

	nbr_noeuds
}

fn main() -> io::Result<()> {
    let duree = Instant::now();

	let repo = "/home/user/";
	let chemin = &format!("{}{}", repo, "texte.txt")[..];

	let mut fichier = File::open(chemin)?;
	let mut texte = String::new();

    	fichier.read_to_string(&mut texte)?;
	script(texte);

	println!("Temps : {:.2}ms", (duree.elapsed().subsec_nanos() as f64) / 1000000f64);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_texte_0() {
		let texte = "mot".to_string();
		assert_eq!(script(texte), 3);
	}

	#[test]
	fn test_texte_1() {
		let texte = "mot mot mot moka".to_string();
		assert_eq!(script(texte), 5);
	}

	#[test]
	fn test_texte_2() {
		let texte = "un texte simple sans ponctuation ni majuscule".to_string();
		assert_eq!(script(texte), 38);
	}

	#[test]
	fn test_texte_3() {
		let texte = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent pellentesque sapien neque, id sodales enim maximus eu. Maecenas vitae orci eros. Nam euismod volutpat sem, sit amet condimentum enim faucibus ut. Nam nec arcu ut nisl porta consequat. Curabitur ut pulvinar nisl, iaculis fermentum magna. Praesent convallis bibendum urna et venenatis. Maecenas efficitur justo vitae velit eleifend elementum. Vivamus eget viverra massa, vitae commodo neque.".to_string();
		assert_eq!(script(texte), 280);
	}

	#[test]
	fn test_texte_4() {
		let texte = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla faucibus magna eleifend mi blandit, ac iaculis odio dignissim. In hac habitasse platea dictumst. Quisque mi tellus, lacinia eget sollicitudin id, dignissim eu lorem. Nam et ante orci. Aenean elementum, tortor vitae fermentum tempor, magna dui faucibus nisl, sit amet pretium est magna venenatis purus. Quisque condimentum lobortis sollicitudin. Vestibulum ac est vitae quam mattis luctus. Integer orci risus, dictum ut vehicula in, aliquet eget purus. Nam ut nisl malesuada, venenatis tortor ac, dignissim lacus. Maecenas dolor odio, molestie vitae interdum sit amet, luctus eget quam. Nam a pharetra nibh, dictum blandit lorem. Donec scelerisque nunc eget odio consequat vehicula. Nunc vitae ullamcorper sapien. Aenean at nibh enim. Suspendisse potenti. Maecenas consectetur nec nisl facilisis congue. Maecenas commodo magna lorem, sed venenatis ligula rutrum sit amet. Donec condimentum eleifend egestas. Morbi interdum condimentum placerat. Ut metus ligula, iaculis ac lobortis pellentesque, egestas vehicula sem. Donec congue orci vel vehicula mollis. Sed euismod, lacus at accumsan egestas, orci lorem semper turpis, sit amet blandit sapien magna non mauris. Vivamus facilisis purus felis, ac lacinia tortor malesuada sit amet. Nullam commodo lorem quis justo malesuada tincidunt. Suspendisse ac tellus sem. In et felis ut ligula finibus sodales. Nunc a malesuada nisl. Mauris feugiat quis urna et bibendum. Integer aliquam lacus massa, nec egestas tellus venenatis vel. Donec nunc orci, sollicitudin rhoncus felis blandit, consequat malesuada lectus. Sed tincidunt sit amet dui tincidunt porttitor. Duis iaculis massa ac fermentum ornare. Sed venenatis mi eu imperdiet aliquet. Nam in arcu dolor. Nulla blandit pellentesque ligula, quis pulvinar nibh posuere quis. Curabitur ut urna sed diam elementum tempor non efficitur nisi. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. In a libero tempor, aliquet tortor vitae, viverra lacus. Sed orci lorem, eleifend eget felis eleifend, aliquet iaculis enim. Vivamus metus diam, mattis convallis feugiat et, pulvinar non mauris. Integer volutpat rhoncus urna quis tempus. Cras urna justo, mattis id est in, venenatis bibendum ex. Curabitur sit amet leo ut lorem faucibus viverra. Sed libero augue, consequat a ligula vitae, rhoncus bibendum massa. Duis et dictum turpis. Aenean pulvinar feugiat metus, ac ultricies metus. Donec vitae enim dui. Donec a posuere sem. Cras vulputate odio vitae leo ultricies, nec vehicula nunc dapibus. Pellentesque sagittis, erat ac venenatis pretium, dolor est consequat diam, et elementum velit ex vel massa. Ut sed tempus dolor. Sed nec nibh nec felis tincidunt venenatis aliquet sagittis est. Proin urna est, venenatis eget fringilla quis, condimentum ullamcorper risus. Aenean ornare est ac urna ultricies volutpat. Praesent luctus elit sed nunc fringilla fringilla. Etiam maximus sodales erat non tristique. Etiam rutrum massa dolor, non sodales lectus rutrum sit amet. Etiam sollicitudin at lacus sed aliquet. Curabitur semper, metus id lobortis rhoncus, sem est facilisis nisl, quis dapibus lacus ante eu tellus. Quisque ultrices interdum odio, at pulvinar ipsum egestas vehicula. Nunc et mauris nulla. Duis et viverra nibh. Duis in lorem quam. Etiam vitae lectus sit amet felis facilisis condimentum ut eu ex. Nulla nec purus non odio elementum tempor vitae vel quam. Proin quam felis, eleifend nec lorem ut, pulvinar fermentum est. Maecenas vel tortor vel quam pulvinar rhoncus. Quisque id felis ut eros varius finibus non id eros. Vestibulum vulputate bibendum congue. Pellentesque ullamcorper non nunc in cursus. Ut imperdiet sem eros, eget feugiat urna volutpat ac. Curabitur maximus ultricies metus, vitae finibus justo finibus sit amet. Sed gravida suscipit finibus. Nulla ultrices dictum enim, in tempus dui facilisis a. Duis iaculis est faucibus elit sollicitudin, in hendrerit sapien auctor. Aliquam in quam ac eros sodales sollicitudin. Suspendisse elementum, orci nec efficitur consectetur, erat urna condimentum risus, quis tempor libero ante sit amet est. Ut rhoncus odio ac sem blandit, nec fringilla risus dignissim. Praesent et libero nec neque sollicitudin venenatis vitae faucibus dui. Nulla ullamcorper aliquam nibh, at suscipit sem congue malesuada. Aliquam metus risus, lobortis id sem non, sagittis tristique libero. In venenatis tellus id magna ornare tempor. Integer urna dui, fermentum et tincidunt euismod, ullamcorper vel metus. Sed ornare mi tristique, faucibus nunc eu, consectetur nisi. Aenean vel sem eget ligula varius sollicitudin. Aliquam varius est elementum mauris mattis laoreet. Sed varius vehicula tortor consequat ullamcorper. Nam ex velit, semper sed quam quis, ultrices ultrices mauris. Duis et mi vel tellus posuere pellentesque id id sapien. Quisque facilisis tristique velit, vitae pharetra arcu mollis vitae. Mauris sed rhoncus ipsum. Donec a nibh pharetra, sagittis urna sollicitudin, semper felis. Aenean quis eleifend ante. Phasellus vitae ipsum urna. Nulla eget facilisis mauris. Vivamus laoreet eget ante cursus tempus. In felis magna, auctor vitae justo eget, pulvinar dictum arcu. Donec porttitor quis nunc et rhoncus.".to_string();
		assert_eq!(script(texte), 880);
	}
}
