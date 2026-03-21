use std::collections::{LinkedList};
use common::acciones_cliente::AccionCliente;
use common::status::Status;

/// Determina la acción que el cliente desea realizar a partir de una línea de entrada.
///
/// # Descripción
/// - Recibe una línea de texto ingresada por el usuario.
/// - Valida y separa la instrucción y sus argumentos.
/// - Construye una instancia de `AccionCliente` correspondiente.
///
/// # Regresa
/// Una variante (struct) de `AccionCliente` que representa la acción solicitada.
/// Si ocurre algún error en la validación, retorna `AccionInvalida`.
pub fn determinar_accion(linea: String) -> AccionCliente {

    let mut args: Vec<String> = Vec::new();
    
    match verifica_linea(linea) {
        Ok(a) => {
            args = a;
        },
        Err(e) => println!("Error: {}", e),
    };

    //ya sabemos que si hay elemento 0 por toda la validación previa, el match es obligado por el lenguage
    let instruccion = match args.get(0) {
        Some(i) => i,
        None => "",
    };
    
    let mut accion_struct: AccionCliente = AccionCliente::AccionInvalida {  };

    //teoricamente los casos de None no deberían de pasar JAMAS por la validación hecha previamente
    match instruccion {
        "identificarse" => {
            let nombre: &str = match args.get(1) {
                Some(n) => n,
                None => "",
            };
            accion_struct = AccionCliente::Identificarse { nombre: (nombre.to_string()) };
        },
        "cambiarestado" => {
            let nuevo_status: &str = match args.get(1) {
                Some(ns) => ns,
                None => "",
            };
            let status_normalizado = nuevo_status.to_lowercase()
                .replace("\n", "")
                .replace("\r", "")
                .replace(" ", "")
                .replace(":", "")
                .replace("_", "");
            if status_normalizado == "active" {
                accion_struct = AccionCliente::CambiarEstado { nuevo_status: (Status::ACTIVE) };
            }else if status_normalizado == "busy" {
                accion_struct = AccionCliente::CambiarEstado { nuevo_status: (Status::BUSY) };
            }else if status_normalizado == "away" {
                accion_struct = AccionCliente::CambiarEstado { nuevo_status: (Status::AWAY) };
            }
        },
        "listausuarios" => {
            accion_struct = AccionCliente::PedirListaUsuarios {  };
        },
        "textoprivado" => {
            let destinatario: &str = match args.get(1) {
                Some(d) => d,
                None => "",
            };
            let texto: &str = match args.get(2) {
                Some(t) => t,
                None => "",
            };
            accion_struct = AccionCliente::MandaTextoPrivado {
                texto: (texto.to_string()),
                destinatario: (destinatario.to_string())
            };
        },
        "textopublico" => {
            let texto: &str = match args.get(1) {
                Some(t) => t,
                None => "",
            };
            accion_struct = AccionCliente::MandaTextoPublico { texto: (texto.to_string()) };
        },
        "creacuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };
            accion_struct = AccionCliente::CreaCuarto { nombre_cuarto: (nombre_cuarto.to_string()) };
        },
        //-------------------ARREGLAR ESTE CASO---------------------------------------------
        "invitacuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };

            let nombres_usuarios: LinkedList<String> = args
                .iter()
                .skip(2)
                .cloned()
                .collect();

            accion_struct = AccionCliente::InvitaUsuariosCuarto {
                nombre_cuarto: (nombre_cuarto.to_string()),
                usuarios: (nombres_usuarios),
            };
        },
        "unirsecuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };
            accion_struct = AccionCliente::UnirseCuarto { nombre_cuarto: (nombre_cuarto.to_string()) };
        },
        "usuarioscuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };
            accion_struct = AccionCliente::PedirUsuariosCuarto { nombre_cuarto: (nombre_cuarto.to_string()) };
        },
        "textocuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };
            let texto: &str = match args.get(2) {
                Some(t) => t,
                None => "",
            };
            accion_struct = AccionCliente::MandaTextoCuarto { nombre_cuarto: (nombre_cuarto.to_string()), texto: (texto.to_string()) };
        },
        "abandonacuarto" => {
            let nombre_cuarto: &str = match args.get(1) {
                Some(nc) => nc,
                None => "",
            };
            accion_struct = AccionCliente::AbandonaCuarto { nombre_cuarto: (nombre_cuarto.to_string()) };
        },
        "desconectarse" => {
            accion_struct = AccionCliente::Desconectarse {  };
        },
        _ => {
            accion_struct = AccionCliente::AccionInvalida {  };
        },
    }
    return accion_struct;
}

