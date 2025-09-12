use emmylua_parser::{LuaVersionCondition, VisibilityKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuaCommonProperty {
    pub visibility: VisibilityKind,
    pub description: Option<Box<String>>,
    pub source: Option<Box<String>>,
    pub deprecated: Option<Box<LuaDeprecated>>,
    pub version_conds: Option<Box<Vec<LuaVersionCondition>>>,
    pub tag_content: Option<Box<LuaTagContent>>,
    pub export: Option<LuaExport>,
}

impl LuaCommonProperty {
    pub fn new() -> Self {
        Self {
            visibility: VisibilityKind::Public,
            description: None,
            source: None,
            deprecated: None,
            version_conds: None,
            tag_content: None,
            export: None,
        }
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_deref()
    }

    pub fn version_conds(&self) -> Option<&Vec<LuaVersionCondition>> {
        self.version_conds.as_deref()
    }

    pub fn export(&self) -> Option<&LuaExport> {
        self.export.as_ref()
    }

    pub fn tag_content(&self) -> Option<&LuaTagContent> {
        self.tag_content.as_deref()
    }

    pub fn deprecated(&self) -> Option<&LuaDeprecated> {
        self.deprecated.as_deref()
    }

    pub fn source(&self) -> Option<&String> {
        self.source.as_deref()
    }

    pub fn add_extra_description(&mut self, description: String) {
        self.description = Some(Box::new(description));
    }

    pub fn add_extra_source(&mut self, source: String) {
        self.source = Some(Box::new(source));
    }

    pub fn add_extra_deprecated(&mut self, message: Option<String>) {
        self.deprecated = match message {
            Some(msg) => Some(Box::new(LuaDeprecated::DeprecatedWithMessage(msg))),
            None => Some(Box::new(LuaDeprecated::Deprecated)),
        };
    }

    pub fn add_extra_version_cond(&mut self, conds: Vec<LuaVersionCondition>) {
        self.version_conds = Some(Box::new(conds));
    }

    pub fn add_extra_tag(&mut self, tag: String, content: String) {
        self.tag_content
            .get_or_insert_with(|| Box::new(LuaTagContent::new()))
            .add_tag(tag, content);
    }

    pub fn add_extra_export(&mut self, export: LuaExport) {
        self.export = Some(export);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaDeprecated {
    Deprecated,
    DeprecatedWithMessage(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuaExportScope {
    Global,
    Namespace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuaTagContent {
    pub tags: Vec<(String, String)>,
}

impl LuaTagContent {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    pub fn add_tag(&mut self, tag: String, content: String) {
        self.tags.push((tag, content));
    }

    pub fn get_all_tags(&self) -> &[(String, String)] {
        &self.tags
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuaExport {
    pub scope: LuaExportScope,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct LuaPropertyId {
    id: u32,
}

impl LuaPropertyId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
