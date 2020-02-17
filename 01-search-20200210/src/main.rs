use std::{cmp, io};
use std::io::Write;

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn print(str: &str) {
    print!("{}", &str);
    io::stdout().flush().unwrap();
}

fn collect_input(str_args: [&str; 2]) -> Vec<usize> {
    // Input haystack length
    print(str_args[0]);
    let len = get_input().trim().parse::<usize>().unwrap();

    // Input haystack numbers
    print(str_args[1]);
    let input: Vec<usize> = get_input().trim().split(" ")
        .map(|x| x.parse().expect("Not a valid integer!"))
        .collect();

    // Determine haystack slice ceiling
    let ceil = cmp::max(len, input.len());

    // Slice haystack inputs based on maximum haystack length
    input[..ceil].to_owned()
}

fn main() {
    // Input functions
    println!("=== Input ===");

    // Slice haystack inputs based on maximum haystack length
    let haystack = collect_input([
        "Input haystack length > ",
        "Input haystack numbers > "
    ]);

    // Slice needle inputs based on maximum needle length
    let needles = collect_input([
        "Input needle length > ",
        "Input needle numbers > "
    ]);

    // Search functions
    println!("=== Search ===");

    // Iterate over needle stack
    for needle in needles.iter() {
        // Boolean flag used for search
        let mut found = false;

        // Iterate over haystack
        for hay in haystack.iter() {
            // Break out of loop if needle is found in haystack
            if needle == hay {
                found = true;
                break;
            }
        }

        // Print outputs based on search result
        if found {
            println!("{}: Ditemukan.", &needle);
        } else {
            println!("{}: Tidak ditemukan.", &needle);
        }
    }
}

