/// Given the start and end points of a journey, and the time it took to complete the journey,
/// calculate the average speed of the journey.
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message if `time_elapsed` is 0

    // 0割り算をした場合にpanicを発生させる
    // panicはプログラムが異常終了することを意味する
    // そのため、実用上ではクリティカルなエラーが発生した場合にのみ使用するべき
    if time_elapsed == 0 {
        panic!("The journey took no time at all, that's impossible!");
    }

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    //    under test to panic. We can also check the panic message by using `expected`.
    //    This is all part of Rust's built-in test framework!
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
