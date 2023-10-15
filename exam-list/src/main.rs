#![allow(dead_code)]

use std::rc::Rc;
// use study::{exam_rc};

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

fn main() {
    for i in 1..3 {
        println!("{}", i);
    }

    for i in 5..=7 {
        println!("{}", i);
    }

    let guess: u32 = "4a2".parse().expect("err");
    println!("{}", guess);

    let guess: u32 = "4a2".parse().unwrap();
    println!("{}", guess);

    let opt = Some(3);
    println!("expect opt: {}", opt.expect("bad option"));
    println!("unwrap opt: {}", opt.unwrap());

    // exam_rc();
}
