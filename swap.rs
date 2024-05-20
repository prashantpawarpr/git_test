fn main() {
    let mut a = 12000;
    let mut b = 12121;
    println!("previous value of a is {} and b is {}", a, b);
    let c = a;
    a = b;
    b = c;
    println!("new value of a is {} and b is {}", a, b);
}
