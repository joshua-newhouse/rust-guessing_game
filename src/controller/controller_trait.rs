use std::io::Result;

pub(crate) trait Controller {
    fn receive_user_input(&self) -> Result<String>;
}
