use vertigo::{
    Value,
    Driver,
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
}
