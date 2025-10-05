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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub kind: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Graph {
    #[serde(default)]
    nodes: HashMap<u64, Node>,
    #[serde(default)]
    edges: Vec<Edge>,
}

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("el id {0} ya existe")]
    DuplicateId(u64),
    #[error("no existe el nodo con id {0}")]
    MissingNode(u64),
}

impl Graph {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
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

    /// Agrega una relación dirigida  `from -> to` del tipo `kind`.
    /// Falla si `from` o `to` no existen.
    ///
    /// # Errors
    /// Retorna `GraphError::MissingNode(id)` si alguno de los nodos no existe.
    pub fn add_edge(&mut self, from: u64, to: u64, kind: String) -> Result<(), GraphError> {
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::MissingNode(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::MissingNode(to));
        }
        self.edges.push(Edge { from, to, kind });
        Ok(())
    }

    pub fn list_edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    #[cfg(test)]
    pub(crate) fn edges_len(&self) -> usize {
        self.edges.len()
    }
}
