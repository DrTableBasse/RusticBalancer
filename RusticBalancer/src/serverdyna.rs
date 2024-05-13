use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
