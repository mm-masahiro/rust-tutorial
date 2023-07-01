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

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition_num = 2;
    let num = if condition_num > 0 {
        "This is more than 0"
    } else {
        "This is less than 0"
    };

    println!("{}", num);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut count_down = 3;
    while count_down > 0 {
        println!("count_down is {}", count_down);
        count_down -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("the value is {}", element)
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    let n = 8;

    let mut two_previous_num = 1;
    let mut previous_num = 1;
    let mut fibonacci = 1;

    for _ in 1..n - 1 {
        if n == 1 || n == 2 {
            1;
            break;
        }

        fibonacci = two_previous_num + previous_num;
        two_previous_num = previous_num;
        previous_num = fibonacci;
    }

    println!("fibonacci is {}", fibonacci);
}

fn return_five() -> i32 {
    5
}

fn ownership() {
    let s = "hello";
}
