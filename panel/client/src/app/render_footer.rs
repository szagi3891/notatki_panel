use vertigo::{
    VDomElement,
    Css,
    computed::{
        Computed,
    }
};

use vertigo_html::{html_component};
use super::state::State;

fn css_footer() -> Css {
    Css::one("
        flex-shrink: 0;
        line-height: 25px;
        padding: 0 5px;
    ")
}

pub fn render_footer(state: &Computed<State>) -> VDomElement {
    html_component! {
        <div css={css_footer()}>
            Lista plików które zostały zmodyfikowane ale nie zapisane
        </div>
    }
}