/// Verifica y procesa la línea ingresada por el usuario.
///
/// # Descripción
/// - Divide la línea en instrucción y argumentos.
/// - Valida que la instrucción sea correcta.
/// - Verifica que los argumentos correspondan a dicha instrucción.
/// - Normaliza y organiza los datos en un vector.
///
/// # Regresa
/// Un `Vec<String>` donde:
/// - El primer elemento es la instrucción.
/// - Los siguientes elementos son los argumentos procesados.
///
/// # Errores
/// Regresa un error si:
/// - No se encuentra una instrucción.
/// - La instrucción es inválida.
/// - Los argumentos no cumplen con el formato esperado.
fn verifica_linea(linea: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let palabras: Vec<&str> = linea.splitn(3, ' ').collect();

    let arg: bool = match palabras.get(0) {
        Some(_) => true,
        None => false,
    };

    if arg == false {
        return Err(String::from("No se encontró la instrucción a ejecutar").into());
    }

    //aqui ya sabemos que hay argumento por lo cual none jamás debería ocurrir
    let accion = match palabras.get(0) {
        Some(a) => a,
        None => "",
    };
    
    let instruccion = verifica_instruccion(accion.to_string())?;
    let mut args: Vec<String> = Vec::new();
    
    if instruccion == "listausuarios" || instruccion == "desconectarse" {

        //dividimos en 2 en el caso de que sea un texto público para que todo lo que venga después de la instrucción lo tome como argumento
    }else if instruccion == "textopublico" {
        let palabras2: Vec<&str> = linea.splitn(2, ' ').collect();
        args = verifica_argumentos(palabras2, instruccion.clone())?;
    }else {
        args = verifica_argumentos(palabras, instruccion.clone())?;
    }

    //vector final con toda la info refinada
    let mut instrucciones: Vec<String> = Vec::new();
    //agregamos la acción a realizar como primer elemento del vector
    instrucciones.push(instruccion.clone());

    //pasamos el segundo argumento de invita cuarto (usuarios a la función que regresa los usuarios en forma de vector)
    //al final en este caso se regresará un vector donde el primer elemento es la instrucción, el segundo es el nombre del cuarto y los demas son los strings de usuarios
    if instruccion == "invitacuarto" {
        let arg_users: String = match args.get(1) {
            Some(u) => u.to_string(),
            None => "".to_string(),
        };
        
        let usuarios: Vec<String> = saca_usuarios(arg_users)?;

        //nuevamente, teóricamente jamás debería de ocurrir None por toda la validación previa
        instrucciones.push(match args.get(0) {
            Some(nombre_cuarto) => nombre_cuarto.to_string(),
            None => "".to_string(),
        });
        
        for i in usuarios {
            instrucciones.push(i);
        }
    }else {
        if instruccion != "listausuarios" && instruccion != "desconectarse" {
            for i in args {
                instrucciones.push(i);
            } 
        }
    }
    //los demás elementos del vector son los argumentos de la acción a realizar (el texto es lo último)
    return Ok(instrucciones);
}

