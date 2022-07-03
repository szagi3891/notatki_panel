use vertigo::{
    Css,
    VDomComponent, dom,
};

use vertigo::{html, css, bind};

use crate::app::App;
use crate::components::render_path;

use super::app_render_list::render_list;
use super::app_render_content::render_content;
use super::app_render_menu::MenuComponent;

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        padding: 0;
        box-sizing: border-box;

        height: 100vh;
    ")
}

fn css_content() -> Css {
    css!("
        flex-grow: 1;
        display: flex;
        border-bottom: 1px solid black;
        overflow: hidden;
    ")
}

fn css_content_list() -> Css {
    css!("
        width: 300px;
        flex-grow: 0;
        flex-shrink: 0;
        border-right: 1px solid black;

        display: flex;
    ")
}

fn css_content_content() -> Css {
    css!("
        width: 100%;
        flex-grow: 1;
        padding: 5px;
        overflow-y: scroll;

        display: flex;
        background-color: #e8e8e8;
    ")
}

pub fn app_index_render(app: &App) -> VDomComponent {
    let view_alert = VDomComponent::dom(dom! {                     //TODO - usunąć nadmiarowego diva
        <div>
            {app.alert.render()}
        </div>
    });

    let view_menu = VDomComponent::dom(MenuComponent::component(app));

    let on_click_path = bind(&app.data).call_param(|context, data, node_id: Vec<String>| {
        data.tab.set_path(context, node_id.clone());
    });
    
    let view_header = VDomComponent::dom(dom! {                     //TODO - usunąć nadmiarowego diva
        <div>
            {render_path(&app.data.tab.router.path, on_click_path)}
        </div>
    });

    let view_list = render_list(app);
    let view_content = render_content(app);

    let app = app.clone();

    VDomComponent::from_fn(move |_| {
        let hook_keydown = bind(&app).call_param(
            |context, state, event: vertigo::KeyDownEvent| {
                state.keydown(context, event.code)
            }
        );

        html! {
            <div css={css_wrapper()} hook_key_down={hook_keydown}>
                { view_menu.clone() }
                { view_header.clone() }
                <div css={css_content()}>
                    <div css={css_content_list()}>
                        { view_list.clone() }
                    </div>
                    <div css={css_content_content()}>
                        { view_content.clone() }
                    </div>
                </div>
                { view_alert.clone() }
            </div>
        }
    })
}
