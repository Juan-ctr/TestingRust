MiniGraph

Pequeño grafo en Rust con CLI. Permite crear nodos, listarlos y persistir el estado entre ejecuciones mediante un archivo JSON con escritura atómica. 
La idea es practicar el lenguaje Rust.

Características

Núcleo (lib) separado del binario (CLI).

Comandos:

add-node <id> <name>

list-nodes

Persistencia local en ./.minigraph/graph.json (JSON + escritura atómica).

1 test de persistencia ida/vuelta.

0 warnings con clippy (pedantic) y unsafe prohibido.

Requisitos

Rust/Cargo ≥ 1.90.0

Toolchain instalado con rustup.

Instalación
```powershell 
# Clonar (ejemplo)
git clone <url-del-repo> minigraph
cd minigraph

# Formatear + análisis estático
cargo fmt
cargo clippy -- -D warnings

# Ejecutar tests
cargo test
```

Desarrollo

Comandos útiles:
```powershell
# Verificar compilación
cargo check

# Ejecutar en modo dev
cargo run --bin minigraph-cli -- <comandos>

# Formateo y análisis
cargo fmt
cargo clippy -- -D warnings

# Tests
cargo test
```