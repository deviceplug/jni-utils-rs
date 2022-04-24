use ::jni::{errors::Result, objects::{JObject, JClass}, JNIEnv};
use std::sync::{Arc, Mutex};

macro_rules! define_fn_adapter {
    (
        fn_once: $fo:ident,
        fn_once_local: $fol:ident,
        fn_once_internal: $foi:ident,
        fn_mut: $fm:ident,
        fn_mut_local: $fml:ident,
        fn_mut_internal: $fmi:ident,
        fn: $f:ident,
        fn_local: $fl:ident,
        fn_internal: $fi:ident,
        impl_class: $ic:literal,
        doc_class: $dc:literal,
        doc_method: $dm:literal,
        doc_fn_once: $dfo:literal,
        doc_fn: $df:literal,
        doc_noop: $dnoop:literal,
        signature: $closure_name:ident: impl for<'c, 'd> Fn$args:tt -> $ret:ty,
        closure: $closure:expr,
    ) => {
        fn $foi<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            $closure_name: impl for<'c, 'd> FnOnce$args -> $ret + 'static,
            local: bool,
        ) -> Result<JObject<'a>> {
            let adapter = env.auto_local(fn_once_adapter(env, $closure, local)?);
            env.new_object(
                JClass::from(crate::classcache::get_class($ic).unwrap().as_obj()),
                "(Lio/github/gedgygedgy/rust/ops/FnAdapter;)V",
                &[(&adapter).into()],
            )
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`FnOnce`]. The closure can later be called "]
        #[doc = "by calling the object's `"]
        #[doc = $dm]
        #[doc = "` method. The closure can be freed without calling it by "]
        #[doc = "calling the object's `close()` method."]
        #[doc = "\n\n"]
        #[doc = "If the closure panics, the unwind will be caught and thrown "]
        #[doc = "as an `io.github.gedgygedgy.rust.panic.PanicException`."]
        #[doc = "\n\n"]
        #[doc = "It is safe to call the object's `"]
        #[doc = $dm]
        #[doc = "` method recursively, but the second call will "]
        #[doc = $dnoop]
        #[doc = "."]
        pub fn $fo<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> FnOnce$args -> $ret + Send + 'static,
        ) -> Result<JObject<'a>> {
            $foi(env, f, false)
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`FnOnce`] without checking if it is "]
        #[doc = "[`Send`]. Attempting to call `"]
        #[doc = $dm]
        #[doc = "` or `close()` on the resulting object from a thread other "]
        #[doc = "than its origin thread will result in an "]
        #[doc = "`io.github.gedgygedgy.rust.thread.LocalThreadException` "]
        #[doc = "being thrown."]
        pub fn $fol<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> FnOnce$args -> $ret + 'static,
        ) -> Result<JObject<'a>> {
            $foi(env, f, true)
        }

        fn $fmi<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            mut $closure_name: impl for<'c, 'd> FnMut$args -> $ret + 'static,
            local: bool,
        ) -> Result<JObject<'a>> {
            let adapter = env.auto_local(fn_mut_adapter(env, $closure, local)?);
            env.new_object(
                JClass::from(crate::classcache::get_class($ic).unwrap().as_obj()),
                "(Lio/github/gedgygedgy/rust/ops/FnAdapter;)V",
                &[(&adapter).into()],
            )
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`FnMut`]. The closure can later be called "]
        #[doc = "by calling the object's `"]
        #[doc = $dm]
        #[doc = "` method. The closure can be freed without calling it by "]
        #[doc = "calling the object's `close()` method."]
        #[doc = "\n\n"]
        #[doc = "If the closure panics, the unwind will be caught and thrown "]
        #[doc = "as an `io.github.gedgygedgy.rust.panic.PanicException`."]
        #[doc = "\n\n"]
        #[doc = "Unlike [`"]
        #[doc = $df]
        #[doc = "`] and [`"]
        #[doc = $dfo]
        #[doc = "`], it is not safe to call the resulting object's `"]
        #[doc = $dm]
        #[doc = "` method recursively. The [`FnMut`] is managed with an "]
        #[doc = "internal [`Mutex`], so calling `"]
        #[doc = $dm]
        #[doc = "` recursively will result in a deadlock."]
        pub fn $fm<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> FnMut$args -> $ret + Send + 'static,
        ) -> Result<JObject<'a>> {
            $fmi(env, f, false)
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`FnMut`] without checking if it is "]
        #[doc = "[`Send`]. Attempting to call `"]
        #[doc = $dm]
        #[doc = "` or `close()` on the resulting object from a thread other "]
        #[doc = "than its origin thread will result in an "]
        #[doc = "`io.github.gedgygedgy.rust.thread.LocalThreadException` "]
        #[doc = "being thrown."]
        pub fn $fml<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> FnMut$args -> $ret + 'static,
        ) -> Result<JObject<'a>> {
            $fmi(env, f, true)
        }

        fn $fi<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            $closure_name: impl for<'c, 'd> Fn$args -> $ret + 'static,
            local: bool,
        ) -> Result<JObject<'a>> {
            let adapter = env.auto_local(fn_adapter(env, $closure, local)?);
            env.new_object(
                JClass::from(crate::classcache::get_class($ic).unwrap().as_obj()),
                "(Lio/github/gedgygedgy/rust/ops/FnAdapter;)V",
                &[(&adapter).into()],
            )
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`Fn`]. The closure can later be called by "]
        #[doc = "calling the object's `"]
        #[doc = $dm]
        #[doc = "` method. The closure can be freed without calling it by "]
        #[doc = "calling the object's `close()` method."]
        #[doc = "\n\n"]
        #[doc = "If the closure panics, the unwind will be caught and thrown "]
        #[doc = "as an `io.github.gedgygedgy.rust.panic.PanicException`."]
        #[doc = "\n\n"]
        #[doc = "It is safe to call the object's `"]
        #[doc = $dm]
        #[doc = "` method recursively."]
        pub fn $f<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> Fn$args -> $ret + Send + Sync + 'static,
        ) -> Result<JObject<'a>> {
            $fi(env, f, false)
        }

        #[doc = "Create an `"]
        #[doc = $dc]
        #[doc = "` from a given [`Fn`] without checking if it is [`Send`]. "]
        #[doc = "Attempting to call `"]
        #[doc = $dm]
        #[doc = "` or `close()` on the resulting object from a thread other "]
        #[doc = "than its origin thread will result in an "]
        #[doc = "`io.github.gedgygedgy.rust.thread.LocalThreadException` "]
        #[doc = "being thrown."]
        pub fn $fl<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            f: impl for<'c, 'd> Fn$args -> $ret + 'static,
        ) -> Result<JObject<'a>> {
            $fi(env, f, true)
        }
    };
}

