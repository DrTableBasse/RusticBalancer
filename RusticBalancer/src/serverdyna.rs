use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Gère la connexion d'un client, enregistre les détails de la connexion et envoie une réponse.
///
/// Cette fonction est exécutée de manière asynchrone pour chaque client connecté.
///
/// # Arguments
///
/// * `socket` - Un objet `TcpStream` représentant la connexion du client.
/// * `user` - Une `String` représentant le nom de l'utilisateur.
/// * `ip` - Une `String` représentant l'adresse IP du client.
/// * `server` - Une `String` représentant l'adresse du serveur.
///
/// # Examples
///
/// ```
/// tokio::spawn(handle_client(socket, "user1".to_string(), "127.0.0.1".to_string(), "127.0.0.1:8080".to_string()));
/// ```
///
/// # Panics
///
/// Cette fonction ne devrait pas paniquer dans des conditions normales d'utilisation.
///
/// # Errors
///
/// Cette fonction enregistre les erreurs dans la sortie standard d'erreurs (`stderr`) lorsqu'elles se produisent.
///

async fn handle_client(mut socket: TcpStream, user: String, ip: String, server: String) {
    println!("Nouvelle connexion établie avec l'utilisateur '{}' depuis l'adresse IP '{}' sur le serveur '{}'.", user, ip, server);

    // Ouverture du fichier de logs
    let mut log_file = match OpenOptions::new().create(true).append(true).open("log.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier de logs : {}", e);
            return;
        }
    };

    // Écriture des détails de la connexion dans le fichier de logs
    if let Err(e) = writeln!(log_file, "Nouvelle connexion de l'utilisateur '{}' depuis l'adresse IP '{}' sur le serveur '{}'.", user, ip, server) {
        eprintln!("Erreur lors de l'écriture dans le fichier de logs : {}", e);
        return;
    }

    // Envoyer la réponse "hello world" au client
    if let Err(e) = socket.write_all(b"hello world").await {
        eprintln!("Erreur d'écriture pour le client '{}': {}", user, e);
    } else {
        println!("Réponse 'hello world' envoyée au client '{}'.", user);
    }

    // Fermer la connexion
    if let Err(e) = socket.shutdown().await {
        eprintln!("Erreur lors de la fermeture de la connexion pour le client '{}': {}", user, e);
    } else {
        println!("Client '{}' déconnecté.", user);
    }
}



