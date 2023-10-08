pub use rooted_tree::*;
mod rooted_tree {
    use std::cmp::Ordering;
    use std::collections::BTreeSet;

    pub fn root_tree(
        par: usize,
        u: usize,
        lvl: usize,
        uvs: &BTreeSet<(usize, usize)>,
        pars: &mut Vec<usize>,
        lvls: &mut Vec<usize>,
    ) {
        pars[u] = par;
        lvls[u] = lvl;
        for &(_, v) in uvs.range((u, 0)..(u + 1, 0)) {
            if v != par {
                root_tree(u, v, lvl + 1, uvs, pars, lvls);
            }
        }
    }

    #[allow(clippy::similar_names)]
    pub fn rooted_tree_path<'a>(
        mut u: usize,
        mut v: usize,
        pars: &[usize],
        lvls: &[usize],
        upars: &'a mut Vec<usize>,
        vpars: &'a mut Vec<usize>,
    ) -> usize {
        unsafe { upars.set_len(0) };
        unsafe { vpars.set_len(0) };
        let ulvl = lvls[u];
        let vlvl = lvls[v];
        match ulvl.cmp(&vlvl) {
            Ordering::Less => {
                for _ in ulvl..vlvl {
                    vpars.push(v);
                    v = pars[v];
                }
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                for _ in vlvl..ulvl {
                    upars.push(u);
                    u = pars[u];
                }
            }
        }
        while u != v {
            vpars.push(v);
            upars.push(u);
            v = pars[v];
            u = pars[u];
        }
        u
    }
}
