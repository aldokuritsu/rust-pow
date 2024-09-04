use sha2::{Sha256, Digest}; // Importation du module de hachage SHA-256
use std::fmt::Write; // Pour formater la sortie en hexadécimal
use std::time::{SystemTime, Duration, UNIX_EPOCH}; // Manipulation du temps
use std::env; // Récupération des arguments en ligne de commande

// Durée d'expiration pour un hash valide (en secondes)
const HASH_EXPIRATION: Duration = Duration::new(10, 0); // 10 secondes

fn main() {
    // Récupérer les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Validation des arguments
    let (data, pattern, difficulty) = match parse_arguments(&args) {
        Ok(result) => result,
        Err(error_message) => {
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
    };

    // Lancer la procédure de minage
    match mine_block(data, pattern, difficulty) {
        Ok(_) => (),
        Err(e) => eprintln!("Erreur lors du minage : {}", e),
    }
}

// Fonction de parsing des arguments
fn parse_arguments(args: &[String]) -> Result<(&str, &str, usize), String> {
    if args.len() < 3 {
        return Err(format!(
            "Usage: {} <data> <pattern> [difficulty]",
            args[0]
        ));
    }

    let data = &args[1];
    let pattern = &args[2];

    let difficulty: usize = if args.len() > 3 {
        args[3].parse().map_err(|_| "La difficulté doit être un entier positif".to_string())?
    } else {
        1 // Valeur par défaut si la difficulté n'est pas spécifiée
    };

    Ok((data, pattern, difficulty))
}

// Fonction de minage
fn mine_block(data: &str, pattern: &str, difficulty: usize) -> Result<(), String> {
    let difficulty_prefix = pattern.repeat(difficulty);
    let mut nonce = 0;
    let mining_start_time = SystemTime::now();

    loop {
        let timestamp = get_current_timestamp()?;
        let input = format!("{}{}{}", data, timestamp, nonce);

        let hash_hex = compute_sha256_hash(&input);

        if hash_hex.starts_with(&difficulty_prefix) {
            let elapsed_time = mining_start_time
                .elapsed()
                .map_err(|_| "Erreur lors du calcul du temps écoulé".to_string())?;

            if elapsed_time > HASH_EXPIRATION {
                return Err("Le hash a expiré ! Nonce invalide.".to_string());
            }

            // Affichage des informations de succès
            display_success_info(pattern, difficulty, timestamp, nonce, &hash_hex, elapsed_time);
            break;
        }

        nonce += 1;
    }

    Ok(())
}

// Fonction pour obtenir le timestamp actuel en secondes
fn get_current_timestamp() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|_| "Erreur lors de la récupération du timestamp".to_string())
}

// Fonction pour calculer le hash SHA-256 et le formater en hexadécimal
fn compute_sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash_result = hasher.finalize();

    let mut hash_hex = String::new();
    for byte in hash_result.iter() {
        write!(&mut hash_hex, "{:02x}", byte).unwrap();
    }

    hash_hex
}

// Fonction pour afficher les informations après un minage réussi
fn display_success_info(
    pattern: &str,
    difficulty: usize,
    timestamp: u64,
    nonce: u64,
    hash_hex: &str,
    elapsed_time: Duration,
) {
    println!("Bloc miné !");
    println!("Pattern attendu : {}", pattern);
    println!("Difficulté: {}", difficulty);
    println!("Timestamp: {}", timestamp);
    println!("Nonce trouvé: {}", nonce);
    println!("Hash correspondant: {}", hash_hex);
    println!("Temps de minage : {} ms", elapsed_time.as_millis());
}
