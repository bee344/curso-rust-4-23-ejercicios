use std::rc::Rc;

struct Person {
    name: String,
    age: u8,
}

pub fn execute() {
    // Creamos una instancia de la estructura Person y la envolvemos en un Rc
    let person = Rc::new(Person {
        name: String::from("Juan"),
        age: 30,
    });

    // Clonamos la referencia Rc varias veces para compartirla
    let person1 = person.clone();
    let person2 = person.clone();
    let person3 = person.clone();

    // Imprimimos los datos de la persona a través de cada referencia clonada
    println!("Name: {}", person1.name);
    println!("Age: {}", person2.age);
    println!("Name and Age: {} {}", person3.name, person3.age);
}
