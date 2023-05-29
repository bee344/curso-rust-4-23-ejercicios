use crossbeam::channel;

pub fn execute() {
    // Creamos un canal sin buffer con capacidad para un solo mensaje
    let (tx, rx) = channel::bounded(1);

    // Creamos un thread que envía mensajes a través del canal
    let tx_handle = std::thread::spawn(move || {
        for i in 0..10 {
            // Enviamos el mensaje al canal
            tx.send(i).unwrap();

            // Esperamos un momento para que el receptor tenga tiempo de leer el mensaje
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Creamos otro thread que recibe mensajes del canal
    let rx_handle = std::thread::spawn(move || {
        for _ in 0..10 {
            // Leemos el mensaje del canal
            let msg = rx.recv().unwrap();

            // Imprimimos el mensaje en la consola
            println!("Mensaje recibido: {}", msg);
        }
    });

    // Esperamos a que ambos threads terminen
    tx_handle.join().unwrap();
    rx_handle.join().unwrap();
}
