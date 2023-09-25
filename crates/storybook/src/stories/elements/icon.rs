use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::IntoEnumIterator;
use ui::{icon, theme, IconAsset};

use crate::story::Story;

#[derive(Element, Default)]
pub struct IconStory {}

impl IconStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let icons = IconAsset::iter();

        Story::container()
            .child(Story::title_for::<_, ui::Icon>())
            .child(Story::label("All Icons"))
            .child(div().flex().gap_3().children(icons.map(icon)))
    }
}