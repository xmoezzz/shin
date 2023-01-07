use super::prelude::*;

impl super::StartableCommand for command::runtime::CHARS {
    fn apply_state(&self, _state: &mut VmState) {
        warn!("TODO: CHARS state: {:?}", self);
    }

    fn start(
        self,
        _context: &UpdateContext,
        _scenario: &Arc<Scenario>,
        _vm_state: &VmState,
        _adv_state: &mut AdvState,
    ) -> CommandStartResult {
        warn!("TODO: CHARS: {:?}", self);
        self.token.finish().into()
    }
}
