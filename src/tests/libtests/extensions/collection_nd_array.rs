use crate::extensions::collection_nd_array::NdArray;
use crate::wrap::wrap;

#[test]
fn test_dimview2() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let view = NdArray::from(&arr[..], [3, 4]);

    assert_eq!(view.at(0)[&[0]], 1);
    assert_eq!(view.at(0)[&[1]], 2);
    assert_eq!(view.at(0)[&[2]], 3);
    assert_eq!(view.at(0)[&[3]], 4);
    assert_eq!(view.at(1)[&[0]], 5);
    assert_eq!(view.at(1)[&[1]], 6);
    assert_eq!(view.at(1)[&[2]], 7);
    assert_eq!(view.at(1)[&[3]], 8);
    assert_eq!(view.at(2)[&[0]], 9);
    assert_eq!(view.at(2)[&[1]], 10);
    assert_eq!(view.at(2)[&[2]], 11);
    assert_eq!(view.at(2)[&[3]], 12);

    assert_eq!(view[[0, 0]], 1);
    assert_eq!(view[[0, 1]], 2);
    assert_eq!(view[[0, 2]], 3);
    assert_eq!(view[[0, 3]], 4);
    assert_eq!(view[[1, 0]], 5);
    assert_eq!(view[[1, 1]], 6);
    assert_eq!(view[[1, 2]], 7);
    assert_eq!(view[[1, 3]], 8);
    assert_eq!(view[[2, 0]], 9);
    assert_eq!(view[[2, 1]], 10);
    assert_eq!(view[[2, 2]], 11);
    assert_eq!(view[[2, 3]], 12);

    assert_eq!(view.get([0, 0]), Some(&1));
    assert_eq!(view.get([(wrap(0usize) - wrap(1)).0, 0]), None);
    assert_eq!(view.get([0, (wrap(0usize) - wrap(1)).0]), None);
    assert_eq!(
        view.get([(wrap(0usize) - wrap(1)).0, (wrap(0usize) - wrap(1)).0]),
        None
    );
}

#[test]
fn test_dimview3_at() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view.at(0).at(0)[&[0]], 1);
    assert_eq!(view.at(0).at(0)[&[1]], 2);
    assert_eq!(view.at(0).at(1)[&[0]], 3);
    assert_eq!(view.at(0).at(1)[&[1]], 4);
    assert_eq!(view.at(0).at(2)[&[0]], 5);
    assert_eq!(view.at(0).at(2)[&[1]], 6);
    assert_eq!(view.at(1).at(0)[&[0]], 7);
    assert_eq!(view.at(1).at(0)[&[1]], 8);
    assert_eq!(view.at(1).at(1)[&[0]], 9);
    assert_eq!(view.at(1).at(1)[&[1]], 10);
    assert_eq!(view.at(1).at(2)[&[0]], 11);
    assert_eq!(view.at(1).at(2)[&[1]], 12);
    assert_eq!(view.at(2).at(0)[&[0]], 13);
    assert_eq!(view.at(2).at(0)[&[1]], 14);
    assert_eq!(view.at(2).at(1)[&[0]], 15);
    assert_eq!(view.at(2).at(1)[&[1]], 16);
    assert_eq!(view.at(2).at(2)[&[0]], 17);
    assert_eq!(view.at(2).at(2)[&[1]], 18);
    assert_eq!(view.at(3).at(0)[&[0]], 19);
    assert_eq!(view.at(3).at(0)[&[1]], 20);
    assert_eq!(view.at(3).at(1)[&[0]], 21);
    assert_eq!(view.at(3).at(1)[&[1]], 22);
    assert_eq!(view.at(3).at(2)[&[0]], 23);
    assert_eq!(view.at(3).at(2)[&[1]], 24);
}

#[test]
fn test_dimview3_index() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view[[0, 0, 0]], 1);
    assert_eq!(view[[0, 0, 1]], 2);
    assert_eq!(view[[0, 1, 0]], 3);
    assert_eq!(view[[0, 1, 1]], 4);
    assert_eq!(view[[0, 2, 0]], 5);
    assert_eq!(view[[0, 2, 1]], 6);
    assert_eq!(view[[1, 0, 0]], 7);
    assert_eq!(view[[1, 0, 1]], 8);
    assert_eq!(view[[1, 1, 0]], 9);
    assert_eq!(view[[1, 1, 1]], 10);
    assert_eq!(view[[1, 2, 0]], 11);
    assert_eq!(view[[1, 2, 1]], 12);
    assert_eq!(view[[2, 0, 0]], 13);
    assert_eq!(view[[2, 0, 1]], 14);
    assert_eq!(view[[2, 1, 0]], 15);
    assert_eq!(view[[2, 1, 1]], 16);
    assert_eq!(view[[2, 2, 0]], 17);
    assert_eq!(view[[2, 2, 1]], 18);
    assert_eq!(view[[3, 0, 0]], 19);
    assert_eq!(view[[3, 0, 1]], 20);
    assert_eq!(view[[3, 1, 0]], 21);
    assert_eq!(view[[3, 1, 1]], 22);
    assert_eq!(view[[3, 2, 0]], 23);
    assert_eq!(view[[3, 2, 1]], 24);
}

/*#[test]
fn test_dimview3_range() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);

    assert_eq!(view.range(1..3)[[0, 0, 0]], 7);
    assert_eq!(view.range(1..3)[[0, 0, 1]], 8);
    assert_eq!(view.range(1..3)[[0, 1, 0]], 9);
    assert_eq!(view.range(1..3)[[0, 1, 1]], 10);
    assert_eq!(view.range(1..3)[[0, 2, 0]], 11);
    assert_eq!(view.range(1..3)[[0, 2, 1]], 12);
    assert_eq!(view.range(1..3)[[1, 0, 0]], 13);
    assert_eq!(view.range(1..3)[[1, 0, 1]], 14);
    assert_eq!(view.range(1..3)[[1, 1, 0]], 15);
    assert_eq!(view.range(1..3)[[1, 1, 1]], 16);
    assert_eq!(view.range(1..3)[[1, 2, 0]], 17);
    assert_eq!(view.range(1..3)[[1, 2, 1]], 18);
}*/

#[test]
fn test_dimview3_mut() {
    let mut arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];

    let mut view = NdArray::from(&mut arr[..], [4, 3, 2]);

    view[[1, 1, 1]] = 100;
    view.at_mut(3).at_mut(2)[&[1]] = 200;

    assert_eq!(view.at(1).at(1)[&[1]], 100);
    assert_eq!(view[[3, 2, 1]], 200);
}

#[test]
#[should_panic(
    expected = "invalid slice length: the len is 12 but the product of dimensions is 16"
)]
fn test_dimview2_size_check() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let _arr = NdArray::from(&arr[..], [4, 4]);
}

#[test]
#[should_panic(
    expected = "failed: invalid slice length: the len is 24 but the product of dimensions is 36"
)]
fn test_dimview3_size_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let _arr = NdArray::from(&arr[..], [4, 3, 3]);
}

#[test]
#[should_panic(expected = "range end index 30 out of range for slice of length 24")]
fn test_dimview3_index1_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(4);
}

#[test]
#[should_panic(expected = "range end index 8 out of range for slice of length 6")]
fn test_dimview3_index2_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(3).at(3);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is [2] but the index is [2]")]
fn test_dimview3_index3_check() {
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let view = NdArray::from(&arr[..], [4, 3, 2]);
    let _ = view.at(2).at(2)[&[2]];
}
