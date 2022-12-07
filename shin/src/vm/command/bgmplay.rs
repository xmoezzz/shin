use super::prelude::*;

impl super::StartableCommand for command::runtime::BGMPLAY {
    fn apply_state(&self, _state: &mut VmState) {
        warn!("TODO: BGMPLAY state: {:?}", self);
    }

    fn start(self, _vm: &mut Vm) -> CommandStartResult {
        warn!("TODO: BGMPLAY: {:?}", self);
        self.token.finish().into()
    }
}