use vertigo::{
    DomDriver,
    computed::{
        Value,
        Computed,
        Dependencies
    }
};

#[derive(PartialEq)]
pub struct State {
    driver: DomDriver,
    current_path: Value<Vec<String>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        root.new_computed_from(State {
            driver: driver.clone(),
            current_path: root.new_value(Vec::new()),
        })
    }
}
