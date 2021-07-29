// A simple websocket server example
// Use webclient.html to connect to the server

fn main() {
    // Certificate configuration, generated with:
    // openssl req -x509 -sha256 -nodes -days 365 -newkey rsa:2048 -keyout localhost.key -out localhost.crt
    let localhost_ssl_key: &[u8] = include_bytes!("../ssl/localhost.key"); // private key
    let localhost_ssl_crt: &[u8] = include_bytes!("../ssl/localhost.crt"); // self signed certificate
    
    let key = rustls::internal::pemfile::pkcs8_private_keys(&mut &*localhost_ssl_key).unwrap().pop().unwrap();
    let crt = rustls::internal::pemfile::certs             (&mut &*localhost_ssl_crt).unwrap();

    // Setup TLS server configuration from certificate configuration
    let mut tls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    tls_config.set_single_cert(crt, key).unwrap();
    let tls_config = std::sync::Arc::new(tls_config);

    // Start server
    println!("Listening on localhost:8443...");
    let server = std::net::TcpListener::bind("localhost:8443").unwrap();

    // For each incoming connection
    for stream in server.incoming() {        
        println!("Incoming connection...");

        // Wrap to a TLS stream
        let tls_session = rustls::ServerSession::new(&tls_config);    
        let tls_stream = rustls::StreamOwned::new(tls_session, stream.unwrap());

        // Spawn a thread with a websocket handling it
        println!("Spawning secure websocket thread...");
        std::thread::spawn(move || {
            // thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: 
            // HandshakeError::Failure(Io(Custom { kind: InvalidData, error: AlertReceived(CertificateUnknown) }))'
            let mut websocket = tungstenite::accept(tls_stream).unwrap(); // <- panicks
            println!("Websocket accepted TLS stream...");
            loop {
                println!("Waiting for text message from connected client...");
                let msg = websocket.read_message().unwrap();
                if msg.is_text() {
                    println!("Client sent message: '{}'", msg);
                    println!("Echoing back message to client...");
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}