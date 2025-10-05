#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

use minigraph::graph::{Graph, GraphError};

#[test]
fn agrega_y_lista_relaciones() -> anyhow::Result<()> {
    let mut g = Graph::new();
    g.add_node(1, "A".to_string())?;
    g.add_node(2, "B".to_string())?;
    g.add_node(3, "C".to_string())?;

    g.add_edge(1, 2, "amigo".to_string())?;
    g.add_edge(1, 3, "primo".to_string())?;

    let edges = g.list_edges().collect::<Vec<_>>();
    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].from, 1);
    assert_eq!(edges[0].to, 2);
    assert_eq!(edges[0].kind, "amigo");
    assert_eq!(edges[1].from, 1);
    assert_eq!(edges[1].to, 3);
    assert_eq!(edges[1].kind, "primo");

    Ok(())
}

#[test]
fn falla_si_nodo_no_existe_en_add_edge() {
    let mut g = Graph::new();
    g.add_node(1, "A".to_string()).unwrap();

    let err = g.add_edge(1, 999, "x".to_string()).unwrap_err();
    match err {
        GraphError::MissingNode(id) => assert_eq!(id, 999),
        _ => panic!("se esperaba un MissingNode(999)"),
    }
}
