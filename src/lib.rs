//Trait Sorter a função sort recebe um slice genérico onde T é ordenável
pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T]) 
    where 
        T: Ord + Copy;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod mergesort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use selectionsort::SelectionSort;
pub use quicksort::QuickSort;
pub use mergesort::MergeSort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where 
            T: Ord 
        {
            slice.sort()
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
