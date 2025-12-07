pub fn deref() {
    //% Dereferencing

    let x = 5;
    let y = &x; // y è un riferimento a x

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{x}, {y}"); // 5, 5

    //, Dereferencing con Mutabilità

    let mut data : i32 = 42;
    let ref1 : &mut i32 = &mut data;
    let deref_ref1 : i32 = *ref1 + 10;
    println!("{ref1}, {deref_ref1}" ); // 42, 52

    //, Deref non possibile
    let mut data : Vec<i32> = vec![1,2,3];
    let ref1 : &mut Vec<i32> = &mut data;

    //let deref_ref1 : Vec<i32> = *ref1; //. Errore

    //? Clonare i dati invece di tentare di spostarli
    let deref_ref1: Vec<i32> = ref1.clone();
    println!("{:?}, {:?}", ref1, deref_ref1); // [1, 2, 3], [1, 2, 3]

    //, Deref mut non possibile
    /* let mut data : Vec<i32> = vec![1,2,3];
    let ref1 : &mut Vec<i32> = &mut data;
    let deref_ref1 : &mut Vec<i32> = &mut *ref1; */
}
