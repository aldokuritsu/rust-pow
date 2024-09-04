use sha2::{Sha256, Digest}; // Importation du module de hachage SHA-256
use std::fmt::Write; // Pour formater la sortie en hexadécimal
use std::time::{SystemTime, Duration, UNIX_EPOCH}; // Manipulation du temps
use std::env; // Récupération des arguments en ligne de commande

// Durée d'expiration pour un hash valide (en secondes)
const HASH_EXPIRATION: Duration = Duration::new(10, 0); // 10 secondes

fn main() {

    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifier que l'utilisateur a bien fourni au moins deux arguments
    if args.len() < 3 {
        eprintln!("Usage: {} <data> <pattern> [difficulty]", args[0]);
        std::process::exit(1);
    }

    let data = &args[1];
    let pattern = &args[2];

    let difficulty: usize = if args.len() > 3 {
        args[3].parse().unwrap_or_else(|_| {
            eprintln!("La difficulté doit être un entier positif");
            std::process::exit(1);
        })
    } else {
        1 // Valeur par défaut si la difficulté n'est pas spécifiée
    };

    let _difficulty_prefix = pattern.repeat(difficulty);

    let mut nonce = 0;
    let mining_start_time = SystemTime::now();

    // Boucle de minage
    loop {
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)
            .expect("Le temps doit être valide")
            .as_secs();

        let input = format!("{}{}{}", data, timestamp, nonce);

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash_result = hasher.finalize();

        let mut hash_hex = String::new();
        for byte in hash_result.iter() {
            write!(&mut hash_hex, "{:02x}", byte).unwrap();
        }

        if hash_hex.starts_with(&_difficulty_prefix) {
            let mining_end_time = SystemTime::now();
            let elapsed_time = mining_end_time
                .duration_since(mining_start_time)
                .expect("Le temps doit être valide");

            // Vérification de l'expiration
            if elapsed_time > HASH_EXPIRATION {
                eprintln!("Le hash a expiré ! Nonce invalide.");
                std::process::exit(1); // Arrêter si le hash a expiré
            }

            // Si le hash est valide et n'a pas expiré
            println!("Bloc miné !");
            println!("Pattern attendu : {}", pattern);
            println!("Difficulté: {}", difficulty);
            println!("Timestamp: {}", timestamp);
            println!("Nonce trouvé: {}", nonce);
            println!("Hash correspondant: {}", hash_hex);
            println!("Temps de minage : {} ms", elapsed_time.as_millis());
            break;
        }

        nonce += 1;
    }
}
