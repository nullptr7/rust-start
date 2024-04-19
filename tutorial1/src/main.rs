fn main() {
    // This is a line comment
    /* This is a block comment */

    let x: i32 = 4;

    let mut y: bool = false;

    println!("Hello, world!");

    println!("value of x {}", x);

    println!("value of y initial: {}", y);

    y = true;

    println!("value of y after update: {}", y);

    let x: u32 = 5;

    println!("value of x after reassignment {}", x);

    {
        let x: bool = false;
        let y: u32 = 42;
        println!("value of x inside a scope: {} and y: {}", x, y);
    }

    {
        let x: u32 = x + 1;
        println!("value of x which access scope outside: {}", x)
    }

    let x: &str = "question";

    println!("value of x after another reassignment {}", x);

    const A_CONSTANT: i32 = 42;
    println!("Value of A_CONSTANT - {}", A_CONSTANT);

    {
        const A_CONSTANT: bool = false;
        println!("Value of A_CONSTANT inside loop- {}", A_CONSTANT);
    }

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    let x: i32 = 5 + /* 90 + */ 5;
    eprintln!("Is `x` 10 or 100? x = {}", x);

    let x: u8 = 255; // 0  - 255
    let y: u8 = 10; // -128 - 127

    let z = x / y;
    println!("{}", z);
}
