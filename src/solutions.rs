#![allow(unused)]

use crate::SortSolution;

struct StdSort;

impl SortSolution for StdSort {
    fn describle(&self) -> String {
        "std::sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        input.sort();
        input
    }
}

struct BubbleSort;

impl SortSolution for BubbleSort {
    fn describle(&self) -> String {
        "bubble sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        let n = input.len();
        for i in (1..n).rev() {
            for j in 0..i {
                if input[j] > input[j + 1] {
                    input.swap(j, j + 1);
                }
            }
        }
        input
    }
}

struct SelectionSort;

impl SortSolution for SelectionSort {
    fn describle(&self) -> String {
        "selection sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        let n = input.len();
        for i in 0..n {
            let mut min_idx = i;
            for j in i + 1..n {
                if input[j] < input[min_idx] {
                    min_idx = j;
                }
            }
            input.swap(i, min_idx);
        }
        input
    }
}

struct InsertionSort;

impl SortSolution for InsertionSort {
    fn describle(&self) -> String {
        "insertion sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        let n = input.len();
        for i in 1..n {
            // inserting i-th element
            let mut j = i;
            while j > 0 && input[j] < input[j - 1] {
                input.swap(j, j - 1);
                j -= 1;
            }
        }
        input
    }
}

/// Observed that insertion sort is fast when the input is almost sorted.
/// So, divide the input into several sub-arrays and sort them separately.
/// sub-arrays index: [0, gap, 2 * gap, 3 * gap, ...], gap = n / 2, n / 4, n / 8, ..., 1
struct ShellSort;

impl SortSolution for ShellSort {
    fn describle(&self) -> String {
        "shell sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        let n = input.len();
        let mut gap = n / 2;
        while gap > 0 {
            for i in gap..n {
                let mut j = i;
                while j >= gap && input[j] < input[j - gap] {
                    input.swap(j, j - gap);
                    j -= gap;
                }
            }
            gap /= 2;
        }
        input
    }
}

struct MergeSort;

impl SortSolution for MergeSort {
    fn describle(&self) -> String {
        "merge sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        fn merge(mut left: Vec<isize>, mut right: Vec<isize>) -> Vec<isize> {
            let mut result = Vec::new();
            while !left.is_empty() && !right.is_empty() {
                if left[0] <= right[0] {
                    result.push(left.remove(0));
                } else {
                    result.push(right.remove(0));
                }
            }
            result.extend(left);
            result.extend(right);
            result
        }
        fn merge_sort(mut input: Vec<isize>) -> Vec<isize> {
            let n = input.len();
            if n <= 1 {
                return input;
            }
            let mid = n / 2;
            let left = merge_sort(input.drain(..mid).collect());
            let right = merge_sort(input);
            merge(left, right)
        }
        merge_sort(input)
    }
}

struct QuickSort;

impl SortSolution for QuickSort {
    fn describle(&self) -> String {
        "quick sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        fn quick_sort(input: &mut [isize]) {
            let n = input.len();
            if n <= 1 {
                return;
            }
            let mut pivot = 0;
            for i in 1..input.len() {
                if input[i] < input[pivot] {
                    for j in (pivot + 1..=i).rev() {
                        input.swap(j, j - 1);
                    }
                    pivot += 1;
                }
            }
            let (left, right) = input.split_at_mut(pivot);
            let right = &mut right[1..];
            quick_sort(left);
            quick_sort(right);
        }
        quick_sort(&mut input);
        input
    }
}

struct HeapSort;

impl SortSolution for HeapSort {
    fn describle(&self) -> String {
        "heap sort".to_string()
    }

    fn sort(&self, mut input: Vec<isize>) -> Vec<isize> {
        fn heapify(mut input: Vec<isize>, n: usize, i: usize) -> Vec<isize> {
            let mut largest = i;
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < n && input[left] > input[largest] {
                largest = left;
            }
            if right < n && input[right] > input[largest] {
                largest = right;
            }
            if largest != i {
                input.swap(i, largest);
                heapify(input, n, largest)
            } else {
                input
            }
        }
        fn heap_sort(mut input: Vec<isize>) -> Vec<isize> {
            let n = input.len();
            for i in (0..n / 2).rev() {
                input = heapify(input, n, i);
            }
            for i in (1..n).rev() {
                input.swap(0, i);
                input = heapify(input, i, 0);
            }
            input
        }
        heap_sort(input)
    }
}

struct CountingSort;

impl SortSolution for CountingSort {
    fn describle(&self) -> String {
        "counting sort".to_string()
    }

    fn sort(&self, input: Vec<isize>) -> Vec<isize> {
        let max = input.iter().max().unwrap();
        let min = input.iter().min().unwrap();
        let mut count = vec![0; (*max - *min + 1) as usize];
        for x in input.iter() {
            count[(x - min) as usize] += 1;
        }
        let mut result = Vec::new();
        for (i, &c) in count.iter().enumerate() {
            for _ in 0..c {
                result.push(i as isize + min);
            }
        }
        result
    }
}

struct BucketSort;

impl SortSolution for BucketSort {
    fn describle(&self) -> String {
        "bucket sort".to_string()
    }

    fn sort(&self, input: Vec<isize>) -> Vec<isize> {
        let max = input.iter().max().unwrap();
        let min = input.iter().min().unwrap();
        let mut buckets = vec![vec![]; 8];
        let step = (*max - *min) / 8 + 1;
        for x in input.iter() {
            buckets[((x - min) / step) as usize].push(x);
        }
        let mut result = Vec::new();
        for bulket in buckets.iter_mut() {
            bulket.sort();
        }
        for bulket in buckets {
            result.extend(bulket);
        }
        result
    }
}

struct RadixSort;

impl SortSolution for RadixSort {
    fn describle(&self) -> String {
        "radix sort".to_string()
    }

    fn sort(&self, input: Vec<isize>) -> Vec<isize> {
        let max = *input.iter().max().unwrap();
        let mut exp = 1;
        let mut result = input;
        while max / exp > 0 {
            let mut count = [0; 10];
            for x in result.iter() {
                count[((x / exp) % 10) as usize] += 1;
            }
            for i in 1..10 {
                count[i] += count[i - 1];
            }
            let mut output = vec![0; result.len()];
            for x in result.into_iter().rev() {
                // imagine we move nums into buckets numbered by `(x / exp) % 10` one by one,
                // thus we should take them out in the reverse order.
                output[count[((x / exp) % 10) as usize] - 1] = x;
                count[((x / exp) % 10) as usize] -= 1;
            }
            result = output;
            exp *= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::Checker;
    use rand::Rng;

    #[test]
    fn test() {
        let mut checker = Checker::new();
        checker.add_solu(&[
            &StdSort,
            &BubbleSort,
            &SelectionSort,
            &InsertionSort,
            &ShellSort,
            &MergeSort,
            &QuickSort,
            &HeapSort,
        ]);
        checker.check();
        let mut checker = Checker::new();
        checker.add_solu(&[&CountingSort, &BucketSort, &RadixSort]);
        let inputs: [[u16; 10]; 10] = rand::thread_rng().gen();
        let inputs = inputs
            .iter()
            .map(|x| x.iter().map(|&x| x as isize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        checker.check_with_inputs(&inputs.iter().map(|x| x.deref()).collect::<Vec<_>>());
    }
}
