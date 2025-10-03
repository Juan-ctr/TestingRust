#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use serde_json::{Deserializer, Serializer};
use tempfile::NamedTempFile;

use crate::graph::Graph;

pub struct GraphStore {
    path: PathBuf,
}

impl GraphStore {
    #[must_use]
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Carga el grafo desde disco; si no existe el archivo, devuelve un grafo vacío.
    ///
    /// # Errors
    /// Devuelve error si el archivo existe pero no puede abrirse, si hay errores de E/S,
    /// o si el contenido no es un JSON válido para `Graph`.
    pub fn load(&self) -> Result<Graph> {
        if !self.path.exists() {
            return Ok(Graph::new());
        }

        let file =
            File::open(&self.path).with_context(|| format!("no se pudo abrir{:?}", self.path.display()))?;

        let reader = BufReader::new(file);
        let mut de = Deserializer::from_reader(reader);
        let graph = Graph::deserialize(&mut de)
            .with_context(|| format!("no se pudo deserializar {:?}", self.path.display()))?;
        Ok(graph)
    }

    /// Guarda el grafo en disco usando escritura atómica (archivo temporal + persist).
    ///
    /// # Errors
    /// Devuelve error si falla la creación del directorio padre, la escritura/serialización
    /// del JSON o la operación de `persist` hacia la ruta de destino.
    pub fn save(&self, graph: &Graph) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            create_dir_all(parent)
                .with_context(|| format!("no se pudo crear el dir {:?}", parent.display()))?;
        }

        let parent = self
            .path
            .parent()
            .map_or_else(|| Path::new(".").to_path_buf(), Path::to_path_buf);

        let tmp = NamedTempFile::new_in(&parent)
            .with_context(|| format!("no se pudo crear archivo temporal en {:?}", parent.display()))?;
        {
            let writer = BufWriter::new(&tmp);
            let formatter = PrettyFormatter::with_indent(b" ");
            let mut ser = Serializer::with_formatter(writer, formatter);
            graph
                .serialize(&mut ser)
                .context("no se pudo serializar el grafo a JSON")?;
        }

        tmp.persist(&self.path)
            .with_context(|| format!("no se pudo persistir en{:?}", self.path.display()))?;

        Ok(())
    }
}
