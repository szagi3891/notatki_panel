use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Computed,
        Dependencies,
    },
    utils::{EqBox}
};

use crate::state_data::CurrentContent;
// use super::{StateViewEditContent, StateViewIndex, StateViewNewContent};
use crate::state_data::StateData;

mod index;
mod edit_content;

#[derive(PartialEq)]
pub enum View {
    Index,
    EditContent {
        state: Computed<edit_content::State>,
    },
    // NewContent {
    //     state: Computed<StateViewNewContent>,
    // }
    //TODO - zmiana nazwy
}



#[derive(PartialEq)]
pub struct State {
    pub state_view_index: Computed<index::State>,
    pub current_view: Computed<View>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        let state_data = StateData::new(root, driver);

        let current_view = root.new_value(View::Index);
        let current_view_computed = current_view.to_computed();

        let state_view_index = {
            let current_view = current_view.clone();

            let callback_redirect_to_content: EqBox<Box<dyn Fn(Vec<String>) -> ()>> = {
                let root = root.clone();
                let driver = driver.clone();
                let state_data = state_data.clone();
                
                EqBox::new(Box::new(move |path: Vec<String>| {

                    let content = state_data.get_content_from_path(&path);

                    match content {
                        CurrentContent::File { file_hash, content, ..} => {

                            let root = root.clone();

                            let callback_redirect_to_index: EqBox<Box<dyn Fn() -> ()>> = {
                                let current_view = current_view.clone();
                                EqBox::new(Box::new(move || {
                                    current_view.set_value(View::Index);
                                }))
                            };

                            let callback_redirect_to_index_with_root_refresh: EqBox<Box<dyn Fn() -> ()>> = {
                                let current_view = current_view.clone();
                                let state_data = state_data.clone();

                                EqBox::new(Box::new(move || {
                                    state_data.state_root.refresh();
                                    current_view.set_value(View::Index);
                                }))
                            };

                            let state = edit_content::State::new(
                                path,
                                file_hash,
                                content.as_ref().clone(),
                                &root,
                                &driver,
                                callback_redirect_to_index,
                                callback_redirect_to_index_with_root_refresh
                            );

                            current_view.set_value(View::EditContent {
                                state: root.new_computed_from(state)
                            });
                        },
                        CurrentContent::Dir { .. } => {
                            log::error!("Oczekiwano pliku, znaleziono katalog");
                        },
                        CurrentContent::None => {
                            log::error!("Oczekiwano pliku, nic nie znaleziono");
                        }
                    }
                }))
            };

            index::State::new(
                root,
                state_data.clone(),
                callback_redirect_to_content
            )
        };

        // {
        //     let root = root.clone();
        //     let action = action.clone();
        //     let driver = driver.clone();
        //     let state_data = state_data.clone();

        //     subscribe.subscribe(move |message| {
        //         match message {
        //             StateAction::RedirectToIndex => {
        //                 current_view.set_value(View::Index);
        //             },
        //             StateAction::RedirectToIndexWithRootRefresh => {
        //                 state_data.state_root.refresh();
        //                 current_view.set_value(View::Index);
        //             },
        //             StateAction::RedirectToContent { path } => {
        //                 let content = state_data.get_content_from_path(&path);

        //                 match content {
        //                     CurrentContent::File { file_hash, content, ..} => {

        //                         let state = StateViewEditContent::new(
        //                             path,
        //                             file_hash,
        //                             content.as_ref().clone(),
        //                             &action,
        //                             &root,
        //                             &driver
        //                         );

        //                         current_view.set_value(View::EditContent {
        //                             state: root.new_computed_from(state)
        //                         });
        //                     },
        //                     CurrentContent::Dir { .. } => {
        //                         log::error!("Oczekiwano pliku, znaleziono katalog");
        //                     },
        //                     CurrentContent::None => {
        //                         log::error!("Oczekiwano pliku, nic nie znaleziono");
        //                     }
        //                 }
        //             },
        //             StateAction::RedirectToNewContent { parent } => {
        //                 let new_content = StateViewNewContent::new(
        //                     &root,
        //                     parent,
        //                     &action,
        //                 );
        //                 current_view.set_value(View::NewContent{
        //                     state: root.new_computed_from(new_content)
        //                 });
        //             }
        //         }
        //     });
        // }

        root.new_computed_from(State {
            state_view_index,
            current_view: current_view_computed,
        })
    }
}


pub fn render(state: &Computed<State>) -> VDomElement {

    let state_value = state.get_value();
    let view = state_value.current_view.get_value();

    match view.as_ref() {
        View::Index => {
            index::render(&state_value.state_view_index)
        },
        View::EditContent { state }=> {
            edit_content::render(state)
        },
        // View::NewContent { state } => {
        //     html! {
        //         <div>
        //             "todo - dodanie kontentu"
        //         </div>
        //     }
        // }
    }
}
