use crate::extensions::solve_ax_plus_by_eq_gcd_ab::solve_ax_plus_by_eq_gcd_ab;

#[test]
fn test_solve_ax_plus_by_eq_gcd_ab() {
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(0, 0), (0, 1, 0));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(10, 0), (10, 1, 0));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(0, 10), (10, 0, 1));

    assert_eq!(solve_ax_plus_by_eq_gcd_ab(1, 1), (1, 0, 1));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(2, 3), (1, -1, 1));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(6, 15), (3, -2, 1));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(30, 105), (15, -3, 1));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(123, 234), (3, -19, 10));
    assert_eq!(solve_ax_plus_by_eq_gcd_ab(321, 321), (321, 0, 1));
}
