// Diseña una estructura de datos para una caché LRU con operaciones O(1):
// - obtener: obtener el valor asociado a una clave / verificar si la clave existe.
// - insertar: insertar un valor asociado a una clave.

// Consideraciones:
// - El usuario solo almacenará valores de 0 o números enteros positivos.
// - Si no se encuentra la clave en la caché, devuelve -1.

// Caché LRU (Least Recently Used / Menos Recientemente Usado):
// - Si la caché está llena y llega un nuevo dato, se elimina el dato más antiguo
//   que haya sido accedido (ya sea para lectura o escritura).

// Ejemplo:
// Capacidad máxima de la caché: 2
// insertar("cuba",   7)
// insertar("ramona", 4)
// insertar("nube",   9)
// obtener("cuba")   -> -1 // La clave 1 fue eliminada porque la caché estaba llena.
// obtener("ramona") -> 4  // La clave 2 sigue en la caché.
// obtener("nube")   -> 9  // La clave 3 sigue en la caché.
