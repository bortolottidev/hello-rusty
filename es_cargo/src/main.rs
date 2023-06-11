#[derive(Debug)]
struct LoggedStructure {
    width: u32,
    height: u32,
}

impl LoggedStructure {
    fn test(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &LoggedStructure) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn log(&self) -> () {
        println!("Widht is {}, while height is {}", self.width, self.height);
    }
}

impl LoggedStructure {
    fn build() -> Self {
        Self {
            width: 1,
            height: 2,
        }
    }
}

fn main() {
    println!("Cargo but PROD ready!");
    let s = LoggedStructure {
        width: 2,
        height: 8,
    };

    println!("structure {:#?}", s);
    dbg!(&s);
    println!("test defined {}", s.test());
    println!("test getter {}", s.width());

    let bigger = LoggedStructure {
        width: 4,
        height: 10,
    };
    let smaller = LoggedStructure {
        width: 1,
        height: 2,
    };

    println!("s can contains bigger: {}", s.can_hold(&bigger));
    println!("s can contains smaller: {}", s.can_hold(&smaller));

    LoggedStructure::build().log();
}


