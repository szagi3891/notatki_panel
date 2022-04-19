use vertigo::{
    VDomElement,
};
use vertigo::html;

use crate::app::new_dir::AppNewdir;
use crate::app::rename_item::AppRenameitem;

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
        View::RenameItem { base_path, prev_name, prev_hash, prev_content } => {
            let view = AppRenameitem::component(
                app_state,
                base_path.clone(),
                prev_name.clone(),
                prev_hash.clone(),
                prev_content.clone(),
            );

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir => {
            let view = AppNewdir::component(app_state);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
