fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }

    println!("The value of x is: {}", x);

    const Y: i32 = 10;
    println!("The value of Y id:{}", Y);

    let tup = (500, 6.4, "hoge", true);

    let (a, b, c, d) = tup;
    println!("a: {}, b: {}, c:{}, d:{}", a, b, c, d);
}
