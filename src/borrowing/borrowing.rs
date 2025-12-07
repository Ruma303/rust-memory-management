fn show_all_data(slice: &[i32]) {
    for &value in slice {
        println!("{}", value);
    }
}

fn increment(n: &mut i32) {
    *n += 1; // 11: Incrementiamo il valore a cui il riferimento punta
}

fn update_word(word: &mut String) {
    word.push_str(" World");
    println!("{}", word);
}

fn do_stuff(vec: &Vec<i32>) -> &Vec<i32> {
    println!("{:?}", vec);
    &vec
}

pub fn borrowing() {
    //% Borrowing

    //, Immutable Borrowing
    let data = vec![1, 2, 3, 4];
    show_all_data(&data); // Borrow immutabile di data
    println!("data: {:?}.", data); // data è ancora valido e mantiene l'ownership

    //, Mutable Borrowing
    let mut numero = 10;
    increment(&mut numero); // Passiamo un riferimento mutabile alla funzione
    println!("Il numero incrementto è: {}", numero); // Stampiamo il valore incrementto

    //# Evitare Race conditions
    /* let mut vec_1 : Vec<i32> = vec![1, 2, 3];&
    let ref1 : &mut Vec<i32> = &mut vec_1;
    let ref2 : &mut Vec<i32> = &mut vec_1;

    println!("ref1: {:?}. ref2: {:?}", ref1, ref2); */ //. Errore

    //* Immutable Borrowing
    let mut vec_1: Vec<i32> = vec![1, 2, 3];
    let ref1: &Vec<i32> = &vec_1;
    let ref2: &Vec<i32> = &vec_1;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    //* Mutable Borrowing
    let ref3: &mut Vec<i32> = &mut vec_1;
    println!("ref3: {:?}", ref3);

    //# Con funzioni
    let mut word = String::from("Hello");
    update_word(&mut word); // borrow mutabile
    println!("{}", word); // "Hello World"

    //# Con tipi che implementano Copy
    let array1 = [1, 2, 3, 4, 5];
    let array2 = array1; // copia, non move
    println!("{:?}", (array1, array2));

    //# Per tipi allocati
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1; // borrow immutabile
    // println!("{:?}", (vec1, vec2)); // Errore

    //? Volendo, possiamo permettere la copia con clone
    let vec3 = vec1.clone();
    println!("{:?}", (vec2, vec3));

    //, Borrowing e lifetimes
    //* Es 1
    let mut numero = 10;
    {
        let r1 = &numero; // r1 è un riferimento immutabile a numero
        println!("r1: {}", r1); // OK: leggere attraverso r1 è sicuro

        // r1 esce dallo scope qui in quanto non viene più usato

        let r2 = &mut numero; // r2è un riferimento mutabile a numero
        println!("r2: {}", r2); // OK: leggere attraverso r2 è sicuro
    }

    //* Es 2
    //# Evitare dangling pointers
    {
        let r3 = &mut numero;
        *r3 += 1; // OK: modifichiamo numero attraverso r3
        println!("r3 : {}", r3); // OK: leggere attraverso r3 è sicuro

        let r4 = &mut numero; // me lo fa fare lo stesso
        println!("r4: {}", r4);
    } // r2 esce dallo scope qui. Possiamo creare nuovamente riferimenti mutabili o immutabili a numero */

    //* Es 3
    let v = vec![42];
    do_stuff(&v);
}
