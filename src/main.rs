fn main() {
   //% Ownership

    /* let s1 = String::from("ciao");
    let s2 = s1;

    let s1 = String::from("nuovo valore");
    println!("Valore di s1: {}", s1); // nuovo valore
    println!("Valore di s2: {}", s2); // ciao */


    //, Deep copy
    /* let mut s1 = String::from("ciao");
    let mut s2 = s1.clone();
    println!("Valore di s1: {}", s1); // ciao
    println!("Valore di s2: {}", s2); // ciao

    s1 = String::from("Hello");
    println!("Valore di s1: {}", s1); // Hello
    s2 = String::from("World");
    println!("Valore di s2: {}", s2); // World */


    //, Scope ownership
    /* let s1 = String::from("ciao");
    {
        let s2 = s1;
        println!("Valore di s2{}", s2); // Qui s2 è accessibile
     }
    println!("Valore di s2{}", s2);  */ //. Qui s2 non è raggiungibile


    //, Ownership dei dati scalari
    /* let x = 10;
    let y = x;
    println!("Valore di x: {}", x); // 10, x è ancora valido e accessibile
    println!("Valore di y: {}", y); // 10, y ha lo stesso valore di x */


}

// Ownership dei dati scalari
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
