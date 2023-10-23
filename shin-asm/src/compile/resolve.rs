use crate::compile::def_map::{RegisterName, ResolveKind};
use crate::{
    compile::{Db, DefMap},
    syntax::ast::{self},
};
use shin_core::format::scenario::instruction_elements::Register;

#[derive(Debug, Copy, Clone)]
enum ResolveContextInner {
    Empty,
    Real {
        def_map: DefMap,
        resolve_kind: ResolveKind,
    },
}

pub struct ResolveContext<'a> {
    db: &'a dyn Db,
    inner: ResolveContextInner,
}

impl<'a> ResolveContext<'a> {
    pub fn new_empty(db: &'a dyn Db) -> Self {
        Self {
            db,
            inner: ResolveContextInner::Empty,
        }
    }

    pub fn new(db: &'a dyn Db, def_map: DefMap, resolve_kind: ResolveKind) -> Self {
        Self {
            db,
            inner: ResolveContextInner::Real {
                def_map,
                resolve_kind,
            },
        }
    }

    pub fn resolve_register(&self, register: &ast::RegisterIdentKind) -> Option<Register> {
        match register {
            &ast::RegisterIdentKind::Register(register) => Some(register),
            ast::RegisterIdentKind::Alias(name) => match self.inner {
                ResolveContextInner::Empty => None,
                ResolveContextInner::Real {
                    def_map,
                    resolve_kind,
                } => def_map.resolve_register(self.db, RegisterName(name.clone()), resolve_kind),
            },
        }
    }

    // pub fn resolve_definition(&self, _name: &Name) -> Option<DefRef> {
    //     todo!()
    // }
}
