pub fn sort(arr: &mut Vec<i32>) {
    // vector size
    let size = arr.len();

    // loop through vector indexes
    for index in 0..size {
        // set initial min_index value
        let mut min_index = index;

        // loop through the next indexes
        for j in (index + 1)..size {
            // if this number is smaller than number at min index
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // swap vector member positions
        arr.swap(min_index, index);
    }
}
