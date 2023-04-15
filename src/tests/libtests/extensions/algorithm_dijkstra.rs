use crate::extensions::algorithm_dijkstra::{Dijkstra, NonMax};

#[test]
fn test_btreemap_dijkstra() {
    use core::iter::once;
    use std::collections::BTreeMap;

    let g: BTreeMap<_, _> = vec![
        ((0, 1), 9),
        ((0, 2), 2),
        ((0, 4), 14),
        ((1, 3), 6),
        ((2, 3), 11),
        ((2, 4), 9),
        ((2, 5), 10),
        ((3, 5), 15),
        ((4, 5), 7),
        //
        ((1, 0), 9),
        ((2, 0), 2),
        ((4, 0), 14),
        ((3, 1), 6),
        ((3, 2), 11),
        ((4, 2), 9),
        ((5, 2), 10),
        ((5, 3), 15),
        ((5, 4), 7),
    ]
    .into_iter()
    .collect();

    let _d = Dijkstra::new(6, once((4, 0)), |n, l| {
        Some(g.range((n, 0)..(n + 1, 0)).map(move |n| (n.0 .1, l + n.1)))
    });
    let d = Dijkstra::new(6, once((4, 0)), |n, l| {
        Some(g.range((n, 0)..(n + 1, 0)).map(move |n| (n.0 .1, l + n.1)))
    });
    let _l = g.len();

    assert_eq!(d.nodes, &[4, 5, 2, 0, 1, 3]);
    assert_eq!(
        d.dists,
        &[Some(11), Some(20), Some(9), Some(20), Some(0), Some(7)]
    );
    assert_eq!(
        d.prevs,
        &[
            NonMax::from(2),
            NonMax::from(0),
            NonMax::from(4),
            NonMax::from(2),
            NonMax::none(),
            NonMax::from(4)
        ]
    );
}