define_fn_adapter! {
    fn_once: fn_once_runnable,
    fn_once_local: fn_once_runnable_local,
    fn_once_internal: fn_once_runnable_internal,
    fn_mut: fn_mut_runnable,
    fn_mut_local: fn_mut_runnable_local,
    fn_mut_internal: fn_mut_runnable_internal,
    fn: fn_runnable,
    fn_local: fn_runnable_local,
    fn_internal: fn_runnable_internal,
    impl_class: "io/github/gedgygedgy/rust/ops/FnRunnableImpl",
    doc_class: "io.github.gedgygedgy.rust.ops.FnRunnable",
    doc_method: "run()",
    doc_fn_once: "fn_once_runnable",
    doc_fn: "fn_runnable",
    doc_noop: "be a no-op",
    signature: f: impl for<'c, 'd> Fn(&'d JNIEnv<'c>, JObject<'c>) -> (),
    closure: move |env, _obj1, obj2, _arg1, _arg2| {
        f(env, obj2);
        JObject::null()
    },
}

define_fn_adapter! {
    fn_once: fn_once_bi_function,
    fn_once_local: fn_once_bi_function_local,
    fn_once_internal: fn_once_bi_function_internal,
    fn_mut: fn_mut_bi_function,
    fn_mut_local: fn_mut_bi_function_local,
    fn_mut_internal: fn_mut_bi_function_internal,
    fn: fn_bi_function,
    fn_local: fn_bi_function_local,
    fn_internal: fn_bi_function_internal,
    impl_class: "io/github/gedgygedgy/rust/ops/FnBiFunctionImpl",
    doc_class: "io.github.gedgygedgy.rust.ops.FnBiFunction",
    doc_method: "apply()",
    doc_fn_once: "fn_once_bi_function",
    doc_fn: "fn_bi_funciton",
    doc_noop: "return `null`",
    signature: f: impl for<'c, 'd> Fn(&'d JNIEnv<'c>, JObject<'c>, JObject<'c>, JObject<'c>) -> JObject<'c>,
    closure: move |env, _obj1, obj2, arg1, arg2| {
        f(env, obj2, arg1, arg2)
    },
}

define_fn_adapter! {
    fn_once: fn_once_function,
    fn_once_local: fn_once_function_local,
    fn_once_internal: fn_once_function_internal,
    fn_mut: fn_mut_function,
    fn_mut_local: fn_mut_function_local,
    fn_mut_internal: fn_mut_function_internal,
    fn: fn_function,
    fn_local: fn_function_local,
    fn_internal: fn_function_internal,
    impl_class: "io/github/gedgygedgy/rust/ops/FnFunctionImpl",
    doc_class: "io.github.gedgygedgy.rust.ops.FnFunction",
    doc_method: "apply()",
    doc_fn_once: "fn_once_function",
    doc_fn: "fn_function",
    doc_noop: "return `null`",
    signature: f: impl for<'c, 'd> Fn(&'d JNIEnv<'c>, JObject<'c>, JObject<'c>) -> JObject<'c>,
    closure: move |env, _obj1, obj2, arg1, _arg2| {
        f(env, obj2, arg1)
    },
}

struct SendSyncWrapper<T>(T);

unsafe impl<T> Send for SendSyncWrapper<T> {}
unsafe impl<T> Sync for SendSyncWrapper<T> {}

type FnWrapper = SendSyncWrapper<
    Arc<
        dyn for<'a, 'b> Fn(
                &'b JNIEnv<'a>,
                JObject<'a>,
                JObject<'a>,
                JObject<'a>,
                JObject<'a>,
            ) -> JObject<'a>
            + 'static,
    >,
>;

fn fn_once_adapter<'a: 'b, 'b>(
    env: &'b JNIEnv<'a>,
    f: impl for<'c, 'd> FnOnce(
            &'d JNIEnv<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
        ) -> JObject<'c>
        + 'static,
    local: bool,
) -> Result<JObject<'a>> {
    let mutex = Mutex::new(Some(f));
    fn_adapter(
        env,
        move |env, obj1, obj2, arg1, arg2| {
            let f = {
                let mut guard = mutex.lock().unwrap();
                if let Some(f) = guard.take() {
                    f
                } else {
                    return JObject::null();
                }
            };
            f(env, obj1, obj2, arg1, arg2)
        },
        local,
    )
}

