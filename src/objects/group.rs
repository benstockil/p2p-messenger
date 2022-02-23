use uuid::Uuid;
use super::UserId;

pub type GroupId = Uuid;

pub struct Group {
    id: GroupId,
    members: Vec<UserId>,
    name: String, 
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: GroupId::new_v4(),
            members: Vec::new(),
            name,
        }
    }

    pub fn add_user(&mut self, user: UserId) {
        self.members.push(user);
    }
}
