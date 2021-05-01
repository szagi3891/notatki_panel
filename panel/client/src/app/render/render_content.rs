use vertigo::{VDomElement, computed::Computed};
use vertigo_html::html;

use crate::app::state::State;

pub fn render_content(state: &Computed<State>) -> VDomElement {

    let state = state.get_value();

    let current_content = state.current_content.get_value();

    let content = current_content.to_string();

    if let Some(content) = content {
        return html!("
            <div> { content } </div>
        ");
    }

    return html!("
        <div></div>
    ");
}

