pub struct Bind1<T1: Clone> {
    param1: T1,
}

pub fn bind<T1: Clone>(param1: &T1) -> Bind1<T1> {
    let param1 = param1.clone();

    Bind1 {
        param1
    }
}

impl<T1: Clone> Bind1<T1> {
    pub fn bind<T2: Clone>(self, param2: &T2) -> Bind2<T1, T2> {
        Bind2 {
            param1: self.param1,
            param2: param2.clone(),
        }
    }

    pub fn exec_ref<F: Fn(&T1) + 'static>(self, fun: F) -> impl Fn() {
        let Self { param1 } = self;

        move || {
            fun(&param1);
        }
    }

    pub fn exec<F: Fn(T1) + 'static>(self, fun: F) -> impl Fn() {
        let Self { param1 } = self;

        move || {
            let param1 = param1.clone();
            fun(param1);
        }
    }
}

pub struct Bind2<T1: Clone, T2: Clone> {
    param1: T1,
    param2: T2,
}

impl<T1: Clone, T2: Clone> Bind2<T1, T2> {
    pub fn exec_ref<F: Fn(&T1, &T2) + 'static>(self, fun: F) -> impl Fn() {
        let Self { param1, param2 } = self;

        move || {
            fun(&param1, &param2);
        }
    }

    pub fn exec<F: Fn(T1, T2) + 'static>(self, fun: F) -> impl Fn() {
        let Self { param1, param2 } = self;

        move || {
            let param1 = param1.clone();
            let param2 = param2.clone();
            fun(param1, param2);
        }
    }

    pub fn spawn<Fut: Future<Output=()> + 'static, F: Fn(T1, T2) -> Fut + 'static>(self, driver: Driver, fun: F) -> impl Fn() {
        let Self { param1, param2 } = self;

        move || {
            let param1 = param1.clone();
            let param2 = param2.clone();
            let future = fun(param1, param2);
            driver.spawn(future);
        }
    }
}


// trait RefParam<'a> {
//     type Ref: CloneParams;
// }

// impl<'a, T1: Clone, T2: Clone> RefParam<'a> for (T1, T2) {
//     type Ref = (&'a T1, &'a T2);
// }

// pub trait RefParams {
//     type 
// }


// pub trait CloneParams {
//     type Cloned: Clone;

//     fn clone_param(self) -> Self::Cloned;
// }

// impl<'a, T1: Clone> CloneParams for &T1 {
//     type Cloned = T1;

//     fn clone_param(self) -> Self::Cloned {
//         self.clone()
//     }
// }

// impl<'a, T1: Clone, T2: Clone> CloneParams for (T1, T2) {
//     type Cloned = (T1, T2);

//     fn clone_param(self) -> Self::Cloned {
//         let (param1, param2) = self;
//         (param1.clone(), param2.clone())
//     }
// }

// pub fn bind_all<
//     K: Clone + 'static,
//     T: CloneParams<Cloned=K>,
//     F: Fn(K) + 'static
// >(param1: T, fun: F) -> impl Fn() {
//     let param1 = param1.clone_param();

//     move || {
//         let param1 = param1.clone();
//         fun(param1);
//     }
// }


// pub fn bind<T: Clone + 'static, F: Fn(T) + 'static>(param1: &T, fun: F) -> impl Fn() {
//     let param1 = param1.clone();

//     move || {
//         let param1 = param1.clone();
//         fun(param1);
//     }
// }

// pub fn bind_ref<T: Clone + 'static, F: Fn(&T) + 'static>(param1: &T, fun: F) -> impl Fn() {
//     let param1 = param1.clone();

//     move || {
//         fun(&param1);
//     }
// }

// pub fn bind2<T1: Clone + 'static, T2: Clone + 'static, F: Fn(T1, T2) + 'static>(param1: &T1, param2: &T2, fun: F) -> impl Fn() {
//     let param1 = param1.clone();
//     let param2 = param2.clone();

//     move || {
//         let param1 = param1.clone();
//         let param2 = param2.clone();
//         fun(param1, param2);
//     }
// }

// pub fn bind2_ref<T1: Clone + 'static, T2: Clone + 'static, F: Fn(&T1, &T2) + 'static>(param1: &T1, param2: &T2, fun: F) -> impl Fn() {
//     let param1 = param1.clone();
//     let param2 = param2.clone();

//     move || {
//         fun(&param1, &param2);
//     }
// }

use std::future::Future;

use vertigo::Driver;
