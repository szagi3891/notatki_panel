use std::process::Output;

use vertigo::{Css, VDomComponent, css, html};

use super::{AppNewdir};
use crate::{components::{button}, app::new_dir::wrap_async::{BoxAsyncFn, PinBoxFuture}};

fn css_wrapper() -> Css {
    css!("
        display: flex;
        flex-direction: column;
        border: 1px solid black;
        background-color: #e0e0e0;
        width: 100vw;
        height: 100vh;
    ")
}

fn css_header() -> Css {
    css!("
        border-bottom: 1px solid black;
        padding: 5px;
    ")
}

pub fn app_newdir_render(view_new_name: VDomComponent, state: AppNewdir) -> VDomComponent {
    VDomComponent::new(state, move |state| {
        let on_click = {
            let state = state.clone();
            move || {
                state.redirect_to_index();
            }
        };

        let parent_path = state.parent.as_slice().join("/");

        let mut buttons = vec!(button("Wróć", on_click));

        let save_enable = state.save_enable.get_value();

        //bind!(state, on_save(true))

        /*

        let callback = {
            let state = state.clone();

            bind!(state.clone().on_save())
        };

        callback będzie typu : |&Driver| {}
        */

        let _aaa: _ = {
            let state = state.clone();
            let gg = move || {
                state.clone().on_save();

                // let future = state.clone().on_save();
                // state.driver.spawn(future);

            };

            let _aa = gg();

            gg
        };
    
        if *save_enable {
            buttons.push(button("Zapisz", {
                let state = state.clone();

                move || {
                    let future = state.clone().on_save();
                    state.driver.spawn(future);
                }

                // wrap_async(move async || {
                //     state.clone().on_save().await;
                // })
            }));

            use std::future::Future;
            fn wrap_async<R, F: Fn() -> Box<dyn Future<Output=R>>>(cl: F) {
                todo!()
            }

            // let aaa = wrap_async(async move || -> () {
            //     state.clone().on_save().await;
            // });

            let ccc = {
                let state = state.clone();
                let aaa2 = BoxAsyncFn::new(move |_: ()| -> PinBoxFuture<()> {

                    let state = state.clone();
                    Box::pin(async move {
                        state.clone().on_save().await;
                    })
                });
            };

            let ddd = || {
                let ccc = async {
                    state.clone().on_save().await;
                };
                Box::new(ccc)
            };


            // buttons.push(button("Zapisz", async_wrap!({
            //     let state = state.clone();
            //     async move || -> () {
            //         state.clone().on_save().await;
            //     }
            // })));

            //Fn() -> PinBoxFuture<()>

            // buttons.push(button("Zapisz", {
            //     let state = state.clone();
            //     move || -> PinBoxFuture<()> {
            //         let state = state.clone();
            //         Box::pin(async move {
            //             state.clone().on_save().await;
            //         })
            //     }
            // }));

            // wrap_async(ddd)

            /*
            buttons.push(button("Zapisz", {
                let state = state.clone();
                bind!(state.clone().on_save());
            }));

            buttons.push(button("Zapisz", {
                let state = state.clone();
                wrap_async!(async move || {                 zamienia na funkcję ---> |&Driver| {}
                    state.clone().on_save().await;
                });
            }));

            buttons.push(button("Zapisz", {
                let state = state.clone();

                move |&Driver| {
                    driver.spawn(state.clone().on_save());
                }
            }));
            */

            fn aaaaaa<K, V, Fut: Future<Output = V> + 'static, F: Fn(K) -> Fut + 'static>(fun: F) {

            }
            let aaa = aaaaaa(async move |ddd: u32| -> u32 {
                todo!()
            });

            use std::rc::Rc;

            struct FFF<K, V> {
                callback: Rc<dyn Fn(K) -> PinBoxFuture<V>>,
            }

            impl<K, V> FFF<K, V> {
                fn new<Fut: Future<Output = V> + 'static, F: Fn(K) -> Fut + 'static>(fun: F) -> FFF<K, V> {
                    let new_fn = Rc::new(move |key: K| -> PinBoxFuture<V> {
                        let value = Box::pin(fun(key));
                        value
                    });

                    FFF { callback: new_fn }
                }
            }

            let aaaaaa = FFF::new(async move |key: u32| -> u32 {
                4
            });

            


            /*

            wrap_async!(async move || -> T {

                //w tym scenariuszu clouser zwracałby boxa z wartością
            })

            wrap_async!(move || async move {

            })
            */
        }

        html! {
            <div css={css_wrapper()}>
                <div css={css_header()}>
                    "tworzenie katalogu => "
                    {parent_path}
                </div>
                <div css={css_header()}>
                    { ..buttons }
                </div>
                { view_new_name.clone() }
            </div>
        }
    })
}