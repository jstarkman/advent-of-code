use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// TODO: Define the `status` method for all states
pub trait State {
    fn status() -> &'static str;
}

macro_rules! impl_state {
    ($t:ty) => {
        impl State for $t {
            fn status() -> &'static str {
                stringify!($t)
            }
        }
    };
}
impl_state!(Empty);
impl_state!(Ready);
impl_state!(Flying);

pub struct Sleigh<T>
where
    T: State,
{
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

impl<T> Sleigh<T>
where
    T: State,
{
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}
