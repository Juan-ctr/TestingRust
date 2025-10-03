#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Graph {
    #[serde(default)]
    nodes: HashMap<u64, Node>,
}

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("el id {0} ya existe")]
    DuplicateId(u64),
}

impl Graph {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    /// Inserta un nodo si el `id` no existe.
    ///
    /// # Errors
    /// Retorna `GraphError::DuplicateId` si el `id` ya existe en el grafo.
    pub fn add_node(&mut self, id: u64, name: String) -> std::result::Result<(), GraphError> {
        if self.nodes.contains_key(&id) {
            return Err(GraphError::DuplicateId(id));
        }
        let node = Node { id, name };
        self.nodes.insert(id, node);
        Ok(())
    }

    pub fn list_nodes(&self) -> impl Iterator<Item = (&u64, &Node)> {
        self.nodes.iter()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn agregar_y_listar_uno() {
//         let mut g = Graph::new();
//         let inserted = g.add_node(1, "Alice");
//         assert!(inserted, "debería insertar un id nuevo");

//         let listed = g.list_nodes();
//         assert_eq!(listed.len(), 1);
//         assert_eq!(listed[0].id, 1);
//         assert_eq!(listed[0].name, "Alice");
//     }
// }
