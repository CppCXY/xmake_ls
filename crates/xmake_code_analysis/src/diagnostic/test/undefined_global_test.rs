#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_250() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();
        assert!(ws.check_code_for(
            DiagnosticCode::UndefinedGlobal,
            r#"
            --- @class A
            --- @field field any
            local A = {}

            function A:method()
            pcall(function()
                return self.field
            end)
            end
            "#
        ));
    }
}
