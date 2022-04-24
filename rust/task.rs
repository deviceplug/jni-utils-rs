use ::jni::{
    errors::Result,
    objects::{JMethodID, JObject, JClass},
    signature::JavaType,
    JNIEnv,
};
use std::task::Waker;

/// Wraps the given waker in a `io.github.gedgygedgy.rust.task.Waker` object.
///
/// Calling this function is generally not necessary, since
/// [`JFuture`](crate::future::JFuture) and [`JStream`](crate::stream::JStream)
/// take care of it for you.
///
/// # Arguments
///
/// * `env` - Java environment in which to create the object.
/// * `waker` - Waker to wrap in a Java object.
pub fn waker<'a: 'b, 'b>(env: &'b JNIEnv<'a>, waker: Waker) -> Result<JObject<'a>> {
    let runnable = crate::ops::fn_once_runnable(env, |_e, _o| waker.wake())?;

    let obj = env.new_object(
        JClass::from(crate::classcache::get_class("io/github/gedgygedgy/rust/task/Waker").unwrap().as_obj()),
        "(Lio/github/gedgygedgy/rust/ops/FnRunnable;)V",
        &[runnable.into()],
    )?;
    Ok(obj)
}

/// Wrapper for [`JObject`]s that implement
/// `io.github.gedgygedgy.rust.task.PollResult`. Provides method to get the
/// poll result.
///
/// Looks up the class and method IDs on creation rather than for every method
/// call.
pub struct JPollResult<'a: 'b, 'b> {
    internal: JObject<'a>,
    get: JMethodID<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> JPollResult<'a, 'b> {
    /// Create a [`JPollResult`] from the environment and an object. This looks
    /// up the necessary class and method IDs to call all of the methods on it
    /// so that extra work doesn't need to be done on every method call.
    ///
    /// # Arguments
    ///
    /// * `env` - Java environment to use.
    /// * `obj` - Object to wrap.
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<Self> {
        let get = env.get_method_id(
            JClass::from(crate::classcache::get_class("io/github/gedgygedgy/rust/task/PollResult").unwrap().as_obj()), 
            "get", 
            "()Ljava/lang/Object;")?;
        Ok(Self {
            internal: obj,
            get,
            env,
        })
    }

    /// Gets the object associated with the [`JPollResult`] by calling
    /// `io.github.gedgygedgy.rust.task.PollResult.get()`. Can throw an
    /// exception.
    pub fn get(&self) -> Result<JObject<'a>> {
        self.env
            .call_method_unchecked(
                self.internal,
                self.get,
                JavaType::Object("java/lang/Object".into()),
                &[],
            )?
            .l()
    }
}

impl<'a: 'b, 'b> ::std::ops::Deref for JPollResult<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JPollResult<'a, 'b>> for JObject<'a> {
    fn from(other: JPollResult<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils;
    use std::sync::Arc;

    #[test]
    fn test_waker_wake() {
        test_utils::JVM_ENV.with(|env| {
            let data = Arc::new(test_utils::TestWakerData::new());
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), false);

            let waker = crate::test_utils::test_waker(&data);
            assert_eq!(Arc::strong_count(&data), 2);
            assert_eq!(data.value(), false);

            let jwaker = super::waker(env, waker).unwrap();
            assert_eq!(Arc::strong_count(&data), 2);
            assert_eq!(data.value(), false);

            env.call_method(jwaker, "wake", "()V", &[]).unwrap();
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), true);
            data.set_value(false);

            env.call_method(jwaker, "wake", "()V", &[]).unwrap();
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), false);
        });
    }

    #[test]
    fn test_waker_close_wake() {
        test_utils::JVM_ENV.with(|env| {
            let data = Arc::new(test_utils::TestWakerData::new());
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), false);

            let waker = crate::test_utils::test_waker(&data);
            assert_eq!(Arc::strong_count(&data), 2);
            assert_eq!(data.value(), false);

            let jwaker = super::waker(env, waker).unwrap();
            assert_eq!(Arc::strong_count(&data), 2);
            assert_eq!(data.value(), false);

            env.call_method(jwaker, "close", "()V", &[]).unwrap();
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), false);

            env.call_method(jwaker, "wake", "()V", &[]).unwrap();
            assert_eq!(Arc::strong_count(&data), 1);
            assert_eq!(data.value(), false);
        });
    }
}
