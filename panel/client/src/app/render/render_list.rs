use vertigo::{
    VDomElement,
    computed::{
        Computed,
    }
};
use vertigo_html::html;

use crate::app::state::State;

fn render_list_files(state: &Computed<State>) -> VDomElement {
    
    html! {"
        <div>
            lista ...
        </div>
    "}

    // let state = state.get_value();
    
    // let list = state.list.get_value();

    // match &*list {
    //     Resource::Loading => {
    //         html! {"
    //             <div>
    //                 Loading ...
    //             </div>
    //         "}
    //     },
    //     Resource::Ready(data) => {
    //         let ids = data
    //             .iter()
    //             .map(|item| format!("{}", item))
    //             .collect::<Vec<String>>()
    //             .join(",");

    //         html! {"
    //             <div>
    //                 ready ==== TODO {ids}
    //             </div>
    //         "}
    //     },
    //     Resource::Failed(err) => {
    //         todo!();
    //     }
    // }
}

pub fn render_list(state: &Computed<State>) -> VDomElement {
    let on_create = {
        //let state = state.clone();
        move || {
            //state.get_value().create_dir("Jakiś".into());

            log::info!("klik w utworz katalog ...");
        }
    };

    //let state_value = state.get_value();

    html! {"
        <div>
            <div onClick={on_create}>utwórz katalog</div>
            <div>lista plikow</div>
            <component {render_list_files} data={state.clone()} />
        </div>
    "}
}
