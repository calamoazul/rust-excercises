# Hello World en Rust

<header style="display:flex; justify-content: space-around; align-items: center; margin: 24px auto;">
<img src="../logo_rust.png" width="100">
<img style="display:grid; vertical-align: middle" src="../rust.png" width="200"> </header>

Este ejercicio no tiene mayor complicación.

Al igual que en Java o en C, el programa se basa en una función principal llamada main.

No hay que especificar el tipo de dato al declarar la variable, pero una vez declarada, la variable se convierte en un tipo de dato inmutable. Para impedirlo, habría que definir la variable así:

```rust
rust
let mut greeting = "Hello World"
```
Esto permitiría reasignar el valor de la variable si fuera necesario.
Otro aspecto importante a tener en cuenta en Rust es que sus funciones macro se distinguen por finalizar con signo de exclamación, por ejemplo:

```rust
rust
println!("{greeting}")
```

Si no se añade el signo de exclamación, el programa daría un error al compilar porque consideraría que la función la hemos definido nosotros.

Al margen de esto, no hay mucho más que señalar en este ejercicio, pues es el más sencillo que hay. Ni siguiera utiliza cargo para compilar.

El ejecutable se crea con el siguiente comando:
```bash
bash 
rustc main.rs
```
De esa manera, se genera el archivo .exe y podríamos ejecutarlo desde bash como cualquier otro ejecutable.

Para comprobar que tienes las dependencias instaladas correctamente, ejecuta los siguientes comandos:

```bash 
bash
rustc --version
cargo --version
``` 