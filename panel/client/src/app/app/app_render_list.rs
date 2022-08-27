// use vertigo::dev::NodeRefs;
use vertigo::{
    Css, dom, Computed, DomElement,
};
use vertigo::{css};
use crate::app::App;
use crate::components::list_items_from_dir;

fn css_wrapper() -> Css {
    css!("
        flex-grow: 1;
        overflow-y: scroll;
    ")
}

//  444 .get_bounding_client_rect_y () .height
/*
    444
    .get_bounding_client_rect_y()
    .height

    window
    .localStorage
    .getItem("dsdadsa")

    window
    .localStorage
    .setItem("dsadsa", "dsadsadas")
    
    potencjalnie kolejne wywołania łańcuchowe mona by doczepić


    
*/

//TODO - trzeba przywrócić tą funkcjonalność

//Koryguj tylko wtedy gdy element aktywny nie jest widoczny
// fn dom_apply(node_refs: &NodeRefs) {

//     if let (
//         Some(wrapper),
//         Some(active)
//     ) = (
//         node_refs.expect_one("wrapper"),
//         node_refs.expect_one("active")
//     ) {
//         let active_rect_y = active.get_bounding_client_rect_y();
//         let active_rect_height = active.get_bounding_client_rect_height() as i32;

//         let wrapper_rect_y = wrapper.get_bounding_client_rect_y();
//         let wrapper_rect_height = wrapper.get_bounding_client_rect_height() as i32;

//         //Wybrany element znajduje się w obszarze widoku, nic nie trzeba korygować
//         if active_rect_y > wrapper_rect_y && active_rect_y < wrapper_rect_y + wrapper_rect_height {
//             return;
//         }

//         if active_rect_y < wrapper_rect_y {
//             let offset = wrapper_rect_y - active_rect_y;

//             let scroll_top = wrapper.scroll_top();
//             wrapper.set_scroll_top(scroll_top - offset);
//             return;
//         }

//         let wrapper_y2 = wrapper_rect_y + wrapper_rect_height;
//         let active_y2 = active_rect_y + active_rect_height;

//         if active_y2 > wrapper_y2 {
//             let offset = active_y2 - wrapper_y2;

//             let scroll_top = wrapper.scroll_top();
//             wrapper.set_scroll_top(scroll_top + offset);
//             return;
//         }
//     }
// }

pub fn render_list(state: &App) -> DomElement {
    let dir = Computed::from({
        let state = state.clone();
        move |context| {
            state.data.tab.router.get_dir(context)
        }
    });

    //dom_ref="wrapper" dom_apply={dom_apply}

    let wrapper = dom! {
        <div css={css_wrapper()} />
    };

    //TODO - ref do wrappera będzie przekazywany do funkcji list_items_from_dir

    let list_view = list_items_from_dir(&state.data, &dir, true);

    wrapper.add_child(list_view);

    wrapper
    // dom! {
    //     <div css={css_wrapper()}>
    //         { list_view }
    //     </div>
    // }
}

//Centrowanie na środku zawsze
// let dom_apply = |node_refs: &NodeRefs| {

//     if let (Some(wrapper), Some(active)) = (node_refs.expect_one("wrapper"), node_refs.expect_one("active")) {
//         let wrapper_rect = wrapper.get_bounding_client_rect();
//         let active_rect = active.get_bounding_client_rect();
//         let scroll_top = wrapper.scroll_top();

//         let active_offset_from_wrapper = active_rect.y as i32 + scroll_top - wrapper_rect.y as i32;
//         let target_offset_from_wrapper = (wrapper_rect.height as i32 - active_rect.height as i32) / 2;

//         let offset = active_offset_from_wrapper - target_offset_from_wrapper;

//         wrapper.set_scroll_top(offset);
//     }
// };
