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

    another_function();
}

fn another_function() {
    println!("Called Another Function");

    next_function();
}

fn next_function() {
    println!("called next function");
    let x = return_five();
    println!("return five is {}", x);

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn return_five() -> i32 {
    5
}
