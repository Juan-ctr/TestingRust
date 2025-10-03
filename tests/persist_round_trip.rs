#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

use minigraph::graph::Graph;
use minigraph::store::GraphStore;
use tempfile::tempdir;

#[test]
fn persist_round_trip() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join(".minigraph").join("graph.json");
    let store = GraphStore::new(&path);

    let mut g: Graph = store.load()?;
    assert_eq!(g.clone().list_nodes().count(), 0);

    g.add_node(1, "Alice".to_string())?;
    store.save(&g)?;

    let g2 = store.load()?;
    let mut items = g2.list_nodes().collect::<Vec<_>>();
    items.sort_by_key(|(id, _)| **id);

    assert_eq!(items.len(),1);
    assert_eq!(*items[0].0,1);
    assert_eq!(items[0].1.name, "Alice");

    Ok(())
}
