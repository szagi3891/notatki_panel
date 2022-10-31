use vertigo::{
    Value, css, Css, bind, Context, DomElement, Computed, dom, DomNodeFragment, ListRendered, bind2,
};

#[derive(Clone, PartialEq)]
pub struct OpenLinks {
    pub tabs_url: Value<Vec<String>>,
    pub tabs_active: Value<Option<String>>,
}

impl OpenLinks {
    pub fn new() -> OpenLinks {
        let tabs_url = Value::new(Vec::new());
        let tabs_active = Value::new(None);

        OpenLinks {
            tabs_url,
            tabs_active
        }
    }

    pub fn tabs_has(&self, context: &Context, url: &String) -> bool {
        let tabs_url = self.tabs_url.get(context);
        tabs_url.contains(url)
    }

    pub fn tabs_add(&self, context: &Context, url: String) {
        log::info!("add ... {}", &url);
        let tabs_url = self.tabs_url.get(context);

        if tabs_url.contains(&url) {
            log::error!("is contain {}", url);
            return;
        }

        let mut tabs_url = tabs_url;
        tabs_url.push(url);
        self.tabs_url.set(tabs_url);
    }

    pub fn tabs_toogle(&self, context: &Context, url: String) {
        let has_open = self.tabs_has(context, &url);

        if has_open {
            self.tabs_remove(context, url);
        } else {
            self.tabs_add(context, url);
        }
    }

    pub fn tabs_remove(&self, context: &Context, url: String) {
        let tabs_url = self.tabs_url.get(context);

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }
        
        let mut new_tabs = Vec::<String>::with_capacity(tabs_url.len());

        for tab_url in tabs_url.into_iter() {
            if tab_url != url {
                new_tabs.push(tab_url);
            }
        }

        self.tabs_url.set(new_tabs);

        let tabs_active = self.tabs_active.get(context);
        if tabs_active == Some(url) {
            self.tabs_default();
        }
    }

    pub fn tabs_set(&self, context: &Context, url: String) {
        let tabs_url = self.tabs_url.get(context);

        if !tabs_url.contains(&url) {
            log::error!("not contain {}", url);
            return;
        }

        self.tabs_active.set(Some(url));
    }

    pub fn tabs_default(&self) {
        self.tabs_active.set(None);
    }

    pub fn render(&self, default_view: impl Into<DomNodeFragment>) -> DomElement {
        open_links_render(self, default_view.into())
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
        height: 100vh;
    ")
}

fn css_left() -> Css {
    css!("
        position: relative;
        overflow: hidden;
        flex-grow:1;
        height: 100vh;
    ")
}

fn css_iframe(active: Computed<bool>) -> Computed<Css>  {
    active.map(|active| {
        let style = css!("
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
            style.push_str("visibility: hidden; width: 0;")
        }
    })
}

fn css_right(show_column: bool) -> Css {
    if show_column {
        css!("
            width: 200px;
            flex-shrink: 0;
            border-left: 1px solid black;
        ")
    } else {
        css! {"
            display: none;
        "}
    }
}

fn css_button(active: &Computed<bool>) -> Computed<Css> {
    active.clone().map(move |active| {
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
    })
}

fn button(
    label: impl Into<String>,
    on_click: impl Fn() + 'static,
    on_close: Option<impl Fn() + 'static>,
    active: &Computed<bool>
) -> DomElement {
    let label: String = label.into();

    let close = match on_close {
        Some(on_close) => dom! {
            <div on_click={on_close}>
                "x"
            </div>
        },
        None => dom! { <div/> }
    };

    dom!{
        <div on_click={on_click} css={css_button(active)}>
            { label }
            { close }
        </div>
    }
}

fn render_main_content(active_default: &Computed<bool>, default_view: impl Into<DomNodeFragment>) -> DomElement {
    let css_wrapper = active_default.clone().map(|active| {
        match active {
            true => css! {"
                display: block;
            "},
            false => css! {"
                display: none;
            "}
        }
    });

    dom! {
        <div css={css_wrapper}>
            { default_view.into() }
        </div>
    }
}

fn render_tab_list(open_links: &OpenLinks, tabs: &Computed<Vec<String>>) -> ListRendered<String> {
    tabs.render_list(|item| item.clone(), {
        let open_links = open_links.clone();
        move |url| {
            let is_select = Computed::from({
                let url = url.clone();
                let open_links = open_links.clone();
                move |context| {
                    let active = open_links.tabs_active.get(context);
                    if let Some(active) = active {
                        url == active
                    } else {
                        false
                    }
                }
            });

            dom! {
                <iframe src={url.clone()} css={css_iframe(is_select)} />
            }
        }
    })
}

fn render_tab_buttons(open_links: &OpenLinks, tabs: &Computed<Vec<String>>) -> ListRendered<String> {
    tabs.render_list(|item| item.clone(), {
        let open_links = open_links.clone();
        move |url| {
            let on_click = bind2(&open_links, url).call(|context, open_links, tab_item| {
                open_links.tabs_set(context, (*tab_item).clone());
            });

            let on_close = bind2(&open_links, url).call(|context, open_links, tab_item| {
                open_links.tabs_remove(context, (*tab_item).clone());
            });

            let is_select = Computed::from({
                let open_links = open_links.clone();
                let url = url.clone();
                move |context| {
                    let active = open_links.tabs_active.get(context);
                    if let Some(active) = active {
                        url == active
                    } else {
                        false
                    }
                }
            });

            let url = url.clone();
            button(
                url,
                on_click,
                Some(on_close),
                &is_select
            )
        }
    })
}

fn open_links_render(open_links: &OpenLinks, default_view: DomNodeFragment) -> DomElement {
    let active_default = Computed::from({
        let open_links = open_links.clone();
        move |context| {
            let active = open_links.tabs_active.get(context);
            active.is_none()
        }
    });

    let tabs = Computed::from({
        let open_links = open_links.clone();
        move |context| {
            open_links.tabs_url.get(context)
        }
    });

    let default_tab_button = {
        let on_click = bind(open_links).call(|_, open_links| {
            open_links.tabs_default();
        });

        button("default", on_click, None::<fn()>, &active_default)
    };

    let css_right_column = tabs.map(|tabs| {
        css_right(tabs.len() > 0)
    });

    dom! {
        <div css={css_iframe_bg()}>
            <div css={css_left()}>
                { render_main_content(&active_default, default_view) }
                { render_tab_list(open_links, &tabs) }
            </div>
            <div css={css_right_column}>
                { default_tab_button }
                { render_tab_buttons(open_links, &tabs)}
            </div>
        </div>
    }
}

/*
*/