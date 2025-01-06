# <center>Juego de adivinanzas con Rust</center>

<header style="display:flex; justify-content: space-around; align-items: center; margin: 24px auto;">

<img style="display:grid; vertical-align: middle" src="../rust.png" width="200"> </header>

## Documentación del ejercicio

Este ejericicio lo he sacado y adaptado del [tutorial de Rust](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

EN el tutorial, el juego se va refinando más, mostrando manejos de errores, creando bucles para que el usuario pueda jugar con la aplicación hasta que adivine el número que genere la terminal. Sin embargo, he decidido subir esta primera parte, donde solo se le da al usuario una oportunidad para adivinar el número, ya que hay demasiados conceptos interesantes y quiero documentarlos poco a poco.

### Novedades respecto al ejercicio anterior

Aquí, usamos librerías de Rust al estilo de las librerías de Java y C para trabajar con string y generar números aleatorios. Las librerías que se van a usar se mencionan al principio del archivo main.rs de este directorio.

Por otro lado, ya se usa una variable mutable a cuyos datos en memoria se acceden después con un puntero. En este sentido, Rust tiene muchas semejanzas con C y lo más importante sería aprenderse las librerías que utiliza.
En el tutorial, los ejemplos de código que usan están desactualizados y tuve que hacer unos cambios con la ayuda del compilador de Cargo, pero la aplicación está operativa. Solo hay que compilar con Cargo y ejecutar la aplicación.

Novedades en el código están en primer lugar el manejo de errores con la función expect, que se encarga de manejar errores muy sencillos y el uso de la función match, muy similar a la función match de php8.

Al leer el código, se nota que escasea una validación robusta de los datos que introduce el usuario por terminal, pues solo se formatea la entrada de datos a un número entero, pero no marcamos error si escribe un número inferior a 1 o uno superior a 5. Simplemente respondemos si se ha pasado por arriba o por abajo a la hora de adivinar el número.

Esto será corregido en el siguiente ejercicio, que optimizará el código de esta versión.