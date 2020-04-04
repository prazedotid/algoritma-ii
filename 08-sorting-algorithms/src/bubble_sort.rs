pub fn sort(arr: &mut Vec<i32>) {
    // vector size
    let size = arr.len();

    // loop from 0 to size - 1
    for i in 0..(size - 1) {
        // swapped flag
        let mut swapped = false;

        // subarray loop
        for j in 0..(size - i - 1) {
            // swap if current number is larger than next number
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // break loop if there is nothing swapped on subarray
        if !swapped {
            break;
        }
    }
}
