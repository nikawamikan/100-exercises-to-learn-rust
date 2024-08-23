// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

// 以下の関数は、2つの値のうち小さい方を返す。
// この関数は、PartialOrd トレイトを実装している型に対してのみ使用できるようにする。
// つまり、Tの型はPartialOrdトレイトを実装している型であれば、なんでも利用できるということである。
// 例えば min_u32(left: u32, right: u32) のように具象的に型を指定した場合、u64型などは指定できない。
// しかし、min<T: PartialOrd>(left: T, right: T) のようにジェネリクスを使うことで、PartialOrd トレイトを実装している型ならば、どんな型でも指定できるようになる。
// このように、ジェネリクスを使うことで、柔軟に型を指定できるようになる。
// また、この関数は`left: u64`, `right: u32` のように型が異なる場合にはコンパイルエラーとなる。（Tの型は同じである必要がある）

/// Return the minimum of two values.
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
