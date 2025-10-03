#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use minigraph::graph::{Graph, GraphError};
use minigraph::store::GraphStore;

/// Por el momento solo guarda grafos en un json y los lista
///
/// # Errors
/// Devuelve error si alguna de las operaciones falla
fn main() -> Result<()> {
    let path: PathBuf = PathBuf::from(".minigraph").join("graph.json");
    let store = GraphStore::new(&path);

    let mut graph: Graph = store
        .load()
        .with_context(|| format!("error al cargar estado desde {:?}", path.display()))?;

    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "add-node" => {
                if i + 2 >= args.len() {
                    bail!("uso: add-node <id> <name>");
                }
                let id_str = &args[i + 1];
                let name = args[i + 2].clone();

                let id = id_str
                    .parse::<u64>()
                    .with_context(|| format!("id inválido: {id_str}"))?;
                match graph.add_node(id, name) {
                    Ok(()) => { /* Ok */ }
                    Err(GraphError::DuplicateId(dup)) => {
                        bail!("no se pudo agregar: el id {dup} ya existe");
                    }
                }
                i += 3;
            }
            "list-nodes" => {
                for (id, node) in graph.list_nodes() {
                    println!("{id}\t{}", node.name);
                }
                i += 1;
            }
            other => {
                bail!("comando desconocido: {other}");
            }
        }
    }
    store
        .save(&graph)
        .with_context(|| format!("error al guardar estado en {}", path.display()))?;
    Ok(())
}

fn print_usage() {
    eprintln!(
        "uso:
  minigraph-cli add-node <id> <name> [list-nodes]
  minigraph-cli list-nodes

ejemplos:
  cargo run -- add-node 1 Alice list-nodes
  cargo run -- list-nodes"
    );
}
