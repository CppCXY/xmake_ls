#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_disable_nextline() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_code_for(
            DiagnosticCode::SyntaxError,
            r#"
        ---@diagnostic disable-next-line: syntax-error
        ---@param
        local function f() end
        "#,
        ));
    }
}
