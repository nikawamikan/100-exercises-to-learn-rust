// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// `Power` トレイトを定義する。
// `Rhs`は、ジェネリック型で、デフォルトは `Self` とする。
pub trait Power<Rhs = Self> {
    // `power` メソッドを定義する。
    fn power(self, n: Rhs) -> Self;
}

// `u32` 型に `Power` トレイトを実装する。
impl Power for u32 {
    // デフォルトのジェネリック型は `Self` とする。
    fn power(self, n: Self) -> Self {
        // `n` 乗を計算する。
        self.pow(n)
    }
}

// 借用された `u32` 型を受け取る `Power` トレイトの実装を行う。
impl Power<&u32> for u32 {
    fn power(self, n: &u32) -> Self {
        // この時、powerを使うことで、実装を再利用することができる。
        // この実装は、`u32` 型の値を受け取る実装と同じである。
        self.power(*n)
    }
}

// `u16` 型を受け取る `Power` トレイトを実装する。
impl Power<u16> for u32 {
    fn power(self, n: u16) -> Self {
        // 最初のPowerの実装を再利用するために、`u16` 型を受け取る実装を行う。
        // この時、`u16` 型を `u32` 型に変換して、再利用する。
        self.power(n as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
