use std::fmt::{Display, Formatter, Result};

macro_rules! display_named_gift {
    ($t:ident) => {
        impl Display for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.write_str(&self.name)
            }
        }
    };
}

pub struct KidsGift {
    pub name: String,
}
display_named_gift!(KidsGift);

pub struct ElvesGift {
    pub name: String,
}
display_named_gift!(ElvesGift);

pub struct ReindeerGift {
    pub name: String,
}
display_named_gift!(ReindeerGift);

pub fn display_gift<G>(gift: G)
where
    G: Display,
{
    println!("{}", gift);
}

pub fn main() {
    let kids_gift = KidsGift {
        name: "toy car".to_string(),
    };
    let elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
    };
    let reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
    };

    display_gift(&kids_gift);
    display_gift(&elves_gift);
    display_gift(&reindeer_gift);
}
