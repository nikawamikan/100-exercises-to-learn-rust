# Enum

RustにおけるEnumの仕様について

- [Enum](#enum)
  - [基本的なEnumの利用方法](#基本的なenumの利用方法)
  - [他の言語との違い](#他の言語との違い)
  - [Enumを利用したOption型やResult型](#enumを利用したoption型やresult型)
    - [Enumを利用したOption型](#enumを利用したoption型)
    - [Enumを利用したResult型](#enumを利用したresult型)

## 基本的なEnumの利用方法

Enumは列挙型を表すもので、以下のように定義することができる。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;
    // switch文のように利用することができる
    match dir {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
    // 好きポイント: breakのような文が必要ないので簡潔でキレイ！
}
```

これは他言語でもおなじみの利用方法かと思う。  
以下はC#での例である。

```cs
public enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

public class Program
{
    public static void Main()
    {
        var dir = Direction.Up;
        switch (dir)
        {
            case Direction.Up:
                Console.WriteLine("Up");
                break;
            case Direction.Down:
                Console.WriteLine("Down");
                break;
            case Direction.Left:
                Console.WriteLine("Left");
                break;
            case Direction.Right:
                Console.WriteLine("Right");
                break;
        }
    }
}
```

RustのEnumは他の言語のEnumと同じように使うことができるが、他の言語とは違い、Enumは構造体や列挙型を組み合わせることができる。

## 他の言語との違い

RustではEnumは構造体や列挙型を組み合わせることができる。  
これにより、他の言語のクラスや構造体と同じような機能を持つことができる。

```rust
// 例: メッセージを表すEnum
enum Message {
    Quit, // 単純なEnum
    Move { x: i32, y: i32 }, // 構造体を含むEnum
    Write(String), // 列挙型を含むEnum
    ChangeColor(i32, i32, i32), // タプルを含むEnum
}

fn main() {
    // 移動メッセージを表すEnum
    // 移動先の座標を表す構造体を含む
    let hoge = Message::Move { x: 10, y: 20 };

    // それぞれの状態に対して処理を行う
    match hoge {
        Message::Quit => println!("Quit"),
        // 今回の例では、Moveの中にx, yが含まれているのでそれを取り出して表示する
        Message::Move { x, y } => println!("Move: x={}, y={}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("ChangeColor: r={}, g={}, b={}", r, g, b),
    }
}
```

このようなEnumは、他の言語ではクラスや構造体を使って表現することが多いが、RustではEnumを使って表現することができる。

```cs
// C#の例
public class Message
{
    // それぞれのイベントを表す列挙型
    public enum MessageType
    {
        Quit,
        Move,
        Write,
        ChangeColor,
    }

    // それぞれのイベントに対応するプロパティ
    public MessageType Type { get; set; }
    public int X { get; set; }
    public int Y { get; set; }
    public string Text { get; set; }
    public (int, int, int) Color { get; set; }
}

public class Program
{
    public static void Main()
    {
        // 移動メッセージを表すクラス
        var hoge = new Message
        {
            Type = Message.MessageType.Move,
            X = 10,
            Y = 20,
        };

        // それぞれの状態に対して処理を行う
        switch (hoge.Type)
        {
            case Message.MessageType.Quit:
                Console.WriteLine("Quit");
                break;
            // 移動する場所のx, yはそれぞれのプロパティに格納しているため、冗長になる
            case Message.MessageType.Move:
                Console.WriteLine($"Move: x={hoge.X}, y={hoge.Y}");
                break;
            case Message.MessageType.Write:
                Console.WriteLine($"Write: {hoge.Text}");
                break;
            case Message.MessageType.ChangeColor:
                Console.WriteLine($"ChangeColor: r={hoge.Color.Item1}, g={hoge.Color.Item2}, b={hoge.Color.Item3}");
                break;
        }
    }
}
```

このように、RustのEnumは他の言語のクラスや構造体と同じような機能を持つことができる。

## Enumを利用したOption型やResult型

Rustでは、Enumを利用してNullableや例外処理のような機能を実装している。  
これらはRustの標準ライブラリに含まれており、Option型やResult型として提供されている。  

面白いのは、これらの型はEnumを使って実装されているということである。  
必要な機能は最小化し、Rustの機能でNullableや例外処理を実装しているということがわかる。  
これはRustの哲学である「ゼロコスト抽象化」の一つの例であるといえそうだ。  
(ゼロコスト抽象化については[こちら](https://qiita.com/kichion/items/d5d87b30176e1d2d5f54)などを参照)

つまり、これらの機能は他の言語のように言語仕様として実装されているのではなく、Enumを使ってライブラリとして提供されているということである。

### Enumを利用したOption型

Option型はほかの言語でいうNullable型に相当するもので、値が存在しない場合にNone、存在する場合にSomeという値を持つ。

以下のようにOption型は定義されている。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

これを利用する場合、以下のように書くことができる。

```rust
fn divide(x: i32, y: i32) -> Option<i32> {
    // ゼロ割り算が発生する場合はNoneを返す
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    // ゼロ割り算が発生した場合はPanicする
    let x = divide(10, 0);
    match x {
        // 値が存在する場合はその値を表示する
        Some(v) => println!("{}", v),
        // もし、値が存在しない場合はPanicする
        None => panic!("division by zero"),
    }
}
```

このように、RustのEnumは他の言語のNullable型としても利用することができる。

### Enumを利用したResult型

先ほどの例では、ゼロ割り算が発生する場合にNoneを返していたが、これをResult型を使って表現したほうが自然と言える。  
Result型は、エラーが発生した場合にエラーの情報を持ち、正常な場合には値を持つ型である。  
以下のようにResult型は定義されている。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

これを利用する場合、以下のように書くことができる。

```rust
fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    // ゼロ割り算が発生する場合はエラーを返す
    if y == 0 {
        Err("division by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    // ゼロ割り算が発生した場合はPanicする
    let x = divide(10, 0).unwrap();
    println!("{}", x);
}
```

Rustには例外処理が存在していないが、Resultを利用することで例外処理のようにエラーを扱うことができる。
