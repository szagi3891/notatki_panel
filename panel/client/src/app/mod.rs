use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
    Callback,
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

#[derive(Clone)]
struct CallbackBuilder {
    root: Dependencies,
    driver: DomDriver,
    state_data: StateData,
    current_view: Value<View>,
}

impl CallbackBuilder {
    pub fn new(
        root: &Dependencies,
        driver: &DomDriver,
        state_data: &StateData,
        current_view: &Value<View>
    ) -> CallbackBuilder {
        CallbackBuilder {
            root: root.clone(),
            driver: driver.clone(),
            state_data: state_data.clone(),
            current_view: current_view.clone(),
        }
    }

    pub fn redirect_to_content(&self) -> Callback<Vec<std::string::String>> {
        let callback = self.clone();
        
        Callback::new(move |path: Vec<String>| {

            let content = callback.state_data.get_content_from_path(&path);

            match content {
                CurrentContent::File { file_hash, content, ..} => {

                    let root = callback.root.clone();

                    let callback_redirect_to_index = callback.redirect_to_index();
                    let callback_redirect_to_index_with_root_refresh = callback.redirect_to_index_with_root_refresh();

                    let state = edit_content::State::new(
                        path,
                        file_hash,
                        content.as_ref().clone(),
                        &root,
                        &callback.driver,
                        callback_redirect_to_index,
                        callback_redirect_to_index_with_root_refresh
                    );

                    callback.current_view.set_value(View::EditContent {
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
        })
    }

    pub fn redirect_to_index(&self) -> Callback<()> {
        let current_view = self.current_view.clone();
        Callback::new(move |_| {
            current_view.set_value(View::Index);
        })
    }

    pub fn redirect_to_index_with_root_refresh(&self) -> Callback<()> {
        let current_view = self.current_view.clone();
        let state_data = self.state_data.clone();

        Callback::new(move |_| {
            state_data.state_root.refresh();
            current_view.set_value(View::Index);
        })
    }
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

        let callback = CallbackBuilder::new(root, driver, &state_data, &current_view);

        root.new_computed_from(State {
            state_view_index: index::State::new(
                root,
                state_data.clone(),
                callback.redirect_to_content(),
            ),
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
        View::EditContent { state } => {
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
