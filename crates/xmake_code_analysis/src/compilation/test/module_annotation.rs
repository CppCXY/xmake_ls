#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_module_annotation() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def_files(vec![(
            "a.lua",
            r#"
                local a = {
                }
                return a
                "#,
        )]);

        ws.def(
            r#"
            ---@module "a"
            aaa = {}
            "#,
        );

        let aaa_ty = ws.expr_ty("aaa");
        let expected = ws.expr_ty("require('a')");
        assert_eq!(aaa_ty, expected);
    }
}
