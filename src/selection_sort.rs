fn main() {}

/// Selection Sort is a comparission based sorting algorithm. It sorts an array by repeatedly
/// selecting the Smallest (or Largest) element from the unsorted array and swapping it with the
/// first element of the unsorted array. This process continious until the entire array is sorted.
///
/// 1. First, find the smallest element and swap it with the first element.
/// 2. Then find the smallest elemetn among the remaining elements ans swap it with the first element
/// of the array.
pub fn selection_sort(data_array: &mut [i32]) {
    let array_len: usize = data_array.len();
    for index in 0..array_len {
        let mut minimun_value_index: usize = index; // Assume the current value is the minimun_value in the array

        for jndex in (index + 1)..array_len {
            if data_array[jndex] < data_array[minimun_value_index] {
                minimun_value_index = jndex;
            }
        }
        data_array.swap(index, minimun_value_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::{thread_rng, Rng, ThreadRng};

    #[test]
    fn test_selection_sort_best_case() {
        // Selection Sort best case is when the data_array is already sorted
        let mut unsorted_data_array: Vec<i32> = (0..1000).collect();
        let sorted_data_array: Vec<i32> = unsorted_data_array.clone();
        selection_sort(&mut unsorted_data_array);
        assert_eq!(unsorted_data_array, sorted_data_array);
    }

    #[test]
    fn test_selection_sort_average_case() {
        // Selection Sort average case is when the data array is randomly ordered
        let mut unsorted_data_array: Vec<i32> = Vec::with_capacity(1000);

        let mut rng: ThreadRng = thread_rng();
        for value in &mut unsorted_data_array {
            *value = rng.gen_range(0..5000);
        }

        let mut sorted_data_array: Vec<i32> = unsorted_data_array.clone();
        sorted_data_array.sort();
        selection_sort(&mut unsorted_data_array);
        assert_eq!(unsorted_data_array, sorted_data_array);
    }

    #[test]
    fn test_selection_sort_worst_case() {
        // for worst case the array should be in reverse ordered
        let data_array: Vec<i32> = (0..1000).collect();
        let mut reverse_data_array: Vec<i32> = data_array.clone();
        reverse_data_array.reverse();
        selection_sort(&mut reverse_data_array);
        assert_eq!(reverse_data_array, data_array);
    }

    #[test]
    fn test_selection_sort_empty_array() {
        // init empty array
        let mut empty_array: Vec<i32> = Vec::new();
        selection_sort(&mut empty_array);
        assert_eq!(empty_array, []);
    }

    #[test]
    fn test_selection_sort_duplicate_values() {
        // create an data_array of size n with all elements same
        let mut data_array: Vec<i32> = vec![0; 100000];
        selection_sort(&mut data_array);
        assert_eq!(data_array, vec![0; 100000]);
    }

    #[test]
    #[ignore]
    fn test_bench_selection_sort_01k_100k() {}

    #[test]
    #[ignore]
    fn bench_selection_sort_05m_5m() {}
}
