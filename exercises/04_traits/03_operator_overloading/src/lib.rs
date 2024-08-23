use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: Implement the `PartialEq` trait for `Ticket`.

impl PartialEq for Ticket {
    // PartialEq トレイトの eq メソッドを実装する。
    // このメソッドは、自分自身と他の値を比較して、等しいかどうかを判定する。
    // このメソッドは、== 演算子を使って比較する際に呼び出される。
    // なお、実装する際は自動導出を使うことができる。（#[derive(PartialEq)]）
    // 自動導出を使う場合は、以下のようになる。
    // #[derive(PartialEq)]
    // struct Ticket {
    //     title: String,
    //     description: String,
    //     status: String,
    // }

    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.status == other.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 == ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert!(ticket1 != ticket2);
    }
}
