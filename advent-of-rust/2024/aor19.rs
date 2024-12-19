use std::marker::PhantomData;

// 1. We have 3 states:
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Sleigh struct definition
pub struct Sleigh<T> {
    __state: PhantomData<T>,
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Sleigh<Empty> {
        Self {
            __state: PhantomData,
        }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            __state: PhantomData,
        }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            __state: PhantomData,
        }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            __state: PhantomData,
        }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            __state: PhantomData,
        }
    }
}
