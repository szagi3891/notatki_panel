use vertigo::{
    DomDriver,
    VDomElement,
    computed::{
        Value,
        Computed,
        Dependencies,
    },
};

use crate::state_data::CurrentContent;
use crate::state_data::StateData;

use self::index::ListItem;

mod index;
mod edit_content;
mod new_content;

#[derive(PartialEq)]
pub enum View {
    Index,
    EditContent {
        state: Computed<edit_content::State>,
    },
    NewContent {
        state: Computed<new_content::State>,
    }
    //TODO - zmiana nazwy
}

#[derive(Clone, PartialEq)]
pub struct CallbackBuilder {
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

    pub fn redirect_to_content(&self, path: Vec<String>) {
        let content = self.state_data.get_content_from_path(&path);

        match content {
            CurrentContent::File { file_hash, content, ..} => {

                let root = self.root.clone();

                let state = edit_content::State::new(
                    path,
                    file_hash,
                    content.as_ref().clone(),
                    &root,
                    &self.driver,
                    self.clone(),
                );

                self.current_view.set_value(View::EditContent {
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
    }

    pub fn redirect_to_index(&self) {
        self.current_view.set_value(View::Index);
    }

    pub fn redirect_to_index_with_path(&self, new_path: Vec<String>, new_item: Option<String>) {
        self.current_view.set_value(View::Index);
        self.state_data.current_path_dir.set_value(new_path);
        self.state_data.current_path_item.set_value(new_item);
        self.state_data.state_root.refresh();
    }

    pub fn redirect_to_index_with_root_refresh(&self) {
        self.state_data.state_root.refresh();
        self.current_view.set_value(View::Index);
    }

    pub fn redirect_to_new_content(&self, parent: Vec<String>, list: Computed<Vec<ListItem>>) {
        let state = new_content::State::new(
            &self.root,
            parent,
            &self.driver,
            list,
            self.clone(),
        );

        let state = self.root.new_computed_from(state);

        self.current_view.set_value(View::NewContent { state });
    }
}

// use vertigo::utils::BoxRefCell;
// use std::rc::Rc;
// use std::cell::{RefCell, Cell, Ref};
// use std::collections::HashMap;

// struct StateBuilder {

// }

// struct State2<T> {
//     id: u64,
//     data: HashMap<u64, T>,
// }

// impl<T> std::ops::Deref for State2<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         let item = self.data.get(&self.id);

//         item.unwrap()
//     }
// }

// struct State3<T> {
//     value: Rc<RefCell<Option<Rc<T>>>>,
// }

// impl<T> State3<T> {
//     pub fn new() -> State3<T> {
//         State3 {
//             value: Rc::new(RefCell::new(None))
//         }
//     }

//     pub fn get(&self) -> Rc<T> {
//         let inner = self.value.as_ref().borrow();
//         let copy = inner.as_ref().unwrap().clone();
//         copy
//     }
// }

// impl<T> std::ops::Deref for State3<T> {
//     type Target = Ref<T>;

//     fn deref(&self) -> &Rc<T> {
//         &self.value
//     }
// }

// struct StateBox<T> {
//     inner: Option<T>,
// }

// impl<T> StateBox<T> {
//     pub fn new<
//         F: Fn(&StateBox<T>) -> T
//     >(callback: F) -> StateBox<T> {
//         let state = StateBox {
//             inner: None,
//         };

//         let new_inner = callback(&state);

//         let inner = state.inner.get_mut();
//         *inner = Some(new_inner);
//         // state.inner.change(new_inner, |state, data| {
//         //     *state = Some(Rc::new(data));
//         // });

//         state
//     }
// }

// impl<T> std::ops::Deref for StateBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         let f = self.inner.re

//         f.as_ref()
//     }
// }

/*
invalid `self` parameter type: StateBox<AAB>
type of `self` must be `Self` or a type that dereferences to it
consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)rustcE0307
*/
// struct AAB {

// }

// impl AAB {
//     fn test(self: StateBox<AAB>) -> String {
//         String::ftom("das")
//     }
// }

//konstruktor będzie przyjmowal referencje w konstruktorze callbackowym do siebie samego



#[derive(PartialEq)]
pub struct State {
    state_view_index: Computed<index::State>,
    current_view: Computed<View>,
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
                callback.clone(),
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
        View::NewContent { state } => {
            new_content::render(state)
        }
    }
}
