use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Point d'entrée principal de l'application. Cette fonction lit les adresses des serveurs
/// à partir du fichier `conf.txt` et les affiche. Elle inclut également des adresses de serveurs initiales par défaut.
///
/// # Returns
///
/// `io::Result<()>` - Un résultat indiquant le succès ou l'échec de l'exécution de la fonction.
///
/// # Examples
///
/// ```rust
/// fn main() -> io::Result<()> {
///     // Définit les adresses des serveurs initiales
///     let initial_servers: [&str; 2] = ["127.0.0.1:8080", "127.0.0.1:8081"];
///     
///     // Lire les adresses des serveurs à partir du fichier conf.txt
///     let mut servers = Vec::new();
///     servers.extend_from_slice(&initial_servers);
///     
///     if let Ok(lines) = read_lines("conf.txt") {
///         for line in lines {
///             if let Ok(ip) = line {
///                 servers.push(ip);
///             }
///         }
///     }
///
///     // Affiche les serveurs pour vérifier
///     for server in servers {
///         println!("{}", server);
///     }
///
///     Ok(())
/// }
/// ```
///
/// # Panics
///
/// Cette fonction ne panique pas.
///
/// # Errors
///
/// Cette fonction retourne une erreur si elle échoue à lire le fichier `conf.txt`.
fn main() -> io::Result<()> {
    // Définit les adresses des serveurs initiales
    let initial_servers: [&str; 2] = ["127.0.0.1:8080", "127.0.0.1:8081"];
    
    // Lire les adresses des serveurs à partir du fichier conf.txt
    let mut servers = Vec::new();
    servers.extend_from_slice(&initial_servers);
    
    if let Ok(lines) = read_lines("conf.txt") {
        for line in lines {
            if let Ok(ip) = line {
                servers.push(ip);
            }
        }
    }

    // Affiche les serveurs pour vérifier
    for server in servers {
        println!("{}", server);
    }

    Ok(())
}

/// Lit les lignes d'un fichier et retourne un itérateur sur les lignes. Chaque ligne est encapsulée dans un `io::Result`.
///
/// # Parameters
///
/// - `filename`: Le chemin vers le fichier à lire. Le type de `filename` doit implémenter le trait `AsRef<Path>`.
///
/// # Returns
///
/// `io::Result<io::Lines<io::BufReader<File>>>` - Un résultat contenant un itérateur sur les lignes du fichier.
/// Chaque ligne est encapsulée dans un `io::Result`, permettant de gérer les erreurs de lecture.
///
/// # Examples
///
/// ```rust
/// fn main() -> io::Result<()> {
///     if let Ok(lines) = read_lines("conf.txt") {
///         for line in lines {
///             if let Ok(ip) = line {
///                 println!("{}", ip);
///             }
///         }
///     }
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Cette fonction retourne une erreur si le fichier ne peut pas être ouvert.
///
/// # Panics
///
/// Cette fonction ne panique pas.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
