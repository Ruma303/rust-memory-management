fn main() {
    //% Ownership

    let s1 = String::from("ciao");
    let s2 = s1; // s2 prende l'ownership di s1
    // s1 è stato spostato in s2, s1 non è più valido

    let s1 = String::from("nuovo valore"); // s1 prende l'ownership (riassegnazione)
    println!("Valore di s1: {}", s1); // nuovo valore
    println!("Valore di s2: {}", s2); // ciao

    //, Deep copy
    let mut s1 = String::from("ciao");
    let mut s2 = s1.clone();
    println!("Valore di s1: {}", s1); // ciao
    println!("Valore di s2: {}", s2); // ciao

    s1 = String::from("Hello");
    println!("Valore di s1: {}", s1); // Hello
    s2 = String::from("World");
    println!("Valore di s2: {}", s2); // World

    //, Ownership dei dati scalari
    //? Copia implicita tramite trait Copy
    let x = 10;
    let y = x;
    println!("Valore di x: {}", x); // 10, x è ancora valido e accessibile
    println!("Valore di y: {}", y); // 10, y ha lo stesso valore di x

    //, Ownership durante passaggio dati come parametri di funzioni
    let x = 10;
    let y = 20;
    let z = sum(x, y);
    println!("Valore di z: {}", z);

    // println!("Variabili x e y non piu' valide"); // x e y non sono piu' valide

    let s = String::from("ciao");
    prende_ownership(s);

    //, Restituire l'ownership
    let s1 = String::from("ciao");
    let s2 = restituisci(s1);
    // println!("{}", s1); // ERRORE: s1 è stato mosso
    println!("Ownership di s2 restituita: {}", s2); // OK

    //, Scope ownership
    {
        let s = String::from("ciao");
        println!("{}", s);
    } // s esce dallo scope e viene deallocato. Non è piu' valido
    // println!("{}", s); // ERRORE

    let s1 = String::from("ciao");
    {
        let s2 = s1;
        println!("Valore di s2: {}", s2); // Qui s2 è accessibile
    }
    println!("Valore di s2: {}", s2); // Qui s2 è raggiungibile, ma s1 no.
}

// Ownership dati complessi che non implementano Copy
fn prende_ownership(val: String) {
    println!("Valore trasferito alla funzione: {}", val);
}

// Ownership dei dati scalari
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Restituire l'ownership
fn restituisci(s: String) -> String {
    s
}
