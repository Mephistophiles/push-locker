use serde::Serialize;

#[derive(Serialize)]
pub struct Locked {
    pub(crate) push_available: bool,
    pub(crate) locked_by: Option<String>,
}

#[derive(Default, Clone)]
pub struct Context {
    pub locked_by: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unlocked() {
        let ctx = Context::default();
        let locked = ctx.get_lock_status("john");

        assert_eq!(locked.push_available, true);
        assert_eq!(locked.locked_by, None);
    }

    #[test]
    fn locked_by_me() {
        let mut ctx = Context::default();
        ctx.locked_by = Some("john".to_string());

        let locked = ctx.get_lock_status("john");

        assert_eq!(locked.push_available, true);
        assert_eq!(locked.locked_by.as_deref(), Some("john"));
    }

    #[test]
    fn locked_by_other() {
        let mut ctx = Context::default();
        ctx.locked_by = Some("a".to_string());

        let locked = ctx.get_lock_status("john");

        assert_eq!(locked.push_available, false);
        assert_eq!(locked.locked_by.as_deref(), Some("a"));
    }
}
