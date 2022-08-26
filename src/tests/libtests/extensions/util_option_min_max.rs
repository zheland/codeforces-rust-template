use crate::extensions::util_option_min_max::OptionMinMax;

#[test]
fn test_option_ops() {
    assert_eq!(Some(3).omax(6), 6);
    assert_eq!(Some(7).omax(4), 7);
    assert_eq!(None.omax(5), 5);
    assert_eq!(None.omax(5), 5);

    assert_eq!(Some(3).omax(Some(6)), Some(6));
    assert_eq!(Some(7).omax(Some(4)), Some(7));
    assert_eq!(Some(3).omax(None), Some(3));
    assert_eq!(Some(7).omax(None), Some(7));
    assert_eq!(None::<i32>.omax(Some(6)), Some(6));
    assert_eq!(None::<i32>.omax(Some(4)), Some(4));
    assert_eq!(None::<i32>.omax(None), None);
    assert_eq!(None::<i32>.omax(None), None);

    assert_eq!(Some(3).omin(6), 3);
    assert_eq!(Some(7).omin(4), 4);
    assert_eq!(None.omin(5), 5);
    assert_eq!(None.omin(5), 5);

    assert_eq!(Some(3).omin(Some(6)), Some(3));
    assert_eq!(Some(7).omin(Some(4)), Some(4));
    assert_eq!(Some(3).omin(None), Some(3));
    assert_eq!(Some(7).omin(None), Some(7));
    assert_eq!(None::<i32>.omin(Some(6)), Some(6));
    assert_eq!(None::<i32>.omin(Some(4)), Some(4));
    assert_eq!(None::<i32>.omin(None), None);
    assert_eq!(None::<i32>.omin(None), None);
}
