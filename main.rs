use std::io;
fn main (){
    println!("summation = {}", summation(4294967295));
    // let mut input = String::new(); 
    // io::stdin().read_line(&input).expect("Failed to read line");
    // let input = input.trim();
    // let number: u32 = input.parse().expect("Not a good number!")
}

    fn summation (z: u32) -> u32{
        let mut val = 0; 
        for i in 0..z{
            val += i * i;
        }
        return val;
    }

//Report: The largest number that can be put into the program is 4294967295.
//Integer overflow happens when an attempt to store a value that is too large for an integer type is made.
// Since it is clear it has to be 4294967295 or under to work,  the terminal reads "does not fit into the type `u32` whose range is `0..=4294967295" when a number is too large.
// Because the program was made in a way to only expect numbers u32, then integer overflow is not an issue in the code, the code will just not run if it is over. 

