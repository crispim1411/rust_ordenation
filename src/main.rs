use orst::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

const N: u32 = 100;

fn main() {
    println!("Embaralhando lista de 0 a {}", N);
    println!("Algorithm: {}", "BubbleSort");
    test_sorter(BubbleSort);
    println!("Algorithm: {}", "InsertionSort Dumb");
    test_sorter(InsertionSort {smart: false});
    println!("Algorithm: {}", "InsertionSort Smart");
    test_sorter(InsertionSort {smart: true});
    println!("Algorithm: {}", "SelectionSort");
    test_sorter(SelectionSort);
    println!("Algorithm: {}", "QuickSort");
    test_sorter(QuickSort);
    println!("Algorithm: {}", "MergeSort");
    test_sorter(MergeSort);
}

fn test_sorter<S: Sorter>(algorithm: S) {
    let range: Vec<u32> = (0..N).collect();

    let mut things = range.clone();
    things.shuffle(&mut thread_rng());

    algorithm.sort(&mut things);
    match things == range {
        true => println!("Ok"),
        false => {
            println!("Error");
            println!("{:?}", things);
        },
    }
}