fn fn_mut_adapter<'a: 'b, 'b>(
    env: &'b JNIEnv<'a>,
    f: impl for<'c, 'd> FnMut(
            &'d JNIEnv<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
        ) -> JObject<'c>
        + 'static,
    local: bool,
) -> Result<JObject<'a>> {
    let mutex = Mutex::new(f);
    fn_adapter(
        env,
        move |env, obj1, obj2, arg1, arg2| {
            let mut guard = mutex.lock().unwrap();
            guard(env, obj1, obj2, arg1, arg2)
        },
        local,
    )
}

fn fn_adapter<'a: 'b, 'b>(
    env: &'b JNIEnv<'a>,
    f: impl for<'c, 'd> Fn(
            &'d JNIEnv<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
        ) -> JObject<'c>
        + 'static,
    local: bool,
) -> Result<JObject<'a>> {
    let arc: Arc<
        dyn for<'c, 'd> Fn(
            &'d JNIEnv<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
            JObject<'c>,
        ) -> JObject<'c>,
    > = Arc::from(f);

    let obj = env.new_object(
        JClass::from(crate::classcache::get_class("io/github/gedgygedgy/rust/ops/FnAdapter").unwrap().as_obj()),
        "(Z)V", 
        &[local.into()]
    )?;
    env.set_rust_field::<_, _, FnWrapper>(obj, "data", SendSyncWrapper(arc))?;
    Ok(obj)
}

pub(crate) mod jni {
    use super::FnWrapper;
    use jni::{errors::Result, objects::JObject, JNIEnv, NativeMethod};

