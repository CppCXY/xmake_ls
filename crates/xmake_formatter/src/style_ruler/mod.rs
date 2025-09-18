mod basic_space;

use crate::format::XmakeFormatter;

#[allow(unused)]
pub fn apply_styles(formatter: &mut XmakeFormatter) {
    apply_style::<basic_space::BasicSpaceRuler>(formatter);
}

pub trait StyleRuler {
    /// Apply the style rules to the formatter
    fn apply_style(formatter: &mut XmakeFormatter);
}

pub fn apply_style<T: StyleRuler>(formatter: &mut XmakeFormatter) {
    T::apply_style(formatter)
}
