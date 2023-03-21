fn main() {
    println!("Marry xmasss from rustyyyy");
    print_more();
    looping();
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
