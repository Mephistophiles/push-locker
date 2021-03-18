use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Locked {
    pub(crate) push_available: bool,
    pub(crate) locked_by: Option<String>,
}

#[derive(Default, Clone)]
pub(crate) struct Context {
    pub(crate) locked_by: Option<String>,
}

impl Context {
    pub(crate) fn get_lock_status(&self, my_username: &str) -> Locked {
        // push is available if:
        //  - locks is empty
        //  - this is just my lock
        let push_available =
            self.locked_by.is_none() || self.locked_by.as_deref() == Some(my_username);

        Locked {
            push_available,
            locked_by: self.locked_by.clone(),
        }
    }
}
