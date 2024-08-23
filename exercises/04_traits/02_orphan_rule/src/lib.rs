// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }

// error[E0117]: only traits defined in the current crate can be implemented for primitive types
//  --> exercises/04_traits/02_orphan_rule/src/lib.rs:7:1
//   |
// 7 | impl PartialEq for u32 {
//   | ^^^^^---------^^^^^---
//   | |    |             |
//   | |    |             `u32` is not defined in the current crate
//   | |    `u32` is not defined in the current crate
//   | impl doesn't use only types from inside the current crate
//   |
//   = note: define and implement a trait or new type instead

// オーファンルールとは、トレイトの実装がトレイトや対象の型が定義されているクレートの中にない場合に発生する問題。
// トレイトの実装は、トレイトや対象の型が定義されているクレートの中にないといけない。
// これは、トレイトの実装が対象の型に対して複数のクレートで実装されている場合に、どの実装を使うかが曖昧になるためである。
// この問題を解決するために、新たなトレイトを定義し、そのトレイトを対象の型に対して実装することで解決することができる。
// この問題は、Rustの安全性を保つために導入されている。
