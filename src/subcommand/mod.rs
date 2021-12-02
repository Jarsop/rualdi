mod add;
mod add_env;
mod completions;
mod init;
mod list;
mod list_alias;
mod list_env;
mod remove;
mod remove_env;
mod resolve;
mod resolve_env;

pub use add::Add;
pub use add_env::AddEnv;
pub use completions::Completions;
pub use init::Init;
pub use list::List;
pub use list_alias::ListAlias;
pub use list_env::ListEnv;
pub use remove::Remove;
pub use remove_env::RemoveEnv;
pub use resolve::Resolve;
pub use resolve_env::ResolveEnv;

use anyhow::Result;

pub trait RadSubCmdRunnable {
    fn run(&self) -> Result<String>;
}
