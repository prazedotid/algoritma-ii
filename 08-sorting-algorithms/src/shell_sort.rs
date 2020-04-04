pub fn sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    // Start with a big gap
    let mut gap = size / 2;
    while gap > 0 {
        // Do a gapped insertion sort for this gap size.
        // The first gap elements a[0..gap-1] are already in gapped order
        // keep adding one more element until the entire array is
        // gap sorted
        for i in gap..size {
            // add a[i] to the elements that have been gap sorted
            // save a[i] in temp and make a hole at position i
            let temp = arr[i];

            // shift earlier gap-sorted elements up until the correct
            // location for a[i] is found
            let mut j = i;
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }

            // put temp (the original a[i]) in its correct location
            arr[j] = temp;
        }
        // then reduce the gap
        gap /= 2;
    }
}
