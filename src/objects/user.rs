use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug)]
struct User {
    pub uuid: UserId,
    pub username: String,
}

impl User {
    fn new(username: String) -> Self {
        Self {
            username,
            uuid: UserId::new_v4(),
        }
    }
}
