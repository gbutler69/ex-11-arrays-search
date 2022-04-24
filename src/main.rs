use ex_11_arrays_search::{
    binary_search, linear_search, pairs, reverse, subarrays, subarrays_max_sum,
    subarrays_max_sum_using_prefixes,
};

fn main() {
    let items = [10, 15, 12, 11, 32, 55, 6, 9, 100];
    match linear_search(&11, &items) {
        Some(idx) => println!("Found 11 at index {idx}"),
        None => println!("Did not find 11"),
    }

    let items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    for item in (0..10)
        .into_iter()
        .chain(items.iter().copied())
        .chain((51..60).into_iter())
        .chain((91..100).into_iter())
    {
        match binary_search(&item, &items) {
            Some(idx) => println!("Found {item} at index {idx}"),
            None => println!("Did NOT find {item}"),
        }
    }

    let mut items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    reverse(&mut items);
    for item in items {
        print!("{item}, ");
    }
    println!();

    let items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let pairs = pairs(&items);
    for item in pairs {
        print!("{item:?}, ");
    }
    println!();

    let items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let subarrays = subarrays(&items);
    for subarray in subarrays {
        for item in subarray {
            print!("{item}, ")
        }
        println!()
    }

    let items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    match subarrays_max_sum(&items) {
        Some(sum) => println!("Maximum sum of subarrays of {items:?} is {sum}."),
        None => println!("Nothing to sum so no maximum available!"),
    }

    let items = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    match subarrays_max_sum_using_prefixes(&items) {
        Some(sum) => println!("Maximum sum of subarrays of {items:?} is {sum}."),
        None => println!("Nothing to sum so no maximum available!"),
    }
}
