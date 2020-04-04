pub fn sort(arr: &mut Vec<i32>) {
    // vector size
    let size = arr.len();

    // loop from 0 to vector length
    for i in 0..size {
        // loop from i - 1 to 0
        for j in (0..i).rev() {
            // swap if current number is larger than next number
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
