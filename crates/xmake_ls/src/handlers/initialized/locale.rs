use xmake_code_analysis::get_locale_code;
use log::info;
use lsp_types::InitializeParams;

pub fn set_ls_locale(params: &InitializeParams) -> Option<()> {
    let mut locale: String = params.locale.clone()?;
    locale = get_locale_code(&locale);
    info!("set locale: {}", locale);
    emmylua_parser::set_locale(&locale);
    xmake_code_analysis::set_locale(&locale);
    rust_i18n::set_locale(&locale);
    Some(())
}
