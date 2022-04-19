use vertigo::{
    VDomElement,
};
use vertigo::html;

use super::App;
use super::app_state::View;

pub fn app_render(app_state: &App) -> VDomElement {
    let view = app_state.view.get_value();

    match view.as_ref() {
        View::Index { state }=> {
            let view = state.render(app_state);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::EditContent { state } => {
            let view = state.render(app_state);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { state } => {
            let view = state.render(app_state);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::RenameItem {state } => {
            let view = state.render(app_state);

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { state } => {
            let view = state.render(app_state.clone());

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
