use vertigo::{
    VDomElement,
    computed::{
        Computed,
    }
};
use vertigo_html::html;

use crate::app::state::State;

fn render_list_files(state: &Computed<State>) -> VDomElement {
    
    let mut out: Vec<VDomElement> = Vec::new();

    let state = state.get_value();
    let list = state.list.get_value();
    
    log::info!("lista renderowana {:?}", &list);

    for item in (*list).iter() {
        let label = if item.dir {
            "dir"
        } else {
            "file"
        };

        let on_click = {
            let state = state.clone();
            let item = item.clone();

            move || {
                log::info!("klik w item {}", &item.name);
                state.push_path(item.name.clone());
            }
        };

        out.push(html!{"
            <div onClick={on_click}>{&item.name} ({label})</div>
        "});
    }

    html! {"
        <div>
            { ..out }
        </div>
    "}

    //TODO - dodać loader

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

    html! {"
        <div>
            <div onClick={on_create}>utwórz katalog</div>
            <component {render_list_files} data={state.clone()} />
        </div>
    "}
}
