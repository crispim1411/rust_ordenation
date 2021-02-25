use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_last_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if right == 0 {
            break;
        }
        
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    slice.swap(slice.len() - 1, left);

    let (left, right) = slice.split_at_mut(left);
    quicksort(left);
    quicksort(right);
}

fn median_of_three<T>(slice: &mut [T]) 
where
    T: Ord
{
    let mid: usize = slice.len() / 2;
    let max = slice.len() - 1;
    if slice[0] > slice[mid] {
        slice.swap(0, mid);
    }
    if slice[0] > slice[max] {
        slice.swap(0, max);
    }
    if slice[mid] > slice[max] {
        slice.swap(mid, max);
    }
    slice.swap(mid, max);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord
    {
        median_of_three(slice);
        quicksort(slice);
    }
}

//cargo t --lib quicksort
#[test]
fn it_works() {
    let mut things  = vec!(8, 6, 4, 1, 3, 2, 5, 9, 7);
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}  