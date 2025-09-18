#[cfg(test)]
mod test {
    use crate::reformat_xmake_code;

    #[test]
    fn test_reformat_lua_code() {
        let code = r#"
            local a =  1
            local b   = 2
            local c = a+b
            local e =-  b
            local ccc = a   > 2 and b< 3
            print(c)
        "#;

        let formatted_code = reformat_xmake_code(code);
        println!("Formatted code:\n{}", formatted_code);
    }
}
