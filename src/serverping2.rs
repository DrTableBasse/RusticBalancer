use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

// Démarre un environnement d'exécution asynchrone pour permettre au serveur de fonctionner
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Lie le serveur TCP à l'adresse locale 127.0.0.1 sur le port 8080 et wait
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Server running on 127.0.0.1:8081");

    // Boucle pour accepter les connexions
    loop {
        // Accepte une nouvelle connexion `socket` est utilisé pour communiquer avec le client
        let (mut socket, _) = listener.accept().await?;
        
        // Crée une tâche asynchrone pour chaque connexion entrante pour gérer les communications
        tokio::spawn(async move {
            let response = b"Hello World";  // Définit la réponse à envoyer au client
            // Affiche l'adresse du client connecté, msg de log console
            println!("Received connection from: {:?}", socket.peer_addr().unwrap());
            
            // Crée un buffer pour lire les données
            let mut buf = vec![0; 1024];

            // Boucle pour lire les données envoyées par le client
            loop {
                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => break, // Si le client ferme la connexion, arrête la boucle
                    Ok(_n) => {
                        // Envoie la réponse au client. Si l'envoi échoue, imprime un message d'erreur et arrête la boucle
                        if socket.write_all(response).await.is_err() {
                            eprintln!("Failed to send response");
                            break;
                        }
                    },
                    Err(_) => break, // En cas d'erreur de lecture, arrête la boucle
                }
            }
        });
    }
}
