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
    pub driver: DomDriver,
    pub current_path: Value<Vec<String>>,
}

impl State {
    pub fn new(root: &Dependencies, driver: &DomDriver) -> Computed<State> {
        root.new_computed_from(State {
            driver: driver.clone(),
            current_path: root.new_value(vec!("aaa".into(), "bbb".into())),
        })
    }

    //TODO - do celow testowych

    pub fn push_path(&self) {

        //TODO - dorobić do Value funkcję change ...

        //self.current_path.change
        let mut current = (&*self.current_path.get_value()).clone();
        
        current.push("cokolwiek".into());

        self.current_path.set_value(current);
    }
}


/*
dodać zakładki


    zakładka1: notatki
    zakładka2: parsowanie urla ...

    zakładka - zarządzanie gitem
        menu pozwalające usuwać niepotrzebne gałęzie gita ...
    
        pozwoli np. ta funkcja na uruchomienie polecenia rebejsującego
*/
