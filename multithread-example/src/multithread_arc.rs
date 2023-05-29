use std::sync::Arc;
use std::thread;

/////
// En este ejemplo, creamos una referencia compartida al vector vec utilizando Arc::new(). 
// Luego, creamos cinco hilos que acceden al vector utilizando la copia de la referencia compartida que se crea dentro del ciclo for. 
// Cada hilo accede a un elemento diferente del vector y lo imprime en la consola.
// Es importante destacar que en este caso no se utiliza Mutex o RwLock porque no estamos modificando el vector, sino simplemente leyendo valores de él.
// Si quisieramos modificar el vector de manera concurrente, tendríamos que utilizar una de estas estructuras para sincronizar los accesos al vector.
/////

pub fn execute() {
    let vec = vec![1, 2, 3, 4, 5];
    let arc_vec = Arc::new(vec); // Creamos una referencia compartida al vector

    let mut handles = vec![];
    for i in 0..5 {
        let arc_vec = Arc::clone(&arc_vec); // Creamos una copia de la referencia compartida
        let handle = thread::spawn(move || {
            // Accedemos al vector desde el hilo
            let value = arc_vec[i];
            println!("Valor del elemento {}: {}", i, value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}