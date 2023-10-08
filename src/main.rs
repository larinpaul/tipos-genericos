//// 2023/10/08 // 17:30 //

// #18 Tipos de datos genericos

// Tipos de datos genericos
// El lenguaje RUst nos ofrece usar tipos de datos genericos para creat
// definiciones de funciones o con estructuras, que nos va a permitir poder usar con
// tipos de datos diferentes.

// En este video primero vamos a aprender como definir funciones, estructuras,
// enumeraciones y metodos usando genericos. Luego, analizaremos como los
// tipos de datos genericos afectan el rendimiento del codigo.


fn mayor_i32(lista: &[i32]) -> {
    let mut mayor = lista[0];

    for &item in lista {
        if item > mayor {
            mayor = item;
        }
    }

    mayor
}

fn mayor<T>(lista &[T]) -> T {
    let mut mayor = lista[0];

    for &item in lista {
        if item > mayor {
            mayor = item;
        }
    }

    mayor
}

fn mayor_char(lista: &[char]) -> char {
    let mut mayor = lista[0];

    for &item in lista {
        if item > mayor {
            mayor = item;
        }
    }

    mayor
}

fn main() {

    let lista_numeros = vec![20, 45, 77, 13, 66];
    let resultado = mayor_i32(&lista_numeros);
    println!("El mayor numero de la lista es: {}", resultado);

    let lista_caracteres = vec!['m', 'y', 'a', 'd', 'f'];
    let resultado = mayor_char(&lista_caracteres)
    println!("El caracter mayor es: {}", resultado);


    // let lista_numeros = vec![20, 45, 77, 13, 66];
    // let resultado = mayor(&lista_numeros);
    // println!("El mayor numero de la lista es: {}", resultado);

    // let lista_caracteres = vec!['m', 'y', 'a', 'd', 'f'];
    // let resultado = mayor(&lista_caracteres);
    // println!("El caracter mayor es: {}", resultado);


    let entreo = Punto {
        x: 5,
        y: 17
    };

    let decimal = Punto {
        x: 20.0,
        y: 3.0
    };


    // let fallara = Punto {
    //     x: 20.0,
    //     y: 14 // error
    // };


    let ambos_enteros = Puntp { x: 3, y: 18 };
    let ambos_decimales = Punto { x: 2.5, y: 29.0 };
    let entero_y_decimal = PuntoDuo { x: 5, y: 27.0 };


    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }


    let punto_new = PuntoNew { x: 23.0, y: 14.0 };
    println!("punto.x = {}", punto.x());


    let punto1 = PuntoNewNew { x: 56, y: 23.0 };
    let punto2 = PuntoNewNew { x: "Hola amigos", y: 'z' };
    let punto3 = punto1.mezcla(punto2);
    println!("punto3.x = {}, punto3.y = {}", punto3.x, punto3.y);


}

struct PuntoNewNew<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Punto<X1, Y1> {
    fn mezcla<X2, Y2>(self, otro: PuntoNewNew<X2, Y2>) -> PuntoNewNew<X1, Y2> {
        PuntoNewNew {
            x: self.x,
            y: otro.y
        }
    }
}


impl PuntoNew<f32> {
    fn distancia_desde_origen(&self) -> 32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PuntoNew<T> {
    x: T,
    y: T,
}

impl<T> PuntoNew<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


struct Punto<T> {
    x: T,
    y: T,
}

struct PuntoDuo<T, U> {
    x: T,
    y: U,
}

// Redimiento de los tipos de datos genericos

// Seguro que te estas planteando si existe un costo de tiempo de ejecucion
// cuando usamos los parametros de tipo generico. La buena noticia es que Rust
// implementa genericos de tal manera que su codigo no se ejecuta mas lento
// usando tipos genericos que con tipos concretos.

// Rust logra esto mediante la monomorfizacion del codigo que usa genericos en
// tiempo de compilcaion. La monomorfizacion es el proceso de convertir codigo
// generico en codigo especifico al completar los tipos concretos que se usan
// cuando se compila.
