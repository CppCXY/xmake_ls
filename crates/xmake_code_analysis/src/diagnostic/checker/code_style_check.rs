use crate::{DiagnosticCode, SemanticModel};

use super::{Checker, DiagnosticContext};

pub struct CodeStyleCheckChecker;

impl Checker for CodeStyleCheckChecker {
    const CODES: &[DiagnosticCode] = &[DiagnosticCode::CodeStyleCheck];

    fn check(_: &mut DiagnosticContext, _: &SemanticModel) {}
}
