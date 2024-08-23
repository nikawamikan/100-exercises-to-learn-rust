// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// From トレイトを実装することで、型変換を行うことができる。
// 更に Into トレイトも自動的に実装される。
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    // `WrappingU32` に対して、u32型からの変換を行う。
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

// https://rust-exercises.com/100-exercises/04_traits/09_from
