use crate::compile::def_map::ResolveKind;
use crate::compile::diagnostics::HirDiagnosticAccumulator;
use crate::compile::from_hir::HirBlockId;
use crate::compile::types::SalsaBlockIdWithFile;
use crate::compile::{
    hir, BlockIdWithFile, Db, DefMap, HirBlockBody, HirDiagnosticCollector,
    HirDiagnosticCollectorWithBlock, ResolveContext, WithFile,
};
use binrw::io::NoSeek;
use binrw::BinWrite;
use shin_core::format::scenario::instructions::Instruction;
use std::io;

struct CountWrite {
    count: u64,
}

impl CountWrite {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

impl io::Write for CountWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = buf.len();
        self.count += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Stores the lowered instructions block. The instructions are final, except they don't have fixed addresses yet (kinda like a relocatable object file ig).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredBlock {
    /// Stores the lowered instructions. `None` means that the instruction was not lowered due to an error.
    ///
    /// All the `CodeAddress` elements have zero value, as the addresses in the final file are not yet known at this stages.
    /// They are stored as `BlockIdWithFile` in the `code_addresses` field instead.
    // NOTE: this __can__ be replaced by another type of lowered, but not yet placed instruction, but we opt not to do so
    // it's easier or smth
    pub instructions: Vec<Option<Instruction>>,
    /// Stores the actual values that `CodeAddress` elements in `instructions` refer to.
    pub code_addresses: Vec<BlockIdWithFile>,
}

impl LoweredBlock {
    pub fn from_hir(
        diagnostics: &mut HirDiagnosticCollectorWithBlock,
        resolve_ctx: &ResolveContext,
        block: &HirBlockBody,
    ) -> Self {
        let mut instructions = Vec::with_capacity(block.instructions.len());
        let mut code_addresses = Vec::new();

        for (instr, _) in block.instructions.iter() {
            instructions.push(super::instruction::instruction_from_hir(
                diagnostics,
                resolve_ctx,
                &mut code_addresses,
                block,
                instr,
            ));
        }

        Self {
            instructions,
            code_addresses,
        }
    }

    /// Checks whether all the instructions in the block are lowered.
    pub fn complete(&self) -> bool {
        self.instructions.iter().all(|instr| instr.is_some())
    }

    /// Computes the size of the serialized block in bytes
    pub fn size(&self) -> Option<u32> {
        let mut size = 0;
        for instr in &self.instructions {
            let instr = instr.as_ref()?;

            let mut count_write = NoSeek::new(CountWrite::new());
            instr
                .write(&mut count_write)
                .expect("BUG: failed to write instruction");

            size += count_write.into_inner().count();
        }

        Some(size.try_into().expect("BUG: block size overflow"))
    }

    pub fn debug_dump(&self) -> String {
        use std::fmt::Write;
        let mut buf = String::new();
        writeln!(buf, "instructions:").unwrap();

        for instr in &self.instructions {
            match instr {
                None => writeln!(buf, "  <error>").unwrap(),
                // TODO: make a reasonable `Display` impl?
                Some(instr) => writeln!(buf, "  {:?}", instr).unwrap(),
            }
        }

        writeln!(buf, "code addresses:").unwrap();
        for code_address in &self.code_addresses {
            writeln!(buf, "  {:?}", code_address).unwrap();
        }

        buf
    }
}

