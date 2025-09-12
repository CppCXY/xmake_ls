use internment::ArcIntern;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalId(pub ArcIntern<SmolStr>);

impl GlobalId {
    pub fn new(name: &str) -> Self {
        Self(ArcIntern::new(SmolStr::new(name)))
    }

    pub fn get_name(&self) -> &str {
        self.0.as_ref()
    }

    pub fn get_prev_id(&self) -> Option<GlobalId> {
        let name = self.get_name();
        if let Some(pos) = name.rfind('.') {
            let new_name = &name[..pos];
            return Some(GlobalId::new(new_name));
        }

        None
    }
}