/// Point d'entrée principal de l'application. Lit les configurations du fichier `conf.txt`,
/// démarre les serveurs et gère les connexions entrantes.
///
/// Cette fonction utilise Tokio pour gérer des opérations asynchrones, notamment l'écoute de connexions TCP,
/// le partage de données entre tâches et la gestion des signaux pour arrêter les serveurs proprement.
///
/// # Returns
///
/// `Result<(), Box<dyn std::error::Error>>` - Un résultat indiquant le succès ou l'échec de l'exécution de la fonction.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Récupération du répertoire de travail actuel
///     if let Ok(current_dir) = env::current_dir() {
///         println!("Répertoire actuel : {:?}", current_dir);
///     } else {
///         println!("Impossible de récupérer le répertoire actuel.");
///     }
///
///     let running = Arc::new(tokio::sync::Mutex::new(true));
///     let mut tasks = Vec::new();
///
///     let file = std::fs::File::open("conf.txt")?;
///     let reader = BufReader::new(file);
///
///     for line in reader.lines() {
///         let line = line?;
///         let parts: Vec<&str> = line.split(":").collect();
///
///         if parts.len() != 2 {
///             eprintln!("La ligne '{}' n'est pas valide.", line);
///             continue;
///         }
///
///         let ip = parts[0].trim().to_string();
///         let port = parts[1].trim().parse::<u16>();
///
///         match port {
///             Ok(port) => {
///                 let addr = format!("{}:{}", ip, port);
///                 match addr.parse::<std::net::SocketAddr>() {
///                     Ok(socket_addr) => {
///                         let listener = TcpListener::bind(&socket_addr).await?;
///                         println!("Serveur démarré sur {}", socket_addr);
///
///                         let running_clone = Arc::clone(&running);
///
///                         let task = tokio::spawn(async move {
///                             let user = "utilisateur inconnu".to_string();
///                             loop {
///                                 tokio::select! {
///                                     result = listener.accept() => {
///                                         match result {
///                                             Ok((socket, _)) => {
///                                                 let mut running = running_clone.lock().await;
///                                                 if *running {
///                                                     tokio::spawn(handle_client(socket, user.clone(), ip.clone(), addr.clone()));
///                                                 } else {
///                                                     println!("Arrêt demandé. Fermeture du serveur...");
///                                                     return;
///                                                 }
///                                             },
///                                             Err(e) => eprintln!("Erreur lors de l'acceptation de la connexion: {}", e),
///                                         }
///                                     }
///                                     _ = tokio::signal::ctrl_c() => {
///                                         println!("Signal Ctrl + C reçu. Arrêt des serveurs...");
///                                         *running_clone.lock().await = false;
///                                         return;
///                                     }
///                                 }
///                             }
///                         });
///                         tasks.push(task);
///                     }
///                     Err(e) => eprintln!("Erreur de parsing de l'adresse : {}", e),
///                 }
///             }
///             Err(e) => eprintln!("Erreur de parsing du port : {}", e),
///         }
///     }
///
///     for task in tasks {
///         task.await?;
///     }
///
///     Ok(())
/// }
/// ```
///
/// # Panics
///
/// Cette fonction ne devrait pas paniquer dans des conditions normales d'utilisation.
///
/// # Errors
///
/// Cette fonction retourne une erreur si elle échoue à ouvrir le fichier `conf.txt`,
/// à analyser les adresses IP ou les ports, ou à lier les listeners TCP.
///
/// # Tokio
///
/// Cette fonction utilise l'attribut `#[tokio::main]` pour indiquer qu'elle est le point d'entrée
/// d'une application Tokio asynchrone.


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupération du répertoire de travail actuel
    if let Ok(current_dir) = env::current_dir() {
        println!("Répertoire actuel : {:?}", current_dir);
    } else {
        println!("Impossible de récupérer le répertoire actuel.");
    }

    // Créer un Arc pour partager entre threads
    let running = Arc::new(tokio::sync::Mutex::new(true));

    let mut tasks = Vec::new();

    // Lecture du fichier conf.txt
    let file = std::fs::File::open("conf.txt")?;
    let reader = BufReader::new(file);

    // Boucle de lecture du fichier et démarrage des serveurs
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            eprintln!("La ligne '{}' n'est pas valide.", line);
            continue;
        }

        let ip = parts[0].trim().to_string();
        let port = parts[1].trim().parse::<u16>();

        match port {
            Ok(port) => {
                let addr = format!("{}:{}", ip, port);
                match addr.parse::<std::net::SocketAddr>() {
                    Ok(socket_addr) => {
                        let listener = TcpListener::bind(&socket_addr).await?;
                        println!("Serveur démarré sur {}", socket_addr);

                        // Créer une copie de l'Arc pour les threads spawnés
                        let running_clone = Arc::clone(&running);

                        // Boucle d'écoute des connexions
                        let task = tokio::spawn(async move {
                            let user = "utilisateur inconnu".to_string(); // À remplacer par le nom d'utilisateur approprié
                            loop {
                                tokio::select! {
                                    result = listener.accept() => {
                                        match result {
                                            Ok((socket, _)) => {
                                                let mut running = running_clone.lock().await;
                                                if *running {
                                                    tokio::spawn(handle_client(socket, user.clone(), ip.clone(), addr.clone()));
                                                } else {
                                                    println!("Arrêt demandé. Fermeture du serveur...");
                                                    return; // Quitter le thread si on demande l'arrêt
                                                }
                                            },
                                            Err(e) => eprintln!("Erreur lors de l'acceptation de la connexion: {}", e),
                                        }
                                    }
                                    _ = tokio::signal::ctrl_c() => {
                                        println!("Signal Ctrl + C reçu. Arrêt des serveurs...");
                                        *running_clone.lock().await = false; // Mettre la variable de running à false
                                        return;
                                    }
                                }
                            }
                        });
                        tasks.push(task);
                    }
                    Err(e) => eprintln!("Erreur de parsing de l'adresse : {}", e),
                }
            }
            Err(e) => eprintln!("Erreur de parsing du port : {}", e),
        }
    }

    // Attendre que toutes les tâches se terminent
    for task in tasks {
        task.await?;
    }

    Ok(())
}
