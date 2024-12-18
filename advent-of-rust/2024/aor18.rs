#[derive(Clone)]
pub struct Sleigh {
    color: String,
    engine: String,
    gift_capacity: u32,
    magical_enhancements: bool,
}

#[derive(Clone)]
pub struct SleighBuilder {
    // Define the fields of SleighBuilder here
    sleigh: Sleigh,
}

macro_rules! build_field {
    ($f:ident, $t:ty) => {
        pub fn $f(mut self, $f: $t) -> SleighBuilder {
            self.sleigh.$f = $f.to_owned();
            self
        }
    };
}

impl SleighBuilder {
    // Your code here...
    pub fn new() -> SleighBuilder {
        Self {
            sleigh: Sleigh {
                color: "red".to_owned(),
                engine: "reindeer-powered".to_owned(),
                gift_capacity: 100,
                magical_enhancements: false,
            },
        }
    }
    build_field!(color, &str);
    build_field!(engine, &str);
    build_field!(gift_capacity, u32);
    pub fn magical_enhancements(mut self) -> SleighBuilder {
        self.sleigh.magical_enhancements = true;
        self
    }

    pub fn build(self) -> Sleigh {
        self.sleigh
    }
}

// Don't Change this implementation
// It is used for the tests
impl Sleigh {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn engine(&self) -> &str {
        &self.engine
    }

    pub fn gift_capacity(&self) -> u32 {
        self.gift_capacity
    }

    pub fn magical_enhancements(&self) -> bool {
        self.magical_enhancements
    }
}

pub fn main() {
    let sleigh = SleighBuilder::new()
        .color("gold")
        .engine("magic")
        .gift_capacity(350)
        .magical_enhancements()
        .build();

    assert_eq!(sleigh.color(), "gold");
    assert_eq!(sleigh.engine(), "magic");
    assert_eq!(sleigh.gift_capacity(), 350);
    assert_eq!(sleigh.magical_enhancements(), true);
}
