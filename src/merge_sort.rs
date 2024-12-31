fn merge<T: Ord + Copy>(array: &mut [T], middle: usize) {
    // create temporary vectors to support the merge
    let left_half = array[..middle].to_vec();
    let right_half = array[middle..].to_vec();

    // indexes to track the position while merging
    let mut left_index: usize = 0;
    let mut right_index: usize = 0;

    for value in array {
        // choose which ever the value is smallest or from whichever which vec is not exhausted
        if right_index == right_half.len()
            || (left_index < left_half.len() && left_half[left_index] < right_half[right_index])
        {
            *value = left_half[left_index];
            left_index += 1;
        } else {
            *value = right_half[right_index];
            right_index += 1;
        }
    }
}

///  Merge sort is a sorting algorithm that follows the divide-and-conquer approach. It works by recursively
///  dividing the input array into smaller subarrays and sorting those subarrays then merging them back together
///  to obtain the sorted array.
///
/// In simple terms, we can say that the process of merge sort is to divide the array into two halves, sort each half,
/// and then merge the sorted halves back together. This process is repeated until the entire array is sorted.
pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() > 1 {
        let middle_index = array.len() / 2;

        // sort the left half recursively
        merge_sort(&mut array[..middle_index]);
        //sort the right half recursively
        merge_sort(&mut array[middle_index..]);
        // combine the two halves
        merge(array, middle_index);
    }
}

pub fn gen_unsorted_array(size: i32, range: (i32, i32)) -> Vec<i32> {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen_range(range.0..range.1)).collect()
}

pub fn gen_array(size: i32) -> Vec<i32> {
    (0..size).collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn merge_sort_basic() {
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        merge_sort(&mut unsorted_array);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn merge_sort_best_case() {
        // Best case: Already Sorted Array
        let mut unsorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        merge_sort(&mut unsorted_array);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn merge_sort_average_case() {
        // Average Case: jumbled array
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        merge_sort(&mut unsorted_array);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn merge_sort_worst_case() {
        // Worst Casr: same as average case
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        merge_sort(&mut unsorted_array);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn merge_sort_empty_array() {
        let mut empty: Vec<i32> = vec![];
        merge_sort(&mut empty);
        assert_eq!(empty, vec![]);
    }

    #[test]
    fn merge_sort_duplicates() {
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        merge_sort(&mut unsorted_array);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    #[ignore]
    fn perf_merge_sort_average() {
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::path::Path;

        let path = Path::new("Merge_Sort_Perf.log");
        let display = path.display();

        let mut option = OpenOptions::new();
        let mut file = match option.append(true).open(path) {
            Err(why) => panic!("Create {}: {}", display, why),
            Ok(file) => file,
        };

        let size: i32 = 10_000_000;
        let range: (i32, i32) = (0, size);

        let heading = format!("\nMerge Sort Time Complexity for Data of Size: {}\n", size);
        if let Err(why) = file.write(heading.as_bytes()) {
            panic!("Err While writing to File {}: {}", display, why);
        } else {
            println!("Content Written Successfully");
        };

        // init an unsorted_array
        let mut unsorted_array = gen_unsorted_array(size, range);

        // measure the time taken by merge sort
        let start_time = Local::now();
        merge_sort(&mut unsorted_array);
        let end_time = Local::now();
        let time_taken = end_time - start_time;

        // format the time in more readable format
        let time_format = "%H:%M:%S:%f";
        let start_time = start_time.format(time_format);
        let end_time = end_time.format(time_format);

        // write the oputut to log file
        let formatted_value = format!(
            "\nStart Time: {}\nFinished Time: {}\nTime Taken: {}\n",
            start_time, end_time, time_taken
        );
        if let Err(why) = file.write(formatted_value.as_bytes()) {
            panic!("Err While writing to file {}: {}", display, why);
        } else {
            println!("Content written to File");
        }
    }

    #[test]
    #[ignore]
    fn perf_merge_sort_worst() {
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::path::Path;

        let path = Path::new("Merge_Sort_Perf.log");
        let display = path.display();

        let mut option = OpenOptions::new();
        let mut file = match option.append(true).open(path) {
            Err(why) => panic!("Create {}: {}", display, why),
            Ok(file) => file,
        };

        let size: i32 = 100;
        let range: (i32, i32) = (0, size);

        let heading = format!("\nMerge Sort Time Complexity for Data of Size: {}\n", size);
        if let Err(why) = file.write(heading.as_bytes()) {
            panic!("Err While writing to File {}: {}", display, why);
        } else {
            println!("Content Written Successfully");
        };

        // init an unsorted_array
        let mut unsorted_array = gen_unsorted_array(size, range);

        // measure the time taken by merge sort
        let start_time = Local::now();
        merge_sort(&mut unsorted_array);
        let end_time = Local::now();
        let time_taken = end_time - start_time;

        // format the time in more readable format
        let time_format = "%H:%M:%S:%f";
        let start_time = start_time.format(time_format);
        let end_time = end_time.format(time_format);

        // write the oputut to log file
        let formatted_value = format!(
            "\n\nStart Time: {}\nFinished Time: {}\n Time Taken: {}\n",
            start_time, end_time, time_taken
        );
        if let Err(why) = file.write(formatted_value.as_bytes()) {
            panic!("Err While writing to file {}: {}", display, why);
        } else {
            println!("Content written to File");
        }
    }
}

fn main() {}
