pub mod layer;

use crate::format::scenario::instructions::{
    BitmaskNumberArray, MemoryAddress, NumberSpec, StringArray,
};
use crate::format::scenario::{U16String, U8SmallNumberList, U8String};
use shin_derive::Command;

// those are actually used by the generated code (it's a bit messy, i know)
#[allow(unused)]
use layer::{LayerProperty, LayerType, VLayerId};

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Command, Debug)]
// this is a fake command, real commands are generated by the derive macro
// we want each command to be a separate struct, but defining them is much easier with one enum
//
// The derive macro will generate two versions of each command:
// - A compile time representation, which is mostly the same as this enum
// - A runtime representation, which has some types replaced by their runtime equivalents (e.g. `NumberSpec` -> `i32`)
//
// TODO: describe logic with commands that return a value (it's a bit complicated and i haven't thought it through yet)
// TODO: maybe UX of the derive macro is not the best. consider using build.rs-based codegen
pub enum Command {
    #[cmd(opcode = 0x00u8)]
    EXIT {
        /// This is encoded in the instruction
        /// If it's zero then the VM shuts down
        /// If it's nonzero then the VM treats it as a NOP
        /// Maybe it's a feature that is not used for umineko?
        arg1: u8,
        ///
        arg2: NumberSpec,
    },

    #[cmd(opcode = 0x81u8)]
    SGET {
        #[cmd(dest)]
        dest: MemoryAddress,
        slot_number: NumberSpec,
    },
    #[cmd(opcode = 0x82u8)]
    SSET {
        slot_number: NumberSpec,
        value: NumberSpec,
    },
    #[cmd(opcode = 0x83u8)]
    WAIT {
        wait_kind: u8,
        wait_amount: NumberSpec,
    },
    // 0x84 is unused
    #[cmd(opcode = 0x85u8)]
    MSGINIT { arg: NumberSpec },
    #[cmd(opcode = 0x86u8)]
    MSGSET { msg_id: u32, text: U16String }, // TODO: this string needs a fixup (see ShinDataUtil's OpcodeDefinitions.NeedsStringFixup)
    #[cmd(opcode = 0x87u8)]
    MSGWAIT { arg: NumberSpec },
    #[cmd(opcode = 0x88u8)]
    MSGSIGNAL {},
    #[cmd(opcode = 0x89u8)]
    MSGSYNC { arg1: NumberSpec, arg2: NumberSpec },
    #[cmd(opcode = 0x8au8)]
    MSGCLOSE { arg: u8 },

    #[cmd(opcode = 0x8du8)]
    SELECT {
        choice_set_base: u16,
        choice_index: u16,
        #[cmd(dest)]
        dest: MemoryAddress,
        arg4: NumberSpec,
        choice_title: U16String,
        variants: StringArray,
    },
    #[cmd(opcode = 0x8eu8)]
    WIPE {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
        params: BitmaskNumberArray,
    },
    #[cmd(opcode = 0x8fu8)]
    WIPEWAIT {},
    #[cmd(opcode = 0x90u8)]
    BGMPLAY {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
        arg4: NumberSpec,
    },
    #[cmd(opcode = 0x91u8)]
    BGMSTOP { arg: NumberSpec },
    #[cmd(opcode = 0x92u8)]
    BGMVOL { arg1: NumberSpec, arg2: NumberSpec },
    #[cmd(opcode = 0x93u8)]
    BGMWAIT { arg: NumberSpec },
    #[cmd(opcode = 0x94u8)]
    BGMSYNC { arg: NumberSpec },
    #[cmd(opcode = 0x95u8)]
    SEPLAY {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
        arg4: NumberSpec,
        arg5: NumberSpec,
        arg6: NumberSpec,
        arg7: NumberSpec,
    },
    #[cmd(opcode = 0x96u8)]
    SESTOP { arg1: NumberSpec, arg2: NumberSpec },
    #[cmd(opcode = 0x97u8)]
    SESTOPALL { arg: NumberSpec },
    #[cmd(opcode = 0x98u8)]
    SEVOL {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
    },
    #[cmd(opcode = 0x99u8)]
    SEPAN {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
    },
    #[cmd(opcode = 0x9au8)]
    SEWAIT { arg1: NumberSpec, arg2: NumberSpec },
    #[cmd(opcode = 0x9bu8)]
    SEONCE {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
        arg4: NumberSpec,
        arg5: NumberSpec,
    },
    #[cmd(opcode = 0x9cu8)]
    VOICEPLAY {
        name: U16String,
        arg1: NumberSpec,
        arg2: NumberSpec,
    },
    #[cmd(opcode = 0x9du8)]
    VOICESTOP {},
    #[cmd(opcode = 0x9eu8)]
    VOICEWAIT { arg: NumberSpec },
    #[cmd(opcode = 0x9fu8)]
    SYSSE { arg1: NumberSpec, arg2: NumberSpec },

