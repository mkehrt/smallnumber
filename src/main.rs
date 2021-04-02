// Ugh I was going to use bigints but looks like I will not need to.

fn main() {
    let mut i: u64 = 0;
    loop {
        i += (1 as u64);
        if i % (1_000_000_000 as u64) == 0 {
            println!("{}", i);
        }
        // Find the number of numbers smaller than i.
        let num_less_than_i: u64 = &i - (1 as u64);
        // See if there are at least as many numbers above i than 1 + the
        // number below.
        let min_num_more_than_i: u64 = i + num_less_than_i + (1 as u64) - i;
        if min_num_more_than_i <= num_less_than_i {
            println!("FOUND LARGE NUMBER: {}", i);
        }
    }

    /* I was going to use the following n^2 code; unfortunately it looks like it will only get into the billions.
    let mut i = Integer::from(0);
    loop {
        i += 1;
        println!("{}", i);
        let mut count_less = Integer::new();
        let mut count_less_index = Integer::new();
        loop {
            count_less_index += 1;
            if count_less_index >= i {
                break;
            }
            count_less += 1;
        }
        let mut count_more = Integer::new();
        let mut count_more_index = Integer::new();
        loop {
            count_more_index += 1;
            count_more += 1;
            if count_more_index > count_less_index {
                // Just short circuit since there are more numbers bigger than smaller.
                break;
            }
            // Otherwise continue until we are out of numbers.
        }
        if count_less > count_more {
            println!("FOUND LARGE NUMBER: {}", i);
        }
    } */
}
