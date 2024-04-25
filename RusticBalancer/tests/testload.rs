use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::main;


#[tokio::test]
async fn test_load_balancer() {
    // Prépare le load balancer à l'adresse locale 127.0.0.1 sur le port 7878 et attend
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    println!("Load balancer running on localhost:7878");

    // Crée un cache partagé entre les tâches
    let cache = Arc::new(Mutex::new(main::Cache::new()));

    // Crée une nouvelle tâche pour le loadbalancer
    let loadbalancer = tokio::spawn(async move {
        // Boucle pour accepter les connexions
        loop {
            // Accepte une nouvelle connexion. `socket` est utilisé pour communiquer avec le client
            let (mut socket, addr) = listener.accept().await.unwrap();
            
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
    });

    // Teste le loadbalancer
    test_cache_functionality(cache.clone()).await;
    test_random_server_selection(cache.clone()).await;

    // Arrête le loadbalancer
    loadbalancer.await.unwrap();
}

async fn test_cache_functionality(cache: Arc<Mutex<main::Cache>>) {
    let mut cache = cache.lock().await;

    // Vérifie que le cache fonctionne correctement
    let server1 = cache.get_server("127.0.0.1").await;
    let server2 = cache.get_server("127.0.0.1").await;
    assert_eq!(server1, server2, "Le cache ne fonctionne pas correctement");

    // Vérifie que le cache expire après 2 secondes
    std::thread::sleep(Duration::from_secs(3));
    let server3 = cache.get_server("127.0.0.1").await;
    assert_ne!(server1, server3, "Le cache n'expire pas correctement");
}

async fn test_random_server_selection(cache: Arc<Mutex<main::Cache>>) {
    let mut cache = cache.lock().await;
    let mut server_counts = HashMap::new();

    // Effectue 100 requêtes et compte le nombre de fois que chaque serveur est sélectionné
    for _ in 0..100 {
        let server = cache.get_server("127.0.0.1").await;
        *server_counts.entry(server).or_insert(0) += 1;
    }

    // Vérifie que les deux serveurs sont sélectionnés de manière aléatoire
    assert!(server_counts["127.0.0.1:8080"] > 0, "Le serveur 8080 n'a pas été sélectionné");
    assert!(server_counts["127.0.0.1:8081"] > 0, "Le serveur 8081 n'a pas été sélectionné");
}