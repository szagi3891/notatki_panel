use vertigo::{
    Value,
    Driver, VDomComponent, html, css, VDomElement, Css,
};

#[derive(Clone)]
pub struct OpenLinks {
    pub tabs_url: Value<Vec<String>>,
    pub tabs_active: Value<Option<String>>,
}

impl OpenLinks {
    pub fn new(driver: &Driver) -> OpenLinks {
        let tabs_url = driver.new_value(Vec::new());
        let tabs_active = driver.new_value(None);

        OpenLinks {
            tabs_url,
            tabs_active
        }
    }

    pub fn tabs_has(&self, url: &String) -> bool {
        let tabs_url = self.tabs_url.get_value();
        tabs_url.contains(url)
    }

    pub fn tabs_add(&self, url: String) {
        log::info!("add ... {}", &url);
        let tabs_url = self.tabs_url.get_value();

        if tabs_url.contains(&url) {
            log::error!("is contain {}", url);
            return;
        }

        let mut tabs_url = tabs_url.as_ref().clone();
        tabs_url.push(url);
        self.tabs_url.set_value(tabs_url);
    }

    pub fn tabs_remove(&self, url: String) {
        let tabs_url = self.tabs_url.get_value();

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }
        
        let tabs_url = tabs_url.as_ref().clone();
        let mut new_tabs = Vec::<String>::with_capacity(tabs_url.len());

        for tab_url in tabs_url.into_iter() {
            if tab_url != url {
                new_tabs.push(tab_url);
            }
        }

        self.tabs_url.set_value(new_tabs);

        let tabs_active = self.tabs_active.get_value();
        if *tabs_active == Some(url) {
            self.tabs_default();
        }
    }

    pub fn tabs_set(&self, url: String) {
        let tabs_url = self.tabs_url.get_value();

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }

        self.tabs_active.set_value(Some(url));
    }

    pub fn tabs_default(&self) {
        self.tabs_active.set_value(None);
    }

    pub fn render(self, default_view: VDomComponent) -> VDomComponent {
        open_links_render(self, default_view)
    }
}



fn css_iframe_bg() -> Css {
    css!("
        position: fixed;
        left: 0;
        top: 0;
        right: 0;
        bottom: 0;

        display: flex;
    ")
}

fn css_left() -> Css {
    css!("
        position: relative;
        overflow: hidden;
        flex-grow:1;
    ")
}

fn css_iframe(active: bool) -> Css {
    let style = css!("
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        overflow-y: scroll;

        width: 100%;
        height: 100%;
        padding: 0;
        margin: 0;
        border: 0;
    ");

    if active {
        style
    } else {
        style.push_str("visibility: hidden;")
    }
}

fn css_right() -> Css {
    css!("
        width: 200px;
        flex-shrink: 0;
        border-left: 1px solid black;
    ")
}

fn css_button(active: bool) -> Css {
    let css = css!("
        line-height: 30px;
        padding: 0 5px;
        cursor: pointer;
        word-break: break-word;
    ");

    if active {
        css.push_str("
            background: red;
            color: white;
        ")
    } else {
        css.push_str("
            background: #e0e0e0;
            color: black;
        ")
    }
}

fn button(
    label: impl Into<String>,
    on_click: impl Fn() + 'static,
    on_close: Option<impl Fn() + 'static>,
    active: bool
) -> VDomElement {
    let label: String = label.into();

    let close = match on_close {
        Some(on_close) => html! {
            <div on_click={on_close}>
                "x"
            </div>
        },
        None => html! { <div/> }
    };

    html!{
        <div on_click={on_click} css={css_button(active)}>
            { label }
            { close }
        </div>
    }
}


fn open_links_render(open_links: OpenLinks, default_view: VDomComponent) -> VDomComponent {

    VDomComponent::new(open_links, move |open_links: &OpenLinks| {
        let active = open_links.tabs_active.get_value();
        let tabs = open_links.tabs_url.get_value();

        let style_css = html! {
            <style>
                "
                html, body {
                    width: 100%;
                    height: 100%;
                    margin: 0;
                    padding: 0;
                    border: 0;
                }
                "
            </style>
        };

        if tabs.len() > 0 {
            let mut tabs_iframe = Vec::new();
            let mut tabs_menu = Vec::new();

        
            let is_select_default = active.is_none();

            tabs_menu.push({
                let open_links = open_links.clone();
                let on_click = move || {
                    open_links.tabs_default();
                };

                button("default", on_click, None::<fn()>, is_select_default)
            });

            if is_select_default {
                tabs_iframe.push(html! {
                    <div css={css_iframe(true)}>
                        { default_view.clone() }
                    </div>
                });
            }

            for tab_item in tabs.iter() {
                let tab_item = tab_item.clone();

                let is_select = match active.as_ref() {
                    Some(active) => *active == *tab_item,
                    None => false,
                };

                tabs_iframe.push(html! {
                    <iframe src={tab_item.clone()} css={css_iframe(is_select)} />
                });

                let on_click = {
                    let open_links = open_links.clone();
                    let tab_item = tab_item.clone();
        
                    move || {
                        open_links.tabs_set(tab_item.clone());
                    }
                };

                let on_close = {
                    let open_links = open_links.clone();
                    let tab_item = tab_item.clone();
                    move || {
                        open_links.tabs_remove(tab_item.clone());
                    }
                };
        
                tabs_menu.push(button(tab_item, on_click, Some(on_close), is_select));
            }

            return html! {
                <div css={css_iframe_bg()}>
                    { style_css }
                    <div css={css_left()}>
                        { ..tabs_iframe }
                    </div>
                    <div css={css_right()}>
                        { ..tabs_menu }
                    </div>
                </div>
            };
        }

        html! {
            <div>
                { style_css }
                { default_view.clone() }
            </div>
        }
    })
}