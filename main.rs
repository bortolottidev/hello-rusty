fn main() {
    println!("Marry xmasss from rustyyyy");
    print_more();
    looping();
    ownership();
}

fn eleven() -> u32 {
    11
}

fn print_more() {
    let y = if true {
        let x = 3;
        x + 12
    } else {
        12938213
    };
    println!("y value: {y}");
    let eleven = eleven();
    println!("eleven is: {eleven}");
}

fn looping() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        println!("Current counter: {counter}");
    };
    println!("Final counter: {counter}");
    println!("Result: {result}");

    for number in (1..10).rev() {
        println!("Wow reversing {number}!");
    }
}

fn ownership() {
    // tedious way
    fn calc_length(str: String) -> (String, usize) {
        let length = str.len();
        (str, length)
    }
    let s1 = String::from("hello");
    let (s2, len) = calc_length(s1);
    // s1 is moved and i cant do this
    // println!("{s1}");
    println!("{s2} with len {len}");

    let s3 = String::from("clone instead of mode");
    let (s4, len2) = calc_length(s3.clone());
    // s3 is cloned and i can do this
    println!("{s3}");
    println!("{s4} with len {len2}");
}
