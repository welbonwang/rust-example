
struct Cat {
    age: u32,
    size: u32,
}

fn main() {
    let cat1 = Cat{size:2, age:3};
    println!("{}, {}", cat1.age, cat1.size);

    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);

    // let cat2 = cat1;
    // println!("cat1 age: {}, cag2 age{}", cat1.age, cat2.age);
}
