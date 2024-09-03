use sha2::{Sha256, Digest};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {

    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // si l'utilisateur n'a pas rentré d'arguments on affiche un message d'erreur
    if args.len() < 2 {
        eprintln!("Usage: {} <data> [difficulty]", args[0]);
        std::process::exit(1);
    }

    // Récupérer la data et la difficulté depuis les arguments
    let data =  &args[1];
    let difficulty: usize = if args.len() > 2 {
    args[2].parse().unwrap_or_else(|_| {
        eprintln!("La difficulté doit être un entier positif");
        std::process::exit(1);
        })
    } else {
        2
    };
    let _difficulty_prefix = "0".repeat(difficulty);
    // définir le nonce à 0 pour commencer la recherche
    let mut nonce = 0;

    // lancement de la boucle pour trouver le nonce
    loop {
        // calculer le timestamp actuel
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)
            .expect("Le temps qui s'est écoulé depuis l'epoch UNIX")
            .as_secs();

        // Préparer les données à hacher
        let input = format!("{}{}{}", data, timestamp, nonce);

        // Calculer le hash
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash_result = hasher.finalize();

        // convertir le hash en une chaîne hexadécimale
        let mut hash_hex = String::new();
        for byte in hash_result.iter() {
            write!(&mut hash_hex, "{:02x}", byte).unwrap();
        }

        // vérifier si le hash est inférieur au target
        if hash_hex.starts_with(&_difficulty_prefix) {
            println!("Bloc miné !");
            println!("Difficulté: {}", difficulty);
            println!("Timestamp: {}", timestamp);
            println!("Nonce trouvé: {}", nonce);
            println!("Hash correspondant: {}", hash_hex);
            break;
        }

        nonce += 1;
    }
}
