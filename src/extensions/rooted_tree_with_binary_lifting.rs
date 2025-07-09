pub use rooted_tree_with_binary_lifting::*;
mod rooted_tree_with_binary_lifting {
    use core::cmp::Ordering;
    use std::collections::BTreeSet;

    use crate::extensions::math_log2::Log2;

    pub fn root_tree_with_binary_lifting<const N: usize>(
        mut par: [usize; N],
        u: usize,
        lvl: usize,
        uvs: &BTreeSet<(usize, usize)>,
        pars: &mut Vec<[usize; N]>,
        lvls: &mut Vec<usize>,
    ) {
        let prev = par[0];
        pars[u] = par;
        lvls[u] = lvl;
        par[0] = u;
        #[allow(clippy::needless_range_loop)]
        for j in 1..=(lvl.trailing_zeros() as usize).min(N - 1) {
            par[j] = u;
        }
        for &(_, v) in uvs.range((u, 0)..(u + 1, 0)) {
            if v != prev {
                root_tree_with_binary_lifting(par, v, lvl + 1, uvs, pars, lvls);
            }
        }
    }

    #[allow(clippy::similar_names)]
    #[must_use]
    pub fn rooted_tree_with_binary_lifting_lca<const N: usize>(
        mut u: usize,
        mut v: usize,
        pars: &[[usize; N]],
        lvls: &[usize],
    ) -> usize {
        let mut ulvl = lvls[u];
        let mut vlvl = lvls[v];
        match ulvl.cmp(&vlvl) {
            Ordering::Less => {
                while ulvl < vlvl {
                    let diff = vlvl - ulvl;
                    let pl2 = (log2_floor(diff) as usize).min(N - 1);
                    v = pars[v][pl2];
                    vlvl = lvls[v];
                }
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                while vlvl < ulvl {
                    let diff = ulvl - vlvl;
                    let pl2 = (log2_floor(diff) as usize).min(N - 1);
                    u = pars[u][pl2];
                    ulvl = lvls[u];
                }
            }
        }

        let mut lvl = lvls[u];

        'a: while u != v {
            let xj = log2_floor(lvl) as usize;
            for j in (1..xj).rev() {
                let up = pars[u][j];
                let vp = pars[v][j];
                if pars[u][j] != pars[v][j] {
                    u = up;
                    v = vp;
                    lvl = lvls[u];
                    continue 'a;
                }
            }
            u = pars[u][0];
            v = pars[v][0];
        }
        u
    }

    fn log2_floor(value: usize) -> u32 {
        assert!(value > 0);
        usize::BITS - value.leading_zeros() - 1
    }
}
