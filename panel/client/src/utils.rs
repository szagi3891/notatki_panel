
// trait RefParam<'a> {
//     type Ref: CloneParams;
// }

// impl<'a, T1: Clone, T2: Clone> RefParam<'a> for (T1, T2) {
//     type Ref = (&'a T1, &'a T2);
// }

// pub trait RefParams {
//     type 
// }


pub trait CloneParams<'a> {
    type Cloned: Clone;

    fn clone_param(self) -> Self::Cloned;
}

impl<'a, T1: Clone> CloneParams<'a> for &'a T1 {
    type Cloned = T1;

    fn clone_param(self) -> Self::Cloned {
        self.clone()
    }
}

impl<'a, T1: Clone, T2: Clone> CloneParams<'a> for (&'a T1, &'a T2) {
    type Cloned = (T1, T2);

    fn clone_param(self) -> Self::Cloned {
        let (param1, param2) = self;
        (param1.clone(), param2.clone())
    }
}

pub fn bind_all<
    'a,
    K: Clone + 'static,
    T: CloneParams<'a, Cloned=K>,
    F: Fn(K) + 'static
>(param1: T, fun: F) -> impl Fn() {
    let param1 = param1.clone_param();

    move || {
        let param1 = param1.clone();
        fun(param1);
    }
}


// pub fn bind<T: Clone + 'static, F: Fn(T) + 'static>(param1: &T, fun: F) -> impl Fn() {
//     let param1 = param1.clone();

//     move || {
//         let param1 = param1.clone();
//         fun(param1);
//     }
// }

pub fn bind_ref<T: Clone + 'static, F: Fn(&T) + 'static>(param1: &T, fun: F) -> impl Fn() {
    let param1 = param1.clone();

    move || {
        fun(&param1);
    }
}

// pub fn bind2<T1: Clone + 'static, T2: Clone + 'static, F: Fn(T1, T2) + 'static>(param1: &T1, param2: &T2, fun: F) -> impl Fn() {
//     let param1 = param1.clone();
//     let param2 = param2.clone();

//     move || {
//         let param1 = param1.clone();
//         let param2 = param2.clone();
//         fun(param1, param2);
//     }
// }

pub fn bind2_ref<T1: Clone + 'static, T2: Clone + 'static, F: Fn(&T1, &T2) + 'static>(param1: &T1, param2: &T2, fun: F) -> impl Fn() {
    let param1 = param1.clone();
    let param2 = param2.clone();

    move || {
        fun(&param1, &param2);
    }
}
