#[cfg(test)]
mod test {
    use crate::DiagnosticCode;

    #[test]
    fn test_feat_724() {
        let mut ws = crate::VirtualWorkspace::new_with_init_std_lib();

        assert!(!ws.check_code_for(
            DiagnosticCode::PreferredLocalAlias,
            r#"
            local gsub = string.gsub
            print(string.gsub("hello", "l", "0"))
            "#,
        ));

        assert!(ws.check_code_for(
            DiagnosticCode::PreferredLocalAlias,
            r#"
            local t = {
                a = ""
            }
            local h = t.a
            t.a = 'h'
            print(t.a)
            "#,
        ));
    }
}
