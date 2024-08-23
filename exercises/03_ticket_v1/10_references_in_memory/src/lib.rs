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
    fn u16_ref_size() {
        // これはu16型そのものではなく、u16型の参照を表している。
        // 参照はポインタであり、64bitOSの場合8バイトである。
        // そのため、u16型の参照は8バイトである。
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        // u64型の可変参照は8バイトである。
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        // Ticket構造体の参照は8バイトである。
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
