//Given a number n find all prime numbers

use std::collections::HashMap;

fn find_all_prime_numbers(a: u16) {
    //We can store the prime numbers in the vector or we can print the prime numbers. print prime numbers to save space complexity, since we dont need the data to be stored
    let mut non_prime: HashMap<u16, bool> = HashMap::new();
    //we have a number that we concat until we reach a, we start at the number, then we move the number to the next number that was not accounted for and concat that until we reach a

    /*
    updating variables:
    concat number

    iterate through all numbers from concat and add to hashmap
     */

    /*
        lets start here:

        our first number is 2, if a is 2 or less than 2 then we can exit early and just print a

        if our num satisfies, then we move on to the algorithm:

        our starting number is 2

        then we will add 2 until we hit the num

     */
    if a <= 2 {
        println!("No primes less than {} other than {}", a, a);
        return
    }
    let mut start_num: u16 = 2;

    while start_num * start_num < a {
        if !non_prime.contains_key(&start_num) {
            let mut multiple: u16 = start_num * start_num;
            while multiple <= a {
                non_prime.insert(multiple, true);
                multiple += start_num;
            }
        }
        start_num += 1;
    }

    for num in 2..a {
       if !non_prime.contains_key(&num) {
        println!("{}", num);
       } 
    }
}

fn main() {
    let num: u16 = 100;
    find_all_prime_numbers(num);
}
