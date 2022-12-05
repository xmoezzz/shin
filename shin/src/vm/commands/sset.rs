use crate::vm::Vm;
use shin_core::vm::command;
use shin_core::vm::command::CommandResult;

pub struct SSET;

impl super::Command<command::runtime::SSET> for SSET {
    type Result = CommandResult;

    fn start(command: command::runtime::SSET, vm: &mut Vm) -> Self::Result {
        vm.state
            .globals_info
            .set(command.slot_number, command.value);
        command.token.finish()
    }
}