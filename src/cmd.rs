use rust_util::XResult;

#[derive(Clone, Debug)]
pub enum CommandSupportOS {
    Linux,
    MacOS,
}

pub type FnCallCommand = fn(bool) -> XResult<()>;

pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub support_os: Vec<CommandSupportOS>,
    pub command_fn: FnCallCommand,
}
