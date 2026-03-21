# MyP-Proyecto1

Aplicación tipo cliente/servidor de chat con el protocolo TCP correspondiente al primer proyecto de la materia de Modelado y Programación. 
La aplicación está implementada con el lenguage de programación Rust y con programación asíncrona (Tokio). 

Este proyecto tiene dos ejecutables (servidor y cliente) y la biblioteca/lib common, por lo que se ejecutan por separado aunque el cliente necesita obligatoriamente que el servidor esté activo para tener a donde conectarse.

El ip y puerto son elegibles si se usa cargo, en cuyo caso se pasan como argumentos al momento de ejecutar los crates, si no se especifican argumentos cliente y servidor utilizan la ip `127.0.0.1` y el puerto `1234` por omisión.

---

## Requisitos

### Opción 1

* Docker

### Opción 2

* Rust
* Cargo

---

## Uso con Docker

### Construir el servidor:

```bash
docker build -t servidor -f servidor/Dockerfile .
```

### Construir el cliente:

```bash
docker build -t cliente -f cliente/Dockerfile .
```

---

### Ejecutar el servidor:

```bash
docker run -p 1234:1234 servidor
```

> Nota: 1234:1234 es un ejemplo esto se puede cambiar al puerto que sea.

### Ejecutar el cliente:

```bash
docker run -it cliente 172.17.0.1 1234
```

> Nota: la ip es la de docker e igualmente el puerto depende del puerto definido en el servidor.

---

## Uso sin Docker

### Ejecutar servidor

```bash
cargo run -p servidor
cargo run -p servidor 1234
```

> Nota: Puede pasarse o no el puerto donde se va a ejecutar (ambos ejemplos son válidos), en caso de que no se pase puerto se usará el puerto 1234 por omisión.

---

### Ejecutar cliente

```bash
cargo run -p cliente
cargo run -p cliente 127.0.0.1 1234
```

> Nota: Pueden o no pasarse argumentos (ip y puerto) en caso de que no se pasen argumentos se utilizará 127.0.0.1 como ip y puerto 1234 como valores por omisión.

---

## Uso del cliente

El cliente es interactivo leyendo de la entrada estándar (terminal) donde acepta las siguientes instrucciones (las instrucciones pueden ser cualquiera de los 2 que están separados por "/") (la diagonal no se pone, se utiliza uno u otro comando. Ejemplo: id iker). Los argumentos se separan por espacios. En caso de que la instrucción sea un mensaje el mensaje siempre será el último argumento para aceptar todos los espacios necesarios en el mensaje. Si se quiere invitar una lista de usuarios (aunque sea un único cliente) los usuarios van entre corchetes y separados por comas. Algunas instrucciones reciben argumentos y otras no, en caso de que se utilize una intrucción con argumentos sin haberlos pasado, el cliente indicará que la instrucción está incompleta y no realizará nada. Las instrucciones pueden escribirse en mayúsculas o minúsculas con o sin separación por medio de "_", pero no se pueden separar por espacios porque eso se utiliza como separador de los argumentos.

A continuación se muestran con ejemplos cada una de las posibles acciones:

```bash
identificarse/id iker
```

```bash
cambiarestado/ce away
```

```bash
listausuarios/lu
```

```bash
textoprivado/tpr daniel Hola Daniel, como estás?
```

```bash
textopublico/tpu Hola a todos.
```

```bash
creacuarto/cc sala1
```

```bash
invitacuarto/ic sala1 [iker, daniel]
```

```bash
unirsecuarto/uc sala1
```

```bash
usuarioscuarto/usrc sala1
```

```bash
textocuarto/tc sala1 Hola a toda la sala1
```

```bash
abandonacuarto/ac sala1
```

```bash
desconectarse/desc
```

---

## Notas importantes:

* El cliente termina al escribir `exit` o `desconectarse`.
* Si no se pasan argumentos (corriéndolo directamente con cargo), se usan valores por defecto:
* En caso de que el cliente quiera realizar una acción sin previa identificación el servidor desconectará al cliente.

  * IP: `127.0.0.1`
  * Puerto: `1234`

---

## Test

Para ejecutar las pruebas se usa:

```bash
cargo test
```

---

## Documentación

La documentación del proyecto se genera y abre con:

```bash
cargo doc --open
```

---
