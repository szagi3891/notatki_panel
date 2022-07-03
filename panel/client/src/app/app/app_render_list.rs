// use vertigo::dev::NodeRefs;
use vertigo::{
    Css, VDomComponent, dom,
};
use vertigo::{css, html};
use crate::app::App;
use crate::components::list_items_from_dir;

fn css_wrapper() -> Css {
    css!("
        flex-grow: 1;
        overflow-y: scroll;
    ")
}

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

pub fn render_list(state: &App) -> VDomComponent {
    VDomComponent::from_ref(state, |context, state| {
        let dir = state.data.tab.router.get_dir(context);

        let out = list_items_from_dir(context, &state.data, dir.as_ref(), true);

        //dom_ref="wrapper" dom_apply={dom_apply}

        let container = dom! {
            <div css={css_wrapper()} />
        };

        for child in out.into_iter() {
            container.add_child(child);
        }

        let container = VDomComponent::dom(container);


                        //TODO - usunąć nadmiarowy wrapper
        html! {
            <div css={css_wrapper()}>
                { container }
            </div>
        }
    })
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
