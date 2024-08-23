// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

// 価格と数量を持つOrder構造体を定義する
struct Order {
    price: u32,
    quantity: u32,
}

// Order構造体に対して、数量が0より大きい場合はtrueを返すメソッドを実装する
// structとimplのように、構造体とメソッドを分けて定義することができる
// イメージとしてはclassを定義して、その中にメソッドを定義するような感じ
// これらの仕様はGo言語のstructに似ている
impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
