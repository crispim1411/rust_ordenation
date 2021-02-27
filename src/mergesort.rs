use super::Sorter;

pub struct MergeSort;

fn mergesort<T: Ord + Copy>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {
            let mid = slice.len()/2;
            mergesort(&mut slice[..mid]);
            mergesort(&mut slice[mid..]);

            let mut merged = slice.to_vec();
            merge(&slice[..mid], &slice[mid..], &mut merged);

            slice.copy_from_slice(&merged);
        }
    }
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], merged: &mut [T]) {
    assert_eq!(left.len() + right.len(), merged.len());

    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged[k] = left[i];
            k += 1;
            i += 1;
        } else {
            merged[k] = right[j];
            k += 1;
            j += 1;
        }
    }

    if i < left.len() {
        merged[k..].copy_from_slice(&left[i..]);
    } 
    else if j < right.len() {
        merged[k..].copy_from_slice(&right[j..]);
    }
}

impl Sorter for MergeSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Copy
    {
        mergesort(slice);
    }
}

//cargo t --lib mergesort
#[test]
fn it_works() {
    let mut things  = vec!(8, 6, 4, 1, 3, 2, 5, 9, 7);
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
}  

#[test]
fn merge_works() {
    let a = vec![0,1,4,7,9];
    let b = vec![2,3,5,6,8];
    let mut merged = vec![0; 10];
    merge(&a, &b, &mut merged);
    assert_eq!(merged, (0..10).collect::<Vec<u32>>());
}