use vertigo::{
    VDomElement,
};
use vertigo::html;

use crate::app::edit_content::AppEditcontent;
use crate::app::new_dir::AppNewdir;
use crate::app::newcontent::AppNewcontent;
use crate::app::rename_item::AppRenameitem;

use super::App;
use super::app_state::View;
use super::index::AppIndex;

pub fn app_render(app_state: &App) -> VDomElement {
    let view = app_state.view.get_value();
    let open_links = app_state.data.tab.open_links.clone();

    match view.as_ref() {
        View::Index => {
            let (view, on_keydown) = AppIndex::component(&app_state);
            let view = open_links.render(view);

            html! {
                <div id="root" on_key_down={on_keydown}>
                    { view }
                </div>
            }
        },
        View::EditContent { full_path, file_hash, content } => {
            let view = AppEditcontent::component(
                app_state,
                full_path.clone(),
                file_hash.clone(),
                content.as_ref().clone(),
            );
            let view = open_links.render(view);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        },
        View::NewContent { parent, list } => {
            let view = AppNewcontent::component(
                app_state,
                parent.clone(),
                list.clone(),
            );
            let view = open_links.render(view);

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
            let view = open_links.render(view);

            html! {
                <div id="root">
                    {view}
                </div>
            }
        },
        View::Mkdir { parent, list } => {
            let view = AppNewdir::component(app_state, (*parent).to_vec(), list.clone());
            let view = open_links.render(view);

            html! {
                <div id="root">
                    { view }
                </div>
            }
        }
    }
}
