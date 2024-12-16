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
// obtener("cuba")   -> -1 // La clave "cuba" fue eliminada porque la caché estaba llena.
// obtener("ramona") -> 4  // La clave "ramona" sigue en la caché.
// obtener("nube")   -> 9  // La clave "nube" sigue en la caché.

use std::cell::RefCell;
use std::rc::Rc;

type NodoRef = Rc<RefCell<Nodo>>;

struct Nodo {
    elemento: i32,
    anterior: Option<NodoRef>,
    siguiente: Option<NodoRef>,
}

struct Lista {
    cabeza: Option<NodoRef>,
    cola: Option<NodoRef>,
}

impl Lista {
    fn agregar_cabeza(&self, nodo: NodoRef) {
        todo!()
    }

    fn sacar_cola(&self) -> NodoRef {
        todo!()
    }

    fn sacar(&self, nodo: NodoRef) -> NodoRef {
        // si el nodo es la cabeza
        // si el nodo es la cola

        // si el nodo esta en el medio
        // al anterior apuntarle el siguiente
        // al siguiente apuntarle el anterior
        // al nodo le pongo nada como anterior y siguiente
        let anterior = nodo.borrow().anterior.clone();
        let siguiente = nodo.borrow().siguiente.clone();

        anterior.clone().map(|anterior| anterior.borrow_mut().siguiente = siguiente.clone());
        siguiente.map(|siguiente| siguiente.borrow_mut().anterior = anterior);

        nodo.borrow_mut().anterior = None;
        nodo.borrow_mut().siguiente = None;
        nodo
    }

}
