
struct Cat {
    age: u32,
    size: u32,
}

fn main() {
    let cat = Cat{size:2, age:3};
    let x = 5;

    // address of instance of struct which is allocated in stack
    let rp_cat: *const Cat = &cat;
    let addr_cat = rp_cat as usize;
    let addr_x = &x as *const i32 as usize;
    println!("addr_cat: {}, addr_x: {}", addr_cat, addr_x);
}
