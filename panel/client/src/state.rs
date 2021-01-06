use vertigo::{
    VDomNode,
    DomDriver,
    node_attr,
    computed::{
        Computed,
        Dependencies
    }
};

#[derive(PartialEq)]
pub struct State {
    //..
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        root.new_computed_from(State {})
    }
}

pub fn render(state: &Computed<State>) -> VDomNode {
    use node_attr::{build_node, text};
    build_node("div", vec!(
        text("render ...")
    ))
}
