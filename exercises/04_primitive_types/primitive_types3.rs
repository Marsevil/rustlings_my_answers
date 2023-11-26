// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    const SIZE: usize = 200;
    let a = {
        let mut a = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            a.push(i);
        }
        a
    };
    
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
