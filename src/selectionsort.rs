use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T]) 
    where
        T: Ord 
    {
        for i in 0..slice.len() {

            let mut smallest = i;
            for j in (i+1)..slice.len() {
                if slice[j] < slice[smallest] {
                    smallest = j;
                }
            }
            
            slice.swap(smallest, i);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}