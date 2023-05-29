
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectarse a la base de datos
    let mut client = Client::connect("postgresql://postgres:admin@127.0.0.1:15432/curso_rust", NoTls)?;

    // Crear tabla
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS usuarios (
            id SERIAL PRIMARY KEY,
            nombre TEXT NOT NULL,
            edad INTEGER NOT NULL
        )
    ")?;

    // Insertar registro
    let nombre = "Juan";
    let edad = 30;
    client.execute("INSERT INTO usuarios (nombre, edad) VALUES ($1, $2)", &[&nombre, &edad])?;

    // Consultar registros
    for row in client.query("SELECT id, nombre, edad FROM usuarios", &[])? {
        let id: i32 = row.get(0);
        let nombre: String = row.get(1);
        let edad: i32 = row.get(2);

        println!("ID: {}, Nombre: {}, Edad: {}", id, nombre, edad);
    }

    Ok(())
}
