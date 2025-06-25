# Minecraft Clone en Rust

Este proyecto es un clon de Minecraft escrito en Rust, utilizando `wgpu` para el renderizado. Sirve como un campo de pruebas para aprender sobre gráficos por computadora, desarrollo de juegos y el ecosistema de Rust.

## Características

*   **Renderizado 3D:** Utiliza `wgpu` para renderizar una escena 3D simple con un cubo giratorio.
*   **Superposición de Depuración:** Muestra una superposición de depuración (estilo F3) que se puede activar/desactivar con la tecla F3. La superposición muestra:
    *   FPS (Fotogramas por segundo)
    *   Uso de la CPU y nombre
    *   Uso de la GPU y nombre
    *   Uso de la RAM
    *   Memoria utilizada por el proceso
*   **Cámara 3D:** Una cámara simple que se puede mover y girar.

## Cómo Compilar y Ejecutar

1.  **Instalar Rust:** Si aún no lo has hecho, instala Rust desde [rustup.rs](https://rustup.rs/).
2.  **Clonar el repositorio:**
    ```bash
    git clone <URL_DEL_REPOSITORIO>
    cd minecraft-rust
    ```
3.  **Ejecutar la aplicación:**
    ```bash
    cargo run
    ```

## Dependencias

*   `wgpu`: Para el renderizado de gráficos 3D.
*   `winit`: Para la gestión de ventanas y eventos.
*   `egui`: Para la creación de la interfaz gráfica de usuario (GUI) de la superposición de depuración.
*   `egui-wgpu`: Para la integración de `egui` con `wgpu`.
*   `egui-winit`: Para la integración de `egui` con `winit`.
*   `cgmath`: Para las operaciones matemáticas de vectores y matrices.
*   `sysinfo`: Para obtener información del sistema (CPU, RAM, etc.).
*   `log` y `env_logger`: Para el registro de mensajes en la consola.
*   `tokio`: Para el tiempo de ejecución asíncrono.

## Documentación del Código

Este proyecto utiliza comentarios de documentación en el código. Para generar la documentación, ejecuta:

```bash
cargo doc --open
```

Esto abrirá la documentación en tu navegador web.
