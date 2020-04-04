// sort and merge
pub fn merge(l1: &Vec<i32>, l: usize, m: usize, r: usize, l2: &mut Vec<i32>) {
    // leftmost index
    let mut ptr1 = l;
    // middle index
    let mut ptr2 = m;

    // loop from leftmost index to rightmost index
    for i in l..r {
        // if ptr1 is smaller than middle index and (ptr2 is not less than rightmost index or ptr1 member is not greater than ptr2 member)
        if (ptr1 < m) && (ptr2 >= r || l1[ptr1] <= l1[ptr2]) {
            //
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

// copy over temporary values from arr2 to arr1
pub fn merge_copy(arr1: &mut Vec<i32>, l: usize, r: usize, arr2: &Vec<i32>) {
    (l..r).for_each(|i| arr1[i] = arr2[i]);
}

// split vector
pub fn merge_split(arr1: &mut Vec<i32>, l: usize, r: usize, arr2: &mut Vec<i32>) {
    if r - l > 1 {
        // middle index
        let m: usize = (r + l) / 2;

        // split first half
        merge_split(arr1, l, m, arr2);

        // split second half
        merge_split(arr1, m, r, arr2);

        // sort and merge
        merge(arr1, l, m, r, arr2);

        // copy over temporary values from arr2 to arr1
        merge_copy(arr1, l, r, arr2);
    }
}

pub fn sort(arr: &mut Vec<i32>) {
    // vector size
    let size = arr.len();

    // temporary second vector
    let mut arr2: Vec<i32> = vec![0; size];

    // start initial split
    merge_split(arr, 0, size, &mut arr2);
}
