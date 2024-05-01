// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.



// 1. You could make another, separate version of the data that's in `vec0` and pass that 
// to `fill_vec` instead.

// fn main() {
//     let vec0 = Vec::new();

//     let vec0_copy = vec0.clone();
//     let mut vec1 = fill_vec(vec0_copy);

//     println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
// and then copy the data within the function (`vec.clone()`) in order to return an owned
// `Vec<i32>`.

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(&vec0);

//     println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// 3. Or, you could make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
// mutable), modify it directly, then not return anything. This means that `vec0` will change over the
// course of the function, and makes `vec1` redundant (make sure to change the parameters of the `println!`
// statements if you go this route)

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
