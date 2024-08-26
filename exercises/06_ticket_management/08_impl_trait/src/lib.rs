// TODO: Implement the `in_progress` method. It must return an iterator over the tickets in
//  `TicketStore` with status set to `Status::InProgress`.
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    // fnで直接traitをimplementすることもできる
    // ここでは IntoIterator トレイトを実装した in_progress メソッドを実装している
    fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        // イテレーターを取得して、filterでStatus::InProgressのものだけを取得している
        self.tickets
            .iter()
            .filter(|ticket| ticket.status == Status::InProgress)
        // .collect() を呼び出さないため、Vectorを作成せずにイテレーターを返す
        // これにより、参照(借用)のみ返す事ができ、コピーを作成する必要がない。
    }

    // collect() を呼び出すことで、Vectorを作成して返すこともできる
    fn to_dos(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|ticket| ticket.status == Status::ToDo)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn in_progress() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo);

        let in_progress = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(in_progress.clone());

        let in_progress_tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(in_progress_tickets.len(), 1);
        assert_eq!(in_progress_tickets[0], &in_progress);
    }
}
