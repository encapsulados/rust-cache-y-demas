use std::future::Future;
use futures::{future};

struct CódigoDeErrorPersonalizado {
    mensaje_de_error: String,
}

impl CódigoDeErrorPersonalizado {
    fn nuevo(mensaje: String) -> Self {
        CódigoDeErrorPersonalizado {
            mensaje_de_error: mensaje,
        }
    }
}

mod servicio_de_datos {
    use crate::http::CódigoDeErrorPersonalizado;

    // dado "foo" -> devolver "bar"
    // otra cosa  -> devolver un error
    pub async fn traer_datos(palabra_clave: String) -> Result<String, CódigoDeErrorPersonalizado> {
        match palabra_clave.as_str() {
            "foo" => Ok("bar".to_string()),
            _ => Err(CódigoDeErrorPersonalizado::nuevo(palabra_clave))
        }
    }
}

mod servicio_http {
    use crate::http::servicio_de_datos;

    pub async fn procesar_consulta(palabra_clave: String) -> Result<i32, i32> {
        servicio_de_datos::traer_datos(palabra_clave)
            .await
            .map(|_| 200)
            .map_err(|_| 500)
    }
}

pub async fn main() {
    println!("servicio http");

    let textos = vec!["luigi mangione", "foo", "ay", "otra cosa", "foo"];

    let futuros = textos.iter()
        .map(|texto| depurar(servicio_http::procesar_consulta, texto.to_string()))
        .map(|futuro| async { imprimir_respuesta(futuro.await) });

    future::join_all(futuros).await;
}

async fn depurar<F, Salida>(f: F, parámetro: String) -> Result<i32, i32>
where
    F: Fn(String) -> Salida,
    Salida: Future<Output=Result<i32, i32>>,
{
    use tokio::time::{sleep, Duration};

    println!("procesando <<{}>>", parámetro);

    let duración = Duration::from_millis((parámetro.len() * 200) as u64);
    sleep(duración).await;
    let resultado = f(parámetro.clone()).await;

    println!();
    println!("procesado <<{}>> en {:?}", parámetro, duración);

    resultado
}

fn imprimir_respuesta(resultado: Result<i32, i32>) {
    match resultado {
        Ok(código) => {
            println!("INFO  | Código HTTP: {}", código);
        }
        Err(código) => {
            println!("ERROR | Código HTTP: {}", código);
        }
    }
}
