use crate::extensions::util_solve_ax_cong_b_mod_p::solve_ax_cong_b_mod_p;

#[test]
fn test_solve_ax_cong_b_mod_p() {
    assert_eq!(solve_ax_cong_b_mod_p(3_u32, 4, 11).map(|v| v % 11), Some(5));
    assert_eq!(solve_ax_cong_b_mod_p(120_u32, 51, 12).map(|v| v % 12), None);
    assert_eq!(
        solve_ax_cong_b_mod_p(4270_u32, 1540, 10605).map(|v| v % 10605),
        Some(5368)
    );
}