/// Verifica y procesa la línea ingresada por el usuario.
///
/// # Descripción
/// - Divide la línea en instrucción y argumentos.
/// - Valida que la instrucción sea correcta.
/// - Verifica que los argumentos correspondan a dicha instrucción.
/// - Normaliza y organiza los datos en un vector.
///
/// # Regresa
/// Un `Vec<String>` donde:
/// - El primer elemento es la instrucción.
/// - Los siguientes elementos son los argumentos procesados.
///
/// # Errores
/// Regresa un error si:
/// - No se encuentra una instrucción.
/// - La instrucción es inválida.
/// - Los argumentos no cumplen con el formato esperado.
fn verifica_instruccion(linea: String) -> Result<String, Box<dyn std::error::Error>> {

    let accion_normalizado = linea.to_lowercase()
        .replace("\n", "")
        .replace("\r", "")
        .replace(" ", "")
        .replace(":", "")
        .replace("_", "");

    let a = accion_normalizado.as_str();

    match a {
        "identificarse" | "id" => {
            return Ok("identificarse".to_string());
        },
        "cambiarestado" | "ce" => {
            return Ok("cambiarestado".to_string());
        },
        "listausuarios" | "lu" => {
            return Ok("listausuarios".to_string());
        },
        "textoprivado" | "tpr" => {
            return Ok("textoprivado".to_string());
        },
        "textopublico" | "tpu" => {
            return Ok("textopublico".to_string());
        },
        "creacuarto" | "cc" => {
            return Ok("creacuarto".to_string());
        },
        "invitacuarto" | "ic" => {
            return Ok("invitacuarto".to_string());
        },
        "unirsecuarto" | "uc" => {
            return Ok("unirsecuarto".to_string());
        },
        "usuarioscuarto" | "usrc" => {
            return Ok("usuarioscuarto".to_string());
        },
        "textocuarto" | "tc" => {
            return Ok("textocuarto".to_string());
        },
        "abandonacuarto" | "ac" => {
            return Ok("abandonacuarto".to_string());
        },
        "desconectarse" | "desc" => {
            return Ok("desconectarse".to_string());
        },
        _ => {
            return Err(String::from("Instrucción inválida.").into());
        }
    }
}

