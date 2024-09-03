use sha2::{Sha256, Digest}; // Importation des bibliothèques nécessaires pour le hachage
use std::fmt::Write; // Importation pour formater la sortie hexadécimale
use std::time::{SystemTime, UNIX_EPOCH}; // Importation pour manipuler le temps
use std::env; // Importation pour récupérer les arguments en ligne de commande

fn main() {

    // Récupérer les arguments de la ligne de commande et les stocker dans un vecteur
    let args: Vec<String> = env::args().collect();

    // Vérifier que l'utilisateur a bien fourni au moins deux arguments (data et pattern), sinon afficher un message d'erreur
    if args.len() < 3 {
        eprintln!("Usage: {} <data> <pattern> [difficulty]", args[0]);
        std::process::exit(1); // Arrêt du programme si les arguments ne sont pas suffisants
    }

    // Récupérer la chaîne de caractères (data) à hacher depuis les arguments
    let data = &args[1];
    
    // Récupérer le pattern (chaîne à rechercher en début de hash) depuis les arguments
    let pattern = &args[2];
    
    // Récupérer la difficulté (nombre de répétitions du pattern) depuis les arguments, ou utiliser 1 comme valeur par défaut
    let difficulty: usize = if args.len() > 3 {
        args[3].parse().unwrap_or_else(|_| {
            eprintln!("La difficulté doit être un entier positif");
            std::process::exit(1); // Arrêt du programme en cas de mauvaise entrée
        })
    } else {
        1 // Valeur par défaut si la difficulté n'est pas spécifiée
    };

    // Créer une chaîne de caractères correspondant au préfixe cible du hash (le pattern répété autant de fois que la difficulté)
    let _difficulty_prefix = pattern.repeat(difficulty);

    // Initialiser le nonce à 0, le nonce étant une valeur qui sera incrémenté jusqu'à ce qu'une solution soit trouvée
    let mut nonce = 0;

    // Enregistrer le temps de début du minage pour mesurer la durée totale de l'opération
    let mining_start_time = SystemTime::now();

    // Boucle principale qui cherche à trouver un nonce valide
    loop {
        // Obtenir le timestamp actuel en secondes depuis l'UNIX EPOCH
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)
            .expect("Le temps qui s'est écoulé depuis l'epoch UNIX")
            .as_secs();

        // Combiner les données, le timestamp et le nonce pour former l'entrée du hash
        let input = format!("{}{}{}", data, timestamp, nonce);

        // Initialiser le hasher SHA-256 et calculer le hash de l'entrée
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash_result = hasher.finalize();

        // Convertir le hash (une série de bytes) en une chaîne hexadécimale lisible
        let mut hash_hex = String::new();
        for byte in hash_result.iter() {
            write!(&mut hash_hex, "{:02x}", byte).unwrap();
        }

        // Vérifier si le hash commence par le pattern souhaité
        if hash_hex.starts_with(&_difficulty_prefix) {
            // Si le hash est valide, calculer le temps écoulé depuis le début du minage
            let mining_end_time = SystemTime::now();
            let elapsed_time = mining_end_time
                .duration_since(mining_start_time)
                .expect("Le temps doit être valide")
                .as_millis(); // Convertir le temps écoulé en millisecondes

            // Afficher les résultats : pattern, difficulté, timestamp, nonce trouvé, hash correspondant et temps de minage
            println!("Bloc miné !");
            println!("Pattern attendu : {}", pattern);
            println!("Difficulté: {}", difficulty);
            println!("Timestamp: {}", timestamp);
            println!("Nonce trouvé: {}", nonce);
            println!("Hash correspondant: {}", hash_hex);
            println!("Temps de minage : {} ms", elapsed_time);
            break; // Sortir de la boucle une fois que le hash valide est trouvé
        }
        nonce += 1; // Incrémenter le nonce et recommencer la boucle pour essayer avec une nouvelle valeur
    }
}
