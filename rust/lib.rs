//! # Extra Utilities for JNI in Rust
//!
//! This crate builds on top of the [`jni`](::jni) crate and provides
//! higher-level concepts to more easily deal with JNI. While the
//! [`jni`](::jni) crate implements low-level bindings to JNI,
//! [`jni-utils`](crate) is more focused on higher-level constructs that get
//! used frequently. Some of the features provided by [`jni-utils`](crate)
//! include:
//!
//! * Asynchronous calls to Java code using the [`JFuture`](future::JFuture)
//!   and [`JStream`](stream::JStream) types
//! * Conversion between various commonly-used Rust types and their
//!   corresponding Java types
//! * Emulation of `try`/`catch` blocks with the
//!   [`try_block`](exceptions::try_block) function
//!
//! The overriding principle of [`jni-utils`](crate) is that switches between
//! Rust and Java code should be minimized, and that it is easier to call Java
//! code from Rust than it is to call Rust code from Java. Calling Rust from
//! Java requires creating a class with a `native` method and exporting it from
//! Rust, either by a combination of `#[nomangle]` and `extern "C"` to export
//! the function as a symbol in a shared library, or by calling
//! [`JNIEnv::register_native_methods()`](::jni::JNIEnv::register_native_methods).
//! In contrast, calling Java from Rust only requires calling
//! [`JNIEnv::call_method()`](::jni::JNIEnv::call_method) (though you can cache
//! the method ID and use
//! [`JNIEnv::call_method_unchecked()`](::jni::JNIEnv::call_method_unchecked)
//! for a performance improvement.)
//!
//! To that end, [`jni-utils`](crate) seeks to minimize the number of holes
//! that must be poked through the Rust-Java boundary, and the number of
//! `native` exported-to-Java Rust functions that must be written. In
//! particular, the async API has been developed to minimize such exports by
//! allowing Java code to wake an `await` without creating a new `native`
//! function.
//!
//! Some features of [`jni-utils`](crate) require the accompanying Java support
//! library, which includes some native methods. Therefore,
//! [`jni_utils::init()`](crate::init) should be called before using
//! [`jni-utils`](crate).

use ::jni::{errors::Result, JNIEnv};

pub mod arrays;
pub mod classcache;
pub mod exceptions;
pub mod future;
pub mod ops;
pub mod stream;
pub mod task;
pub mod uuid;

/// Initialize [`jni-utils`](crate) by registering required native methods.
/// This should be called before using [`jni-utils`](crate).
///
/// # Arguments
///
/// * `env` - Java environment with which to register native methods.
pub fn init(env: &JNIEnv) -> Result<()> {
    ops::jni::init(env)?;
    Ok(())
}

#[cfg(test)]
pub(crate) mod test_utils {
    use jni::{objects::GlobalRef, JNIEnv, JavaVM};
    use lazy_static::lazy_static;
    use std::{
        sync::{Arc, Mutex},
        task::{Wake, Waker},
    };

    pub struct TestWakerData(Mutex<bool>);

    impl TestWakerData {
        pub fn new() -> Self {
            Self(Mutex::new(false))
        }

        pub fn value(&self) -> bool {
            *self.0.lock().unwrap()
        }

        pub fn set_value(&self, value: bool) {
            let mut guard = self.0.lock().unwrap();
            *guard = value;
        }
    }

    impl Wake for TestWakerData {
        fn wake(self: Arc<Self>) {
            Self::wake_by_ref(&self);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.set_value(true);
        }
    }

    pub fn test_waker(data: &Arc<TestWakerData>) -> Waker {
        Waker::from(data.clone())
    }

    struct GlobalJVM {
        jvm: JavaVM,
        class_loader: GlobalRef,
    }

    thread_local! {
        pub static JVM_ENV: JNIEnv<'static> = {
            let env = JVM.jvm.attach_current_thread_permanently().unwrap();

            let thread = env
                .call_static_method(
                    "java/lang/Thread",
                    "currentThread",
                    "()Ljava/lang/Thread;",
                    &[],
                )
                .unwrap()
                .l()
                .unwrap();
            env.call_method(
                thread,
                "setContextClassLoader",
                "(Ljava/lang/ClassLoader;)V",
                &[JVM.class_loader.as_obj().into()]
            ).unwrap();

            env
        }
    }

    lazy_static! {
        static ref JVM: GlobalJVM = {
            use jni::InitArgsBuilder;
            use std::{env, path::PathBuf};

            let mut jni_utils_jar = PathBuf::from(env::current_exe().unwrap());
            jni_utils_jar.pop();
            jni_utils_jar.pop();
            jni_utils_jar.push("java");
            jni_utils_jar.push("libs");
            jni_utils_jar.push("jni-utils-0.1.0-SNAPSHOT.jar");

            let jvm_args = InitArgsBuilder::new()
                .option(&format!(
                    "-Djava.class.path={}",
                    jni_utils_jar.to_str().unwrap()
                ))
                .build()
                .unwrap();
            let jvm = JavaVM::new(jvm_args).unwrap();

            let env = jvm.attach_current_thread_permanently().unwrap();
            crate::init(&env).unwrap();

            let thread = env
                .call_static_method(
                    "java/lang/Thread",
                    "currentThread",
                    "()Ljava/lang/Thread;",
                    &[],
                )
                .unwrap()
                .l()
                .unwrap();
            let class_loader = env
                .call_method(
                    thread,
                    "getContextClassLoader",
                    "()Ljava/lang/ClassLoader;",
                    &[],
                )
                .unwrap()
                .l()
                .unwrap();
            let class_loader = env.new_global_ref(class_loader).unwrap();

            GlobalJVM { jvm, class_loader }
        };
    }
}