    #[cmd(opcode = 0xa0u8)]
    SAVEINFO { level: NumberSpec, info: U16String }, // TODO: this string needs a fixup (see ShinDataUtil's OpcodeDefinitions.NeedsStringFixup)
    #[cmd(opcode = 0xa1u8)]
    AUTOSAVE {},
    #[cmd(opcode = 0xa2u8)]
    EVBEGIN { arg: NumberSpec },
    #[cmd(opcode = 0xa3u8)]
    EVEND {},
    #[cmd(opcode = 0xa4u8)]
    RESUMESET {},
    #[cmd(opcode = 0xa5u8)]
    RESUME {},
    #[cmd(opcode = 0xa6u8)]
    SYSCALL { arg1: NumberSpec, arg2: NumberSpec },

    #[cmd(opcode = 0xb0u8)]
    TROPHY { arg: NumberSpec },
    #[cmd(opcode = 0xb1u8)]
    UNLOCK { arg1: u8, arg2: U8SmallNumberList },

    /// Reset property values to their initial state
    #[cmd(opcode = 0xc0u8)]
    LAYERINIT {
        #[cmd(rty = "VLayerId")]
        arg: NumberSpec,
    },
    /// Load a layer resource or smth
    /// There are multiple layer types and they have different arguments
    #[cmd(opcode = 0xc1u8)]
    LAYERLOAD {
        #[cmd(rty = "VLayerId")]
        layer_id: NumberSpec,
        #[cmd(rty = "LayerType")]
        layer_type: NumberSpec,
        // TODO: what does this mean again?
        leave_uninitialized: NumberSpec,
        params: BitmaskNumberArray,
    },
    #[cmd(opcode = 0xc2u8)]
    LAYERUNLOAD {
        #[cmd(rty = "VLayerId")]
        layer_id: NumberSpec,
        delay_time: NumberSpec,
    },
    /// Change layer property, possibly through a transition.
    #[cmd(opcode = 0xc3u8)]
    LAYERCTRL {
        #[cmd(rty = "VLayerId")]
        layer_id: NumberSpec,
        #[cmd(rty = "LayerProperty")]
        property_id: NumberSpec,
        // in the params there are (always?) three numbers
        // ctrl_value, ctrl_time and ctrl_flags
        params: BitmaskNumberArray,
    },
    /// Wait for property transitions to finish.
    #[cmd(opcode = 0xc4u8)]
    LAYERWAIT {
        #[cmd(rty = "VLayerId")]
        layer_id: NumberSpec,
        wait_properties: U8SmallNumberList,
    },
    #[cmd(opcode = 0xc5u8)]
    LAYERSWAP { arg1: NumberSpec, arg2: NumberSpec },
    /// Select a subset of layers to perform batch operations
    /// (TODO: fact check) These can be used as layer_id = -4
    #[cmd(opcode = 0xc6u8)]
    LAYERSELECT {
        selection_start_id: NumberSpec,
        selection_end_id: NumberSpec,
    },
    #[cmd(opcode = 0xc7u8)]
    MOVIEWAIT { arg1: NumberSpec, arg2: NumberSpec },
    // 0xc8 unused
    #[cmd(opcode = 0xc9u8)]
    TRANSSET {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
        params: BitmaskNumberArray,
    },
    #[cmd(opcode = 0xcau8)]
    TRANSWAIT { arg: NumberSpec },
    #[cmd(opcode = 0xcbu8)]
    PAGEBACK {},
    #[cmd(opcode = 0xccu8)]
    PLANESELECT { arg: NumberSpec },
    #[cmd(opcode = 0xcdu8)]
    PLANECLEAR {},
    #[cmd(opcode = 0xceu8)]
    MASKLOAD {
        arg1: NumberSpec,
        arg2: NumberSpec,
        arg3: NumberSpec,
    },
    #[cmd(opcode = 0xcfu8)]
    MASKUNLOAD {},

    #[cmd(opcode = 0xe0u8)]
    CHARS { arg1: NumberSpec, arg2: NumberSpec },
    #[cmd(opcode = 0xe1u8)]
    TIPSGET { arg: U8SmallNumberList },
    #[cmd(opcode = 0xe2u8)]
    QUIZ {
        #[cmd(dest)]
        dest: MemoryAddress,
        arg: NumberSpec,
    },
    #[cmd(opcode = 0xe3u8)]
    SHOWCHARS {},
    #[cmd(opcode = 0xe4u8)]
    NOTIFYSET { arg: NumberSpec },

