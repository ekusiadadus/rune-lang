use crate::compiling::v1::assemble::prelude::*;

/// Compile a literal string `b"Hello World"`.
impl Assemble for ast::LitByteStr {
    fn assemble(&self, c: &mut Compiler<'_, '_>, needs: Needs) -> CompileResult<Asm> {
        let span = self.span();
        log::trace!("LitByteStr => {:?}", c.source.source(span));

        // NB: Elide the entire literal if it's not needed.
        if !needs.value() {
            c.diagnostics.not_used(c.source_id, span, c.context());
            return Ok(Asm::top(span));
        }

        let bytes = self.resolve(c.query.storage(), c.sources)?;
        let slot = c.query.unit_mut().new_static_bytes(span, &*bytes)?;
        c.asm.push(Inst::Bytes { slot }, span);
        Ok(Asm::top(span))
    }
}
