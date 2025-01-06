use std::io;
use std::cmp::Ordering;
use rand::Rng;


/*
* Juego simple de adivinar un número. Solo hay un intento.
*/
fn main() {

    //Solicitar al usuario que escriba un número por terminal
    println!("Adivina el número");
    println!("Por favor escribe un número entre 1 y el 5");

    //Se asigna una variable mutable que sera un string.
    let mut guess = String::new();

    //Se lee eñ dato introducido por el usuario
    //La función expect sirve para manejo de errores
    io::stdin()
        .read_line(&mut guess)
        .expect("No se ha podido leer los datos");

    
    //Convertimos la variable a un dato tipo integer.
    //Si no se puede, se manda mensaje de error al usuario
    let guess: u32 = guess.
        trim()
        .parse()
        .expect("Por favor, escribe un número");

    //Generamos el número secreto.
    //Esta variable es inmutable y es un número aleatorio entre 1 y 5
    let secret_number = rand::rng().random_range(1..=5);

    //Se muestra el número aleatorio al usuario por terminal
    println!("El número generado aleatoriamente es {secret_number}");

    //Se compara el dato que hemos instroducido por terminal con el número secreto
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Te has pasado por debajo"),
        Ordering::Greater => println!("Te has pasado por arriba"),
        Ordering::Equal => println!("¡Has acertado!")
    }
   
}
