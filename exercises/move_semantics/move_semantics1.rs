// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut vec0 = Rc::new(RefCell::new(Vec::new()));

    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {} content `{:?}`", "vec0", vec0.borrow().len(), vec0.borrow());
    println!("{} has length {} content `{:?}`", "vec1", vec1.borrow().len(), vec1.borrow());

    vec1.borrow_mut().push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.borrow().len(), vec1.borrow());
}

fn fill_vec(vec: Rc<RefCell<Vec<i32>>>) -> Rc<RefCell<Vec<i32>>> {
    {
        let mut vec = vec.borrow_mut();
        vec.push(22);
        vec.push(44);
        vec.push(66);
    }
    Rc::clone(&vec)
}
