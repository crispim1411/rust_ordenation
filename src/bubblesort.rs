use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T]) 
    where
        T: Ord 
    {
        for i in 0..slice.len() {
            for j in i..slice.len() {
                if i == j {
                    continue;
                }
                if slice[j] < slice[i] {
                    slice.swap(i, j);
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}