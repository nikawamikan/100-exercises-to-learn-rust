// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

// ドロップトレイトを実装することで、DropBombがドロップされたときにpanicを発生させる
// ドロップのタイミングは以下の時
// 1. スコープを抜けるとき
// 2. ヒープ上のメモリを解放するとき
impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Boom!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        // DropBombがドロップされたときにpanicが発生することを確認する
        let bomb = DropBomb::new();
        // The bomb should panic when dropped

        // bombはスコープを抜ける際にDropトレイトが実行される
        // Dropトレイトのdropメソッド内でpanicが発生する
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();

        // DropBombのdefuseメソッドを呼び出すことでpanicを回避する
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused

        // bombはスコープを抜ける際にDropトレイトが実行される
        // Dropトレイトのdropメソッド内でpanicが発生しない
    }
}
