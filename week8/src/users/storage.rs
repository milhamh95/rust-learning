use std::collections::HashMap;
use crate::users::user::User;

pub struct UserStorage {
    users: HashMap<String, User>
}

impl UserStorage {
    pub fn new() -> Self {
        UserStorage {
            users: HashMap::new()
        }
    }

    pub fn create(&mut self, user: User) {
        if let Some(id) = &user.id {
            self.users.insert(id.clone(), user);
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn fetch(&self) -> Vec<User> {
        // Clone is necessary here to release the lock quickly.
        // This allows other requests to access storage while
        // the handler serializes the response.
        self.users.values().cloned().collect()
    }

    pub fn update(&mut self, id: &str, updated_user: User) -> Option<User> {
        if let Some(user) = self.users.get_mut(id) {
            // Only update name and email, preserve the ID
            user.name = updated_user.name;
            user.email = updated_user.email;
            Some(user.clone())
        } else {
            None
        }
    }

    pub fn delete(&mut self, id: &str) -> Option<User> {
        self.users.remove(id)
    }
}
