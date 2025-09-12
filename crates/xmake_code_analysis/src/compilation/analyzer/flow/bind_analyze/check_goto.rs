use emmylua_parser::LuaAstToken;

use crate::{AnalyzeError, DiagnosticCode, compilation::analyzer::flow::binder::FlowBinder};

pub fn check_goto_label(binder: &mut FlowBinder) {
    let goto_stat_caches = binder.get_goto_caches();
    for goto_stat_cache in goto_stat_caches {
        let label_token = goto_stat_cache.label_token;
        let label_name = goto_stat_cache.label.as_str();
        if binder
            .get_label(goto_stat_cache.closure_id, label_name)
            .is_none()
        {
            binder.report_error(AnalyzeError::new(
                DiagnosticCode::SyntaxError,
                &t!(
                    "goto label '%{label_name}' not found",
                    label_name = label_name
                ),
                label_token.get_range(),
            ));
        }
    }
}
