fn main() {
    println!("Marry xmasss from rustyyyy");
    print_more();
    looping();
    ownership();
    play_with_struct();
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

    // borrowing, not moving
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let s1b = String::from("hello refs");
    let len = calculate_length(&s1b);
    println!("The length of '{}' is {}.", s1b, len);
}

struct User {
    is_active: bool,
    username: String,
}

struct Point(i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

fn play_with_struct() {
    fn build_user(username: String) -> User {
        User {
            is_active: true,
            username,
        }
    }

    let mut user = build_user(String::from("daniele"));
    println!("Hello {}", user.username);
    user.username = String::from("samuele");
    println!("Oh! What a twist! Hello {}", user.username);

    // hello javascript
    let another_user = User { ..user };
    println!(
        "Hello [active] {} [user] {}",
        another_user.is_active, another_user.username
    );

    // cannot use this because username has been moved (see 4.1 @borrow)
    // println!("Hello {}", user.username);
    let point = Point(11, 42, 100);
    let point2 = Point(11, 42, 100);
    fn distance(point1: &Point, point2: &Point) -> i32 {
        // random computing...
        point1.0 * point2.0
    }
    println!("Distance is {}", distance(&point, &point2));
    let unit = UnitLikeStruct;
    dbg!(&unit);
}
