use vertigo::{
    css, Css,
    Resource,
    bind, dom, Computed, DomNode, bind_rc
};
use crate::components::list_item::ItemDefault;
use crate::data::{Data, ListItem};

fn item_default_render(data: &Data, item: &ListItem, mouse_over_enable: bool) -> DomNode {
    let tab = &data.tab;

    if mouse_over_enable {

        let mouse_over_enter = bind_rc!(item, tab, || {
            tab.hover_on(&item.name());
        });

        let mouse_over_leave = bind_rc!(item, tab, || {
            tab.hover_off(item.name().as_str());
        });

        dom! {
            <ItemDefault
                data={data.clone()}
                item={item.clone()}
                on_click={item.redirect_view.clone()}
                mouse_over_enter={Some(mouse_over_enter)}
                mouse_over_leave={Some(mouse_over_leave)}
            />
        }
    } else {
        dom! {
            <ItemDefault
                data={data.clone()}
                item={item.clone()}
                on_click={item.redirect_view.clone()}
                mouse_over_enter={None}
                mouse_over_leave={None}
            />
        }
    }
}

fn css_image() -> Css {
    css!("
        width: 100px;
        margin: 5px;
        border:1px solid black;
        padding: 1px;
        cursor: pointer;
    ")
}

fn item_image_render(data: &Data, item: &ListItem, ext: &String) -> DomNode {
    let data = data.clone();
    let item = item.clone();

    let url = Computed::from(bind!(item, ext, |context| {
        let id = item.id.get(context);

        match id {
            Resource::Ready(id) => Some(format!("/image/{id}/{ext}")),
            _ => None
        }
    }));

    let tab = &data.tab;
    let on_click = tab.build_redirect_to_item(item);

    url.render_value(bind!(on_click, |url| {
        match url {
            Some(url) => {
                dom!{
                    <img
                        css={css_image()}
                        src={url}
                        on_click={on_click.clone()}
                    />
                }
            },
            None => {
                dom! { <span /> }
            }
        }
    }))

}

fn list_items_from_vec(data: &Data, list_sorted: Computed<Vec<(Option<String>, ListItem)>>, mouse_over_enable: bool) -> DomNode {
    list_sorted.render_list(
        |(_, item)| item.to_string_path(),
        {
            let data = data.clone();
            move |(picture, item)| {

                match picture {
                    Some(ext) => {
                        item_image_render(&data, item, ext)
                    },
                    None => {
                        item_default_render(&data, item, mouse_over_enable)
                    }
                }
            }
        }
    )
}

pub fn list_items_from_dir(data: &Data, select_dir: &Computed<ListItem>, mouse_over_enable: bool) -> DomNode {
    let list = Computed::from({
        let select_dir = select_dir.clone();
        move |context| {
            let select_dir = select_dir.get(context);
            let current = select_dir.list.get(context);

            match current {
                Resource::Ready(list) => list,
                _ => {
                    return Vec::new();
                }
            }
        }
    });

    let list_sorted = Computed::from({
        move |context| {
            let list = list.get(context);

            if mouse_over_enable {
                return list.into_iter().map(|item| (None, item)).collect::<Vec<_>>();
            }

            let mut out: Vec<(Option<String>, ListItem)> = Vec::new();
            let mut picture: Vec<(Option<String>, ListItem)> = Vec::new();
        
            for item in list.into_iter() {
                if let Some(ext) = item.get_picture_ext() {
                    picture.push((Some(ext), item));
                } else {
                    out.push((None, item));
                }
            }
        
            out.extend(picture.into_iter());

            out
        }
    });

    list_items_from_vec(data, list_sorted, mouse_over_enable)
}

