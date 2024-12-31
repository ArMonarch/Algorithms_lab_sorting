fn partation(array: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = array[hi];
    let mut i = lo;

    for j in lo..hi {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    // Lastly, exchange the pivot with array[i + 1] & pivot element
    array.swap(i, hi);
    i
}

pub fn quick_sort(array: &mut [i32], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partation(array, lo, hi);
        if pivot > 0 {
            quick_sort(array, lo, pivot - 1);
        }
        quick_sort(array, pivot + 1, hi);
    }
}

#[cfg(test)]

mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn quick_sort_basic() {
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sotred_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        quick_sort(&mut unsorted_array, 0, 24);
        assert_eq!(unsorted_array, sotred_array);
    }

    #[test]
    fn quick_sort_best_case() {
        // Quick sort best case is when the pivot element divides the array in two equal halves.
        let mut unsorted_array = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 11,
        ];
        let sorted_array = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        quick_sort(&mut unsorted_array, 0, 19);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn quick_sort_average_case() {
        // Quick Sort Average case, the pivot divides the array into two parts, but not necessarily equal
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sotred_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        quick_sort(&mut unsorted_array, 0, 24);
        assert_eq!(unsorted_array, sotred_array);
    }

    #[test]
    fn quick_sort_worst_case() {
        // Quick Sort Worst Case, the largest or smallest element is choosen as pivot (i.e. sorted
        // array)
        let mut unsorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        quick_sort(&mut unsorted_array, 0, 24);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    fn quick_sort_empty_array() {
        // check edge case of empty array
        let mut unsorted_array: Vec<i32> = vec![];
        quick_sort(&mut unsorted_array, 0, 0);
        assert_eq!(unsorted_array, vec![]);
    }

    #[test]
    fn quick_sort_duplicates() {
        // check edge case of array having multiple duplicates emements
        let mut unsorted_array = vec![
            20, 17, 49, 15, 5, 21, 15, 21, 12, 9, 47, 34, 24, 35, 4, 47, 27, 9, 23, 44, 2, 5, 23,
            42, 44,
        ];
        let sorted_array = vec![
            2, 4, 5, 5, 9, 9, 12, 15, 15, 17, 20, 21, 21, 23, 23, 24, 27, 34, 35, 42, 44, 44, 47,
            47, 49,
        ];
        quick_sort(&mut unsorted_array, 0, 24);
        assert_eq!(unsorted_array, sorted_array);
    }

    #[test]
    #[ignore]
    fn quick_sort_01k_100k() {
        use chrono::Local;
        use std::fs::File;
        // use std::io;
        use std::path::Path;

        let path = Path::new("Quick_Sort_Pref_01.log");
        let display = path.display();

        //open a file in write-only mode
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {} : {}", display, why),
            Ok(file) => file,
        };
        // perf Test Time Complexity and worst case for quick sort with data of size (100,1_000,10_000,
        // 100_000)
        {
            for power in 2..6 {
                let size: i32 = 10_i32.pow(power);
                let range: (i32, i32) = (0, size - 1);
                let mut unsorted_array = gen_sorting_data(size, range);

                {
                    // perf test Time Complexity Of randomly generated data for Quick Sort
                    let heading =
                        format!("\nQuick Sort Time Complexity for Data of Size: {}\n", size);
                    if let Err(why) = file.write(heading.as_bytes()) {
                        panic!("Err While writing to File {}: {}", display, why);
                    } else {
                        println!("Content Written Successfully");
                    };

                    // for Quick Sort Time Complexity
                    let unsorted_array_size = unsorted_array.len();
                    let start_time = Local::now();
                    quick_sort(&mut unsorted_array, 0, unsorted_array_size - 1);
                    let end_time = Local::now();
                    let time_taken = end_time - start_time;

                    // format each time values in more readable format
                    let time_format = "%H:%M:%S:%f";
                    let start_time = start_time.format(time_format);
                    let end_time = end_time.format(time_format);

                    let formatted_value = format!(
                        "Start Time: {}\nFinished Time: {}\nTime Taken: {}\n",
                        start_time, end_time, time_taken
                    );
                    if let Err(why) = file.write(formatted_value.as_bytes()) {
                        panic!("Err While writing to file {}: {}", display, why);
                    } else {
                        println!("Content Written Successfully")
                    }
                }

                {
                    // perf test for Worst Case of Quick Sort
                    // Worst case is for already sorted array, So feed the already sorted data
                    // through quick sort,
                    // again to quick sort and measure perforamce
                    let heading = format!(
                        "\nQuick Sort Worst Case Time Complexity for Data of size: {}\n",
                        size
                    );
                    if let Err(why) = file.write(heading.as_bytes()) {
                        panic!("Err While writing to File {}: {}", display, why);
                    } else {
                        println!("Content Written Successfully");
                    };

                    // for Quick Sort Time Complexity
                    let unsorted_array_size = unsorted_array.len();
                    let start_time = Local::now();
                    quick_sort(&mut unsorted_array, 0, unsorted_array_size - 1);
                    let end_time = Local::now();
                    let time_taken = end_time - start_time;

                    // format each time values in more readable format
                    let time_format = "%H:%M:%S:%f";
                    let start_time = start_time.format(time_format);
                    let end_time = end_time.format(time_format);

                    let formatted_value = format!(
                        "Start Time: {}\nFinished Time: {}\nTime Taken: {}\n",
                        start_time, end_time, time_taken
                    );
                    if let Err(why) = file.write(formatted_value.as_bytes()) {
                        panic!("Err While writing to file {}: {}", display, why);
                    } else {
                        println!("Content Written Successfully")
                    }
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn quick_sort_05m_5m() {}
}

pub fn gen_unsorted_array(capacity: usize, size: i32) -> Vec<i32> {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let mut vec: Vec<i32> = vec![0; capacity];
    for value in &mut vec {
        *value = rng.gen_range(0..size);
    }
    vec
}

// fn gen_sorting_data(size: usize, range: (i32, i32)) -> (Vec<i32>, Vec<i32>) {
//     use rand::{thread_rng, Rng};
//     let mut rng = thread_rng();
//     let unsorted_array: Vec<i32> = (0..size).map(|_| rng.gen_range(range.0..range.1)).collect();
//     let mut sorted_array = unsorted_array.clone();
//     sorted_array.sort(); // sort the unsorted data
//     (unsorted_array, sorted_array)
// }

fn gen_sorting_data(size: i32, range: (i32, i32)) -> Vec<i32> {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let unsorted_array: Vec<i32> = (0..size).map(|_| rng.gen_range(range.0..range.1)).collect();
    unsorted_array
}

// fn gen_sorted_data(size: i32) -> Vec<i32> {
//     let vec = (0..size).collect();
//     vec
// }

fn main() {}