    #[cmd(opcode = 0xffu8)]
    DEBUGOUT {
        format: U8String,
        args: U8SmallNumberList,
    },
}

#[derive(Debug, Clone)]
pub enum CommandResult {
    None,
    WriteMemory(MemoryAddress, i32),
}

impl RuntimeCommand {
    #[inline]
    pub fn execute_dummy(self) -> Option<CommandResult> {
        Some(match self {
            RuntimeCommand::EXIT(_) => {
                // TODO: actually the logic behind this is a bit more complex
                // works for now though
                return None;
            }
            RuntimeCommand::SGET(cmd) => cmd.token.finish(0),
            RuntimeCommand::SSET(cmd) => cmd.token.finish(),
            RuntimeCommand::WAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGINIT(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGSET(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGSIGNAL(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGSYNC(cmd) => cmd.token.finish(),
            RuntimeCommand::MSGCLOSE(cmd) => cmd.token.finish(),
            RuntimeCommand::SELECT(cmd) => cmd.token.finish(0),
            RuntimeCommand::WIPE(cmd) => cmd.token.finish(),
            RuntimeCommand::WIPEWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::BGMPLAY(cmd) => cmd.token.finish(),
            RuntimeCommand::BGMSTOP(cmd) => cmd.token.finish(),
            RuntimeCommand::BGMVOL(cmd) => cmd.token.finish(),
            RuntimeCommand::BGMWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::BGMSYNC(cmd) => cmd.token.finish(),
            RuntimeCommand::SEPLAY(cmd) => cmd.token.finish(),
            RuntimeCommand::SESTOP(cmd) => cmd.token.finish(),
            RuntimeCommand::SESTOPALL(cmd) => cmd.token.finish(),
            RuntimeCommand::SEVOL(cmd) => cmd.token.finish(),
            RuntimeCommand::SEPAN(cmd) => cmd.token.finish(),
            RuntimeCommand::SEWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::SEONCE(cmd) => cmd.token.finish(),
            RuntimeCommand::VOICEPLAY(cmd) => cmd.token.finish(),
            RuntimeCommand::VOICESTOP(cmd) => cmd.token.finish(),
            RuntimeCommand::VOICEWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::SYSSE(cmd) => cmd.token.finish(),
            RuntimeCommand::SAVEINFO(cmd) => cmd.token.finish(),
            RuntimeCommand::AUTOSAVE(cmd) => cmd.token.finish(),
            RuntimeCommand::EVBEGIN(cmd) => cmd.token.finish(),
            RuntimeCommand::EVEND(cmd) => cmd.token.finish(),
            RuntimeCommand::RESUMESET(cmd) => cmd.token.finish(),
            RuntimeCommand::RESUME(cmd) => cmd.token.finish(),
            RuntimeCommand::SYSCALL(cmd) => cmd.token.finish(),
            RuntimeCommand::TROPHY(cmd) => cmd.token.finish(),
            RuntimeCommand::UNLOCK(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERINIT(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERLOAD(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERUNLOAD(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERCTRL(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERSWAP(cmd) => cmd.token.finish(),
            RuntimeCommand::LAYERSELECT(cmd) => cmd.token.finish(),
            RuntimeCommand::MOVIEWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::TRANSSET(cmd) => cmd.token.finish(),
            RuntimeCommand::TRANSWAIT(cmd) => cmd.token.finish(),
            RuntimeCommand::PAGEBACK(cmd) => cmd.token.finish(),
            RuntimeCommand::PLANESELECT(cmd) => cmd.token.finish(),
            RuntimeCommand::PLANECLEAR(cmd) => cmd.token.finish(),
            RuntimeCommand::MASKLOAD(cmd) => cmd.token.finish(),
            RuntimeCommand::MASKUNLOAD(cmd) => cmd.token.finish(),
            RuntimeCommand::CHARS(cmd) => cmd.token.finish(),
            RuntimeCommand::TIPSGET(cmd) => cmd.token.finish(),
            RuntimeCommand::QUIZ(cmd) => cmd.token.finish(0),
            RuntimeCommand::SHOWCHARS(cmd) => cmd.token.finish(),
            RuntimeCommand::NOTIFYSET(cmd) => cmd.token.finish(),
            RuntimeCommand::DEBUGOUT(cmd) => cmd.token.finish(),
        })
    }
}
