pub fn execute() {

    let mut x = 5;
    let s = String::from("Ejemplo de FnOnce");

    let f = || println!("x is {}", x);

    // let mut g = move || {
    //     println!("x is {}", x);
    //     x += 1;
    //     println!("now x is {}", x);
    //     let other_s = s.to_lowercase();
    //     println!("now other_s is {}", other_s);
    // };

    let h =  || {
        let other_s = s;
        println!("s is {}", other_s.to_uppercase());
    };

    f();
    // g();
    // g();
    // g();
    h();

}