    extern "C" fn fn_adapter_call_internal<'a>(
        env: JNIEnv<'a>,
        obj1: JObject<'a>,
        obj2: JObject<'a>,
        arg1: JObject<'a>,
        arg2: JObject<'a>,
    ) -> JObject<'a> {
        use std::panic::AssertUnwindSafe;

        let arc = if let Ok(f) = env.get_rust_field::<_, _, FnWrapper>(obj1, "data") {
            AssertUnwindSafe(f.0.clone())
        } else {
            return JObject::null();
        };
        crate::exceptions::throw_unwind(&env, || arc(&env, obj1, obj2, arg1, arg2))
            .unwrap_or_else(|_| JObject::null())
    }

    extern "C" fn fn_adapter_close_internal(env: JNIEnv, obj: JObject) {
        let _ = crate::exceptions::throw_unwind(&env, || {
            let _ = env.take_rust_field::<_, _, FnWrapper>(obj, "data");
        });
    }

    pub fn init(env: &JNIEnv) -> Result<()> {
        use std::ffi::c_void;
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/future/Future").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/future/FutureException").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/ops/FnAdapter").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/stream/Stream").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/stream/StreamPoll").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/task/Waker").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/task/PollResult").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/ops/FnRunnableImpl").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/ops/FnBiFunctionImpl").unwrap();
        crate::classcache::find_add_class(env, "io/github/gedgygedgy/rust/ops/FnFunctionImpl").unwrap();
        let class = env.auto_local(env.find_class("io/github/gedgygedgy/rust/ops/FnAdapter")?);
        env.register_native_methods(
            &class,
            &[
                NativeMethod {
                    name: "callInternal".into(),
                    sig:
                        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
                            .into(),
                    fn_ptr: fn_adapter_call_internal as *mut c_void,
                },
                NativeMethod {
                    name: "closeInternal".into(),
                    sig: "()V".into(),
                    fn_ptr: fn_adapter_close_internal as *mut c_void,
                },
            ],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{exceptions::try_block, test_utils};
    use jni::{objects::JObject, JNIEnv};
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    fn create_test_fn() -> (
        Arc<Mutex<u32>>,
        Box<dyn for<'a, 'b> Fn(&'b JNIEnv<'a>, JObject<'a>) + Send + Sync + 'static>,
    ) {
        let arc = Arc::new(Mutex::new(0));
        let arc2 = arc.clone();
        (
            arc,
            Box::new(move |_e, _o| {
                let mut guard = arc2.lock().unwrap();
                *&mut *guard += 1;
            }),
        )
    }

    fn create_test_fn_local() -> (
        Rc<RefCell<u32>>,
        Box<dyn for<'a, 'b> Fn(&'b JNIEnv<'a>, JObject<'a>) + 'static>,
    ) {
        let rc = Rc::new(RefCell::new(0));
        let rc2 = rc.clone();
        (
            rc,
            Box::new(move |_e, _o| {
                let mut guard = rc2.try_borrow_mut().unwrap();
                *&mut *guard += 1;
            }),
        )
    }

    fn test_data(data: &Arc<Mutex<u32>>, expected: u32, expected_refcount: usize) {
        assert_eq!(Arc::strong_count(data), expected_refcount);
        let guard = data.lock().unwrap();
        assert_eq!(*guard, expected);
    }

    fn test_data_local(data: &Rc<RefCell<u32>>, expected: u32, expected_refcount: usize) {
        assert_eq!(Rc::strong_count(data), expected_refcount);
        let guard = data.try_borrow().unwrap();
        assert_eq!(*guard, expected);
    }

    struct DropPanic;

    impl DropPanic {
        pub fn keep_alive(&self) {}
    }

    impl Drop for DropPanic {
        fn drop(&mut self) {
            panic!("DropPanic dropped");
        }
    }

    fn create_drop_panic_fn(
    ) -> Box<dyn for<'a, 'b> Fn(&'b JNIEnv<'a>, JObject<'a>) + Send + Sync + 'static> {
        let p = DropPanic;
        Box::new(move |_e, _o| {
            p.keep_alive();
        })
    }

    #[test]
    fn test_drop_panic() {
        test_utils::JVM_ENV.with(|env| {
            use std::{
                mem::drop,
                panic::{catch_unwind, AssertUnwindSafe},
            };

            let dp = AssertUnwindSafe(create_drop_panic_fn());
            dp(env, JObject::null());
            catch_unwind(|| drop(dp)).unwrap_err();
        });
    }

    #[test]
    fn test_fn_once_runnable_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_run_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_close_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            let runnable = env.new_global_ref(runnable).unwrap();
            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    env.call_method(runnable.as_obj(), "run", "()V", &[])
                        .unwrap();
                });
            });
            thread.join().unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_object() {
        test_utils::JVM_ENV.with(|env| {
            let obj_ref = Arc::new(Mutex::new(env.new_global_ref(JObject::null()).unwrap()));
            let obj_ref_2 = obj_ref.clone();
            let runnable = super::fn_once_runnable(env, move |e, o| {
                let guard = obj_ref_2.lock().unwrap();
                assert!(e.is_same_object(guard.as_obj(), o).unwrap());
            })
            .unwrap();

            {
                let mut guard = obj_ref.lock().unwrap();
                *guard = env.new_global_ref(runnable).unwrap();
            }

            env.call_method(runnable, "run", "()V", &[]).unwrap();
        });
    }

    #[test]
    fn test_fn_once_runnable_recursive() {
        test_utils::JVM_ENV.with(|env| {
            let arc = Arc::new(Mutex::new(false));
            let arc2 = arc.clone();

            let runnable = super::fn_once_runnable(env, move |env, obj| {
                let value = {
                    let mut guard = arc2.lock().unwrap();
                    let old = *guard;
                    *guard = true;
                    old
                };
                if !value {
                    env.call_method(obj, "run", "()V", &[]).unwrap();
                }
            })
            .unwrap();

            env.call_method(runnable, "run", "()V", &[]).unwrap();

            let guard = arc.lock().unwrap();
            assert!(*guard);
        })
    }

    #[test]
    fn test_fn_once_runnable_panic() {
        test_utils::JVM_ENV.with(|env| {
            let runnable =
                super::fn_once_runnable(env, |_e, _o| panic!("This is a panic")).unwrap();
            let result = try_block(env, || {
                env.call_method(runnable, "run", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let str = any.downcast::<&str>().unwrap();
                assert_eq!(*str, "This is a panic");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_once_runnable_close_self() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_once_runnable(env, move |env, obj| {
                env.call_method(obj, "close", "()V", &[]).unwrap();
                f(env, obj);
            })
            .unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_drop_panic() {
        test_utils::JVM_ENV.with(|env| {
            let dp = create_drop_panic_fn();
            let runnable = super::fn_once_runnable(env, dp).unwrap();

            let result = try_block(env, || {
                env.call_method(runnable, "close", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let msg = any.downcast::<&str>().unwrap();
                assert_eq!(*msg, "DropPanic dropped");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_once_runnable_local_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_once_runnable_local(env, f).unwrap();
            test_data_local(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 1, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_once_runnable_local_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_once_runnable_local(env, f).unwrap();
            let runnable = env.new_global_ref(runnable).unwrap();
            test_data_local(&data, 0, 2);

            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "run", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);

                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "close", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);
                });
            });
            thread.join().unwrap();
            test_data_local(&data, 0, 2);
        });
    }

    #[test]
    fn test_fn_mut_runnable_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 2, 2);
        });
    }

    #[test]
    fn test_fn_mut_runnable_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_mut_runnable_run_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_mut_runnable_close_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_mut_runnable_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            let runnable = env.new_global_ref(runnable).unwrap();
            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    env.call_method(runnable.as_obj(), "run", "()V", &[])
                        .unwrap();
                });
            });
            thread.join().unwrap();
            test_data(&data, 1, 2);
        });
    }

    #[test]
    fn test_fn_mut_runnable_object() {
        test_utils::JVM_ENV.with(|env| {
            let obj_ref = Arc::new(Mutex::new(env.new_global_ref(JObject::null()).unwrap()));
            let obj_ref_2 = obj_ref.clone();
            let runnable = super::fn_mut_runnable(env, move |e, o| {
                let guard = obj_ref_2.lock().unwrap();
                assert!(e.is_same_object(guard.as_obj(), o).unwrap());
            })
            .unwrap();

            {
                let mut guard = obj_ref.lock().unwrap();
                *guard = env.new_global_ref(runnable).unwrap();
            }

            env.call_method(runnable, "run", "()V", &[]).unwrap();
        });
    }

    #[test]
    fn test_fn_mut_runnable_panic() {
        test_utils::JVM_ENV.with(|env| {
            let runnable = super::fn_mut_runnable(env, |_e, _o| panic!("This is a panic")).unwrap();
            let result = try_block(env, || {
                env.call_method(runnable, "run", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let str = any.downcast::<&str>().unwrap();
                assert_eq!(*str, "This is a panic");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_mut_runnable_close_self() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_mut_runnable(env, move |env, obj| {
                env.call_method(obj, "close", "()V", &[]).unwrap();
                f(env, obj);
            })
            .unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_mut_runnable_drop_panic() {
        test_utils::JVM_ENV.with(|env| {
            let dp = create_drop_panic_fn();
            let runnable = super::fn_mut_runnable(env, dp).unwrap();

            let result = try_block(env, || {
                env.call_method(runnable, "close", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let msg = any.downcast::<&str>().unwrap();
                assert_eq!(*msg, "DropPanic dropped");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_mut_runnable_local_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_mut_runnable_local(env, f).unwrap();
            test_data_local(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 1, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 2, 2);
        });
    }

    #[test]
    fn test_fn_mut_runnable_local_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_mut_runnable_local(env, f).unwrap();
            let runnable = env.new_global_ref(runnable).unwrap();
            test_data_local(&data, 0, 2);

            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "run", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);

                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "close", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);
                });
            });
            thread.join().unwrap();
            test_data_local(&data, 0, 2);
        });
    }

    #[test]
    fn test_fn_runnable_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 2, 2);
        });
    }

    #[test]
    fn test_fn_runnable_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_runnable_run_close() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_runnable_close_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "close", "()V", &[]).unwrap();
            test_data(&data, 0, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 0, 1);
        });
    }

    #[test]
    fn test_fn_runnable_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, f).unwrap();
            test_data(&data, 0, 2);

            let runnable = env.new_global_ref(runnable).unwrap();
            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    env.call_method(runnable.as_obj(), "run", "()V", &[])
                        .unwrap();
                });
            });
            thread.join().unwrap();
            test_data(&data, 1, 2);
        });
    }

    #[test]
    fn test_fn_runnable_object() {
        test_utils::JVM_ENV.with(|env| {
            let obj_ref = Arc::new(Mutex::new(env.new_global_ref(JObject::null()).unwrap()));
            let obj_ref_2 = obj_ref.clone();
            let runnable = super::fn_runnable(env, move |e, o| {
                let guard = obj_ref_2.lock().unwrap();
                assert!(e.is_same_object(guard.as_obj(), o).unwrap());
            })
            .unwrap();

            {
                let mut guard = obj_ref.lock().unwrap();
                *guard = env.new_global_ref(runnable).unwrap();
            }

            env.call_method(runnable, "run", "()V", &[]).unwrap();
        });
    }

    #[test]
    fn test_fn_runnable_recursive() {
        test_utils::JVM_ENV.with(|env| {
            let arc = Arc::new(Mutex::new(false));
            let arc2 = arc.clone();

            let calling = Mutex::new(false);

            let runnable = super::fn_runnable(env, move |env, obj| {
                let calling_value = {
                    let mut guard = calling.lock().unwrap();
                    let old = *guard;
                    *guard = true;
                    old
                };
                if !calling_value {
                    env.call_method(obj, "run", "()V", &[]).unwrap();
                    let mut guard = calling.lock().unwrap();
                    *guard = false;
                } else {
                    let mut guard = arc2.lock().unwrap();
                    *guard = true;
                }
            })
            .unwrap();

            env.call_method(runnable, "run", "()V", &[]).unwrap();

            let guard = arc.lock().unwrap();
            assert!(*guard);
        })
    }

    #[test]
    fn test_fn_runnable_panic() {
        test_utils::JVM_ENV.with(|env| {
            let runnable = super::fn_runnable(env, |_e, _o| panic!("This is a panic")).unwrap();
            let result = try_block(env, || {
                env.call_method(runnable, "run", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let str = any.downcast::<&str>().unwrap();
                assert_eq!(*str, "This is a panic");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_runnable_close_self() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn();
            test_data(&data, 0, 2);

            let runnable = super::fn_runnable(env, move |env, obj| {
                env.call_method(obj, "close", "()V", &[]).unwrap();
                f(env, obj);
            })
            .unwrap();
            test_data(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data(&data, 1, 1);
        });
    }

    #[test]
    fn test_fn_runnable_drop_panic() {
        test_utils::JVM_ENV.with(|env| {
            let dp = create_drop_panic_fn();
            let runnable = super::fn_runnable(env, dp).unwrap();

            let result = try_block(env, || {
                env.call_method(runnable, "close", "()V", &[])?;
                Ok(false)
            })
            .catch("io/github/gedgygedgy/rust/panic/PanicException", |ex| {
                let ex = crate::exceptions::JPanicException::from_env(env, ex).unwrap();
                let any = ex.take().unwrap();
                let msg = any.downcast::<&str>().unwrap();
                assert_eq!(*msg, "DropPanic dropped");
                Ok(true)
            })
            .result()
            .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_fn_runnable_local_run() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_runnable_local(env, f).unwrap();
            test_data_local(&data, 0, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 1, 2);

            env.call_method(runnable, "run", "()V", &[]).unwrap();
            test_data_local(&data, 2, 2);
        });
    }

    #[test]
    fn test_fn_runnable_local_thread() {
        test_utils::JVM_ENV.with(|env| {
            let (data, f) = create_test_fn_local();
            test_data_local(&data, 0, 2);

            let runnable = super::fn_runnable_local(env, f).unwrap();
            let runnable = env.new_global_ref(runnable).unwrap();
            test_data_local(&data, 0, 2);

            let thread = std::thread::spawn(move || {
                test_utils::JVM_ENV.with(|env| {
                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "run", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);

                    let value = crate::exceptions::try_block(env, || {
                        env.call_method(runnable.as_obj(), "close", "()V", &[])?;
                        Ok(false)
                    })
                    .catch(
                        "io/github/gedgygedgy/rust/thread/LocalThreadException",
                        |_ex| Ok(true),
                    )
                    .result()
                    .unwrap();
                    assert!(value);
                });
            });
            thread.join().unwrap();
            test_data_local(&data, 0, 2);
        });
    }

    #[test]
    fn test_fn_bi_function_object() {
        test_utils::JVM_ENV.with(|env| {
            let arg1 = env
                .new_global_ref(env.new_object("java/lang/Object", "()V", &[]).unwrap())
                .unwrap();
            let arg2 = env
                .new_global_ref(env.new_object("java/lang/Object", "()V", &[]).unwrap())
                .unwrap();
            let ret = env
                .new_global_ref(env.new_object("java/lang/Object", "()V", &[]).unwrap())
                .unwrap();
            let arg1_clone = arg1.clone();
            let arg2_clone = arg2.clone();
            let ret_clone = ret.clone();

            let obj_ref = Arc::new(Mutex::new(env.new_global_ref(JObject::null()).unwrap()));
            let obj_ref_2 = obj_ref.clone();
            let bi_function = super::fn_bi_function(env, move |e, o, a1, a2| {
                let guard = obj_ref_2.lock().unwrap();
                assert!(e.is_same_object(guard.as_obj(), o).unwrap());
                assert!(e.is_same_object(arg1_clone.as_obj(), a1).unwrap());
                assert!(e.is_same_object(arg2_clone.as_obj(), a2).unwrap());
                ret_clone.as_obj().into_inner().into()
            })
            .unwrap();

            {
                let mut guard = obj_ref.lock().unwrap();
                *guard = env.new_global_ref(bi_function).unwrap();
            }

            let actual_ret = env
                .call_method(
                    bi_function,
                    "apply",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    &[arg1.as_obj().into(), arg2.as_obj().into()],
                )
                .unwrap()
                .l()
                .unwrap();
            assert!(env.is_same_object(ret.as_obj(), actual_ret).unwrap());
        });
    }

    #[test]
    fn test_fn_function_object() {
        test_utils::JVM_ENV.with(|env| {
            let arg = env
                .new_global_ref(env.new_object("java/lang/Object", "()V", &[]).unwrap())
                .unwrap();
            let ret = env
                .new_global_ref(env.new_object("java/lang/Object", "()V", &[]).unwrap())
                .unwrap();
            let arg_clone = arg.clone();
            let ret_clone = ret.clone();

            let obj_ref = Arc::new(Mutex::new(env.new_global_ref(JObject::null()).unwrap()));
            let obj_ref_2 = obj_ref.clone();
            let function = super::fn_function(env, move |e, o, a| {
                let guard = obj_ref_2.lock().unwrap();
                assert!(e.is_same_object(guard.as_obj(), o).unwrap());
                assert!(e.is_same_object(arg_clone.as_obj(), a).unwrap());
                ret_clone.as_obj().into_inner().into()
            })
            .unwrap();

            {
                let mut guard = obj_ref.lock().unwrap();
                *guard = env.new_global_ref(function).unwrap();
            }

            let actual_ret = env
                .call_method(
                    function,
                    "apply",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    &[arg.as_obj().into()],
                )
                .unwrap()
                .l()
                .unwrap();
            assert!(env.is_same_object(ret.as_obj(), actual_ret).unwrap());
        });
    }
}
