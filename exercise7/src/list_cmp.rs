use std::cmp::Ordering;

fn min<T: std::cmp::Ord + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];
    list[1..].iter().
        map(|a| smallest = match smallest.cmp(&a){
                Ordering::Greater => *a,
                Ordering::Equal | Ordering::Less => smallest  
                }
            ).last();
    smallest
}

fn max<T: std::cmp::Ord + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    list[1..].iter().
        map(|a| largest = match largest.cmp(&a){
                Ordering::Greater => largest,
                Ordering::Equal | Ordering::Less => *a   
                }
            ).last();
    largest
}

fn main() {
    // Checks for min()
    let list1 = [25, -6, -2, 53];
    let list2 = [563, 132, 746, 90];
    let list3 = ['m', 'c', 'x'];
    let list4 = ['9', '2', '3'];

    assert_eq!(min(&list1), -6);
    assert_eq!(min(&list2), 132);

    assert_eq!(min(&list3), 'c');
    assert_eq!(min(&list4), '2');

    // Checks for max()
    assert_eq!(max(&list1), 53);
    assert_eq!(max(&list2), 746);

    assert_eq!(max(&list3), 'x');
    assert_eq!(max(&list4), '9');
}