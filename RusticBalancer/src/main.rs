use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{SystemTime, Duration};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use tokio::sync::Mutex;

// Définit les adresses des serveurs 
const SERVERS: [&str; 2] = ["127.0.0.1:8080", "127.0.0.1:8081"];

// Structure pour représenter les informations de cache
struct Cache {
    map: HashMap<String, (String, SystemTime)>, // Mappe les adresses IP aux serveurs et aux timestamps
}

impl Cache {
    // Crée un nouveau cache
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    // Gère la logique du cache
    async fn get_server(&mut self, ip: &str) -> String {
        // Vérifie si l'adresse IP est déjà dans le cache
        if let Some((server, timestamp)) = self.map.get(ip) {
            // Vérifie si le cache est encore valide (moins de 2 secondes)
            if SystemTime::now().duration_since(*timestamp).unwrap() < Duration::from_secs(2) {
                return server.clone(); // Retourne le serveur associé
            }
        }

        // Choisis un serveur aléatoire
        let mut rng = thread_rng();
        let server = SERVERS[rng.gen_range(0..SERVERS.len())];

        // Ajoute l'adresse IP, le serveur et le timestamp au cache
        self.map.insert(ip.to_string(), (server.to_string(), SystemTime::now()));
        return server.to_string(); // Retourne le serveur choisi
    }
}

// Fonction principale exécutée de manière asynchrone
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Prépare le load balancer à l'adresse locale 127.0.0.1 sur le port 7878 et attend
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Load balancer running on localhost:7878");

    // Crée un cache partagé entre les tâches
    let cache = Arc::new(Mutex::new(Cache::new()));

    // Boucle pour accepter les connexions
    loop {
        // Accepte une nouvelle connexion. `socket` est utilisé pour communiquer avec le client
        let (mut socket, addr) = listener.accept().await?;
        
        // Clone le cache pour chaque connexion
        let cache = Arc::clone(&cache);

        // Crée une nouvelle tâche pour gérer la connexion
        tokio::spawn(async move {
            // Récupère l'adresse IP du client
            let ip = addr.ip().to_string();

            // Récupère le cache
            let mut cache = cache.lock().await;

            // Obtient le serveur à partir du cache ou choisi un serveur aléatoire
            let server = cache.get_server(&ip).await;

            // Affiche en console l'adresse du client connecté et le serveur cible sélectionné aléatoirement
            let now = SystemTime::now();
            println!("Redirecting connection from: {} to {} at {:?}", ip, server, now);

            // Établit une connexion avec le serveur cible sélectionné aléatoirement
            let mut server_socket = TcpStream::connect(server).await.unwrap();

            // Crée le buffer pour lire les données
            let mut buf = [0; 1024]; // Augmente la taille du buffer si nécessaire

            // Lit les données de la requête envoyées par le client
            let n = socket.read(&mut buf).await.unwrap();
            if n == 0 { return; } // Connexion fermée par le client

            // Transfère les données lues vers le serveur cible sélectionné
            if server_socket.write_all(&buf[..n]).await.is_err() {
                eprintln!("Failed to write to server");
                return;
            }

            // Lit la réponse du serveur cible et la renvoie au client
            match server_socket.read(&mut buf).await {
                Ok(0) => return, // Connexion fermée par le serveur
                Ok(n) => {
                    // Envoie la réponse au client
                    if socket.write_all(&buf[..n]).await.is_err() {
                        eprintln!("Failed to write back to client");
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    return;
                }
            }
        });
    }
}