#[salsa::tracked]
pub fn lower_block(db: &dyn Db, def_map: DefMap, block: SalsaBlockIdWithFile) -> LoweredBlock {
    use crate::compile::MakeWithFile;

    let WithFile {
        file,
        value: block_id,
    } = block.block_id(db);
    let block_bodies = hir::collect_file_bodies(db, file);
    let block_hir = block_bodies.get_block(db, block_id).unwrap();

    let mut diagnostics = HirDiagnosticCollector::new();
    let resolve_ctx = ResolveContext::new(
        db,
        def_map,
        ResolveKind::LocalAndGlobal(block_id.in_file(file)),
    );

    let result = LoweredBlock::from_hir(
        &mut diagnostics
            .with_file(file)
            .with_block(HirBlockId::Block(block_id)),
        &resolve_ctx,
        &block_hir,
    );

    for diag in diagnostics.into_diagnostics() {
        HirDiagnosticAccumulator::push(db, diag)
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::compile::def_map::build_def_map;
    use crate::compile::diagnostics::{HirDiagnosticAccumulator, SourceDiagnosticAccumulator};
    use crate::compile::hir::lower::test_utils;
    use crate::compile::types::SalsaBlockIdWithFile;
    use crate::compile::{hir, File, MakeWithFile, Program};
    use expect_test::{expect, Expect};
    use indoc::indoc;

    fn check_from_hir(source: &str, expected: Expect) {
        use crate::compile::db::Database;
        use std::fmt::Write;

        let db = Database::default();
        let db = &db;

        let file = File::new(db, "test.sal".to_string(), source.to_string());
        let program = Program::new(db, vec![file]);
        let def_map = build_def_map(db, program);

        let bodies = hir::collect_file_bodies(db, file);

        let block_ids = bodies.get_block_ids(db);
        assert_eq!(block_ids.len(), 1, "expected exactly one block");
        let block_id = block_ids[0];

        // put it into a salsa interner
        let block = SalsaBlockIdWithFile::new(db, block_id.in_file(file));
        let lowered = super::lower_block(db, def_map, block);

        let hir_errors =
            super::lower_block::accumulated::<HirDiagnosticAccumulator>(db, def_map, block);
        let source_errors =
            super::lower_block::accumulated::<SourceDiagnosticAccumulator>(db, def_map, block);
        let diags = test_utils::diagnostics_to_str(db, hir_errors, source_errors);

        let mut result = String::new();
        if !diags.is_empty() {
            writeln!(result, "Diagnostics:\n{}", diags).unwrap();
        }

        write!(result, "{}", lowered.debug_dump()).unwrap();

        expected.assert_eq(&result);
    }

    #[test]
    pub fn check_basic() {
        check_from_hir(
            r#"
            zero $v0
            abs $v1, 42
            not16 $v2, $v1
            "#,
            expect![[r#"
                instructions:
                  uo(UnaryOperation { ty: Zero, destination: $v0, source: 0 })
                  uo(UnaryOperation { ty: Abs, destination: $v1, source: 42 })
                  uo(UnaryOperation { ty: Not16, destination: $v2, source: $v1 })
                code addresses:
            "#]],
        );
    }

    #[test]
    pub fn check_error() {
        check_from_hir(
            indoc! {r#"
                x 96
                42
            "#},
            expect![[r#"
                Diagnostics:
                Error: expected an instruction or label
                   ╭─[test.sal:2:1]
                   │
                 2 │ 42
                   │ ─  
                   │     
                ───╯


                Error: Unknown instruction: `x`
                   ╭─[test.sal:1:1]
                   │
                 1 │ x 96
                   │ ─────  
                   │         
                ───╯

                instructions:
                  <error>
                code addresses:
            "#]],
        );

        check_from_hir(
            indoc! {r#"
                zero $v0 aslk as
                abs 42, 42
                not16 $v2, $v1
            "#},
            expect![[r#"
                Diagnostics:
                Error: expected COMMA
                   ╭─[test.sal:1:9]
                   │
                 1 │ zero $v0 aslk as
                   │         ─  
                   │             
                ───╯


                Error: expected COMMA
                   ╭─[test.sal:1:14]
                   │
                 1 │ zero $v0 aslk as
                   │              ─  
                   │                  
                ───╯


                Error: Expected no more than 2 arguments
                   ╭─[test.sal:1:15]
                   │
                 1 │ zero $v0 aslk as
                   │               ──  
                   │                    
                ───╯


                Error: Expected either a number or a register, found a name reference
                   ╭─[test.sal:1:10]
                   │
                 1 │ zero $v0 aslk as
                   │          ────  
                   │                 
                ───╯


                Error: Expected a register, but got an integer literal
                   ╭─[test.sal:2:5]
                   │
                 2 │ abs 42, 42
                   │     ──  
                   │          
                ───╯

                instructions:
                  uo(UnaryOperation { ty: Zero, destination: $v0, source: 0 })
                  <error>
                  uo(UnaryOperation { ty: Not16, destination: $v2, source: $v1 })
                code addresses:
            "#]],
        );
    }
}