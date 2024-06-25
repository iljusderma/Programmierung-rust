use std::cmp::Ordering;

// TODO: Write your implementation for "min" and "max" working with list of values

fn main() {
    // Checks for min()
    assert_eq!(min([25, -6, -2, 53]), -6);
    assert_eq!(min([563, 132, 746, 90]), 132);

    assert_eq!(min(['m', 'c', 'x']), 'c');
    assert_eq!(min(['9', '2', '3']), '2');

    // Checks for max()
    assert_eq!(max([25, -6, -2, 53]), 53);
    assert_eq!(max([563, 132, 746, 90]), 746);

    assert_eq!(max(['m', 'c', 'x']), 'x');
    assert_eq!(max(['9', '2', '3']), '9');
}