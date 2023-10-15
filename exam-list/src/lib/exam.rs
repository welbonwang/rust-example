#![allow(dead_code)]


use std::rc::Rc;

mod exam {

pub fn exam_rc() {
    let t = Rc::new(Box(3));
    let elem = t.deref();
    elem = 5;
    println!({:?}, elem);
}

}
