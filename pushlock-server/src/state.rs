use crossbeam_channel::{Receiver, Sender};
use serde::Serialize;

#[derive(Serialize)]
pub struct LockError {
    pub push_available: bool,
    pub locked_by: Option<String>,
}

#[derive(Clone)]
pub struct Context {
    pub rx_channel: Receiver<Option<String>>,
    pub tx_channel: Sender<Option<String>>,
    pub locked_by: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        let (tx_channel, rx_channel) = crossbeam_channel::unbounded();
        let locked_by = None;

        Self {
            rx_channel,
            tx_channel,
            locked_by,
        }
    }
}

impl Context {
    fn check_locked_by(&self, user: &str) -> Result<(), LockError> {
        if self.locked_by.is_none() {
            return Ok(());
        }

        if self.locked_by.as_deref() != Some(user) {
            return Err(LockError {
                push_available: false,
                locked_by: self.locked_by.clone(),
            });
        }

        Ok(())
    }
}

impl Context {
    pub fn lock(&mut self, user: String) -> Result<(), LockError> {
        self.check_locked_by(&user)?;
        self.locked_by = Some(user);
        self.tx_channel.send(self.locked_by.clone()).unwrap();

        Ok(())
    }

    pub fn unlock(&mut self, user: String) -> Result<(), LockError> {
        self.check_locked_by(&user)?;
        self.locked_by.take();
        self.tx_channel.send(self.locked_by.clone()).unwrap();

        Ok(())
    }

    pub fn get_lock_status(&self, my_username: &str) -> LockError {
        // push is available if:
        //  - locks is empty
        //  - this is just my lock
        let push_available =
            self.locked_by.is_none() || self.locked_by.as_deref() == Some(my_username);

        LockError {
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
