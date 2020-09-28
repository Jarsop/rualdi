mod add;
mod init;
mod list;
mod remove;
mod resolve;

pub use add::Add;
pub use init::Init;
pub use list::List;
pub use remove::Remove;
pub use resolve::Resolve;

use anyhow::Result;

pub trait RadSubCmdRunnable {
    fn run(&self) -> Result<String>;
}