/// Verifica y procesa la línea ingresada por el usuario.
///
/// # Descripción
/// - Divide la línea en instrucción y argumentos.
/// - Valida que la instrucción sea correcta.
/// - Verifica que los argumentos correspondan a dicha instrucción.
/// - Normaliza y organiza los datos en un vector.
///
/// # Regresa
/// Un `Vec<String>` donde:
/// - El primer elemento es la instrucción.
/// - Los siguientes elementos son los argumentos procesados.
///
/// # Errores
/// Regresa un error si:
/// - No se encuentra una instrucción.
/// - La instrucción es inválida.
/// - Los argumentos no cumplen con el formato esperado.
fn verifica_argumentos(args: Vec<&str>, instruccion: String) -> Result<Vec<String>,Box<dyn std::error::Error>> {

    let mut argumentos: Vec<String> = Vec::new();
    
    match instruccion.as_str() {
        "identificarse" => {
            match args.get(1) {
                Some(nombre) => {
                    //determina si el usuario cumple con el requisito de máximo 8 caracteres
                    if nombre.chars().count() > 9 {
                        return Err(String::from("No se permiten nombres de usuarios de más de 8 caracteres.").into());
                    }             
                    argumentos.push(nombre.to_string().replace("\n", ""));
                    return Ok(argumentos);
                }
                None => Err(String::from("Se necesita un nombre para identificarse.").into()),
            }
        },
        "cambiarestado" => {
            match args.get(1) {
                Some(estado) => {
                    argumentos.push(estado.to_string().replace("\n", ""));
                    return Ok(argumentos);
                }
                None => Err(String::from("Se necesita un estado.").into()),
            }
        },
        "textoprivado" => {
            let mut username: String = String::new();
            let mut mensaje: String = String::new();
            
            let arg1: bool = match args.get(1) {
                Some(destinatario) => {
                    username = destinatario.to_string();
                    true
                },
                None => false,
            };
            let arg2: bool = match args.get(2) {
                Some(texto) => {
                    mensaje = texto.to_string();
                    true
                },
                None => false,
            };
            if arg1 == false || arg2 == false {
                return Err(String::from("Se necesitan dos argumentos para esta instrucción.").into());
            }else {
                argumentos.push(username);
                argumentos.push(mensaje.replace("\n", ""));
                return Ok(argumentos);
            }
            
        },
        "textopublico" => {
            match args.get(1) {
                Some(texto) => {
                    argumentos.push(texto.to_string().replace("\n", ""));
                    return Ok(argumentos);
                },
                None => Err(String::from("Se necesita un texto a enviar.").into()),
            }
        },
        "creacuarto" => {
            match args.get(1) {
                Some(nombre_cuarto) => {
                    //checa que el número de caracteres del cuarto no sobrepase los 16
                    if nombre_cuarto.chars().count() > 17 {
                        return Err(String::from("No se permiten nombres de cuarto de más de 16 caracteres.").into());
                    }
                    argumentos.push(nombre_cuarto.to_string().replace("\n", ""));
                    return Ok(argumentos);
                },
                None => Err(String::from("Se necesita el nombre de un cuarto.").into()),
            }
        },
        //Como me van a pasar una lista este caso en específico está en proceso determinar como separar los usuarios que me envien (probablemente otra función auxiliar)
        "invitacuarto" => {
            let mut roomname: String = String::new();
            let mut users: String = String::new();
            
            let arg1: bool = match args.get(1) {
                Some(nombre_cuarto) => {
                    roomname = nombre_cuarto.to_string();
                    true
                },
                None => false,
            };
            let arg2: bool = match args.get(2) {
                Some(usuarios) => {
                    users = usuarios.to_string();
                    true
                },
                None => false,
            };
            if arg1 == false || arg2 == false {
                return Err(String::from("Se necesitan dos argumentos para esta instrucción (uno debe ser una lista).").into());
            }else {
                argumentos.push(roomname);
                argumentos.push(users.replace("\n", ""));
                return Ok(argumentos);
            }
        },
        "unirsecuarto" => {
            match args.get(1) {
                Some(nombre_cuarto) => {
                    argumentos.push(nombre_cuarto.to_string().replace("\n", ""));
                    return Ok(argumentos);
                },
                None => Err(String::from("Se necesita el nombre de un cuarto.").into()),
            }
        },
        "usuarioscuarto" => {
            match args.get(1) {
                Some(nombre_cuarto) => {
                    argumentos.push(nombre_cuarto.to_string().replace("\n", ""));
                    return Ok(argumentos);
                },
                None => Err(String::from("Se necesita el nombre de un cuarto.").into()),
            }
        },
        "textocuarto" => {
            let mut roomname: String = String::new();
            let mut mensaje: String = String::new();
            
            let arg1: bool = match args.get(1) {
                Some(nombre_cuarto) => {
                    roomname = nombre_cuarto.to_string();
                    true
                },
                None => false,
            };
            let arg2: bool = match args.get(2) {
                Some(texto) => {
                    mensaje = texto.to_string();
                    true
                },
                None => false,
            };
            if arg1 == false && arg2 == false {
                return Err(String::from("Se necesitan dos argumentos para esta instrucción.").into());
            }else {
                argumentos.push(roomname);
                argumentos.push(mensaje.replace("\n", ""));
                return Ok(argumentos);
            }
        },
        "abandonacuarto" => {
            match args.get(1) {
                Some(nombre_cuarto) => {
                    argumentos.push(nombre_cuarto.to_string().replace("\n", ""));
                    return Ok(argumentos);
                },
                None => Err(String::from("Se necesita el nombre de un cuarto.").into()),
            }
        },
        _ => return Err(String::from("Instrucción inválida.").into()),
    }
    
}

/// Extrae una lista de usuarios a partir de una cadena con formato de lista.
///
/// # Descripción
/// - Espera una cadena con usuarios entre corchetes (`[ ]`).
/// - Separa los elementos por comas.
/// - Elimina espacios innecesarios.
///
/// # Regresa
/// Un `Vec<String>` con los nombres de usuario.
///
/// # Errores
/// Regresa un error si:
/// - No se encuentran corchetes en la cadena.
fn saca_usuarios(argumentos: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    if !argumentos.contains('[') || !argumentos.contains(']') {
        return Err("Se necesitan corchetes para indicar la lista de usuarios".into());
    }

    let linea_sin_corchetes = argumentos
        .replace("[", "")
        .replace("]", "");
    
    let nombres: Vec<&str> = linea_sin_corchetes.split(',').collect();

    let usernames: Vec<String> = nombres.iter()
        .map(|s| s.trim().to_string())
        .collect();
    
    return Ok(usernames);
}
