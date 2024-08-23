pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // これは、String構造体のサイズを表しており、実際にHeepに確保されるメモリ量は異なる。
        // あくまでStack上で確保されているメモリ量を表している。
        // stringは構造上3つのusize型(このサイズはプラットフォームによって異なるが、64bitOSが多いので8の場合が多い)を持つ。
        // これは、文字列の長さ、文字列の容量、文字列のポインタを表している。
        // Heepメモリでは、文字列の長さと容量が確保され、文字列のポインタがStackに確保される。
        // また、Heepメモリでのメモリ管理は、メモリアロケーターによって行われる。
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.

        // Ticket構造体は、String構造体を3つ持っているため、Stack上ではそれぞれのString構造体のサイズが積まれる。
        // これは、String構造体のサイズが24バイトであるため、72バイトとなる。(64bit OSの場合)
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
