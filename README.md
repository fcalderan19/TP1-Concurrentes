[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/YzV_0XZo)
Instrucciones
-------------

- Descargar el dataset de https://www.kaggle.com/datasets/najzeko/steam-reviews-2021 
- Guardar y descomprimir en un path conocido.
- Implementar el código según el enunciado https://concurrentes-fiuba.github.io/2025_1C_tp1.html

Ejecución
---------

```
cargo run <input-path> <num-threads> <output-file-name>
```

por ejemplo

```
cargo run ~/Downloads/dataset 4 output.json
```

Pruebas
-------

- La salida de la ejecución con el dataset completo debe ser igual a la del archivo `expected_output.json`, sin importar
  el orden de aparición de las keys en los mapas.


Comentarios
-----------

- En la parte de tests hay una carpeta llamada `benchmarks`. En esta carpeta se almacenaran individualmente los tiempos de cada etapa del programa, es decir, que cuando se corran las pruebas con `cargo test`, se guardaran en archivos separados los tiempos con 2 y 4 threads de cada test.

- PADRON: 110873

- Link al video explicativo: https://drive.google.com/file/d/1fEMHQVk6FqAb4qag2eF6-Y9MoS-tym4E/view?usp=sharing. Ante cualquier duda, dejo mi mail por si acaso: fcalderan@fi.uba.ar
