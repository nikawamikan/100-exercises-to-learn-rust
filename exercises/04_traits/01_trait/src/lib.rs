// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

// trait は interface とほぼ同義の概念。
trait IsEven {
    // is_even の抽象メソッドを定義する。
    // つまり is_even メソッドを実装する型は、このメソッドを実装しなければならない。
    // このメソッドは引数を取らず、戻り値がbool型である。ということを意味している。
    fn is_even(&self) -> bool;
}

// 実装する型に対して trait を実装する。
// ここでは、u32型とi32型に対してそれぞれ実装している。
// 既にある型に対して新たな振る舞いを追加することができるのが trait の特徴。
// Javaなどでは、既存のクラスに新たなメソッドを追加することはできない。
// そのため、trait は非常に柔軟な設計を可能にする。
impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
