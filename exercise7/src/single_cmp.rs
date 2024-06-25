use std::cmp::Ordering;

// TODO: Write your implementation for "min" and "max" working with single values
fn min<T: std::cmp::Ord>(a: T, b: T) -> T {
    match a.cmp(&b){
        Ordering::Greater | Ordering::Equal => b,
        Ordering::Less => a
    }
}

fn max<T: std::cmp::Ord>(a: T, b: T) -> T {
    match a.cmp(&b){
        Ordering::Greater | Ordering::Equal => a,
        Ordering::Less => b
    }
}

fn main() {
    // Checks for min()
    assert_eq!(min(25, -6), -6);
    assert_eq!(min(563, 132), 132);

    assert_eq!(min('a', 'y'), 'a');
    assert_eq!(min('9', '2'), '2');

    // Checks for max()
    assert_eq!(max(25, -6), 25);
    assert_eq!(max(563, 132), 563);

    assert_eq!(max('a', 'y'), 'y');
    assert_eq!(max('9', '2'), '9');
}