fn main() {
    println!("hello world");
    let a = 1;
    let b = 2;
    println!("{}", a);
    println!("{}", b);
    let mut total = 0;
    for i in 1..=200 {
        total += i;
    }
    println!("{}", total);
}
