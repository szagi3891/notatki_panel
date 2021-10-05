use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::{collections::HashMap};
use vertigo::DomDriver;
use std::future::Future;
use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
pub enum ResourceError {
    Loading,
    Error(String),
}

pub type Resource<T> = Result<T, ResourceError>;

#[derive(Clone, PartialEq)]
pub struct Request {
    driver: DomDriver,
}

impl Request {
    pub fn new(driver: &DomDriver) -> Request {
        Request {
            driver: driver.clone(),
        }
    }

    pub fn fetch<U: Into<String>>(&self, url: U) -> RequestBuilder {
        let mut headers = HashMap::<String, String>::new();
        let key = "Content-Type".into();
        let value = "application/json".into();
        headers.insert(key, value);

        RequestBuilder {
            driver: self.driver.clone(),
            url: url.into(),
            headers: Some(headers),
            body: Body::None,
        }
    }

    pub fn spawn_local<F>(&self, future: F)
        where F: Future<Output = ()> + 'static {
            self.driver.spawn_local(future);
    }
}

enum Body {
    None,
    Some(String),
    Error(String),
}

enum Method {
    Get,
    Post,
}

pub struct RequestBuilder {
    driver: DomDriver,
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Body,
}

impl RequestBuilder {
    pub fn body<B: Serialize>(self, body: B) -> RequestBuilder {
        let body_str = serde_json::to_string(&body);

        let RequestBuilder { driver , url, headers, .. } = self;

        match body_str {
            Ok(body) => {
                RequestBuilder {
                    driver,
                    url,
                    headers,
                    body: Body::Some(body),
                }
            },
            Err(err) => {
                RequestBuilder {
                    driver,
                    url,
                    headers,
                    body: Body::Error(format!("{}", err)),
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn headers(self, headers: HashMap<String, String>) -> RequestBuilder {
        let RequestBuilder { driver, url, body, .. } = self;

        RequestBuilder {
            driver,
            url,
            headers: Some(headers),
            body,
        }
    }

    async fn call<T: PartialEq + DeserializeOwned>(self, method: Method) -> Result<T, String> {
        let RequestBuilder { driver, url, body, headers } = self;

        let builder = driver.fetch(url);

        let builder = match body {
            Body::None => {
                builder
            },
            Body::Some(body) => {
                builder.set_body(body)
            },
            Body::Error(err) => {
                return Err(err);
            }
        };

        let builder = match headers {
            Some(headers) => builder.set_headres(headers),
            None => builder,
        };

        let result = match method {
            Method::Get => builder.get().await,
            Method::Post => builder.post().await,
        };

        let result = match result {
            Ok((_, result)) => result,
            Err(err) => {
                return Err(err);
            }
        };

        match serde_json::from_str::<T>(result.as_str()) {
            Ok(result) => {
                Ok(result)
            },
            Err(err) => {
                Err(format!("{}", err))
            }
        }
    }

    async fn call_wrapper<T: PartialEq + Debug + DeserializeOwned>(self, method: Method) -> Resource<T> {
        let result = self.call::<T>(method).await;
        match result {
            Ok(value) => {
                Ok(value)
            },
            Err(err) => {
                log::error!("Response err - {:?}", err);
                Err(ResourceError::Error(err))
            },
        }
    }

    pub async fn get<T: PartialEq + Debug + DeserializeOwned>(self) -> Resource<T> {
        self.call_wrapper::<T>(Method::Get).await
    }

    pub async fn post<T: PartialEq + Debug + DeserializeOwned>(self) -> Resource<T> {
        self.call_wrapper::<T>(Method::Post).await
    }
}
