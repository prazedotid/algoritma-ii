use std::io::Write;

mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod shell_sort;
mod selection_sort;

fn print_vector(arr: &Vec<i32>) {
    for num in arr.iter() {
        print!("{} ", num);
        std::io::stdout().flush().unwrap();
    }
}

fn main() {
    let mut arr: Vec<i32> = vec![32, 11, 25, 58, 2, 88, 91, 4, 65, 33];

    println!("Array before sorting:");
    print_vector(&arr);

    // bubble_sort::sort(&mut arr);
    insertion_sort::sort(&mut arr);
    // merge_sort::sort(&mut arr);
    // shell_sort::sort(&mut arr);
    // selection_sort::sort(&mut arr);

    println!("\nArray after sorting:");
    print_vector(&arr);
}
