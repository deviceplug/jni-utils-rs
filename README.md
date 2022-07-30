# Extra Utilities for JNI in Rust

This crate builds on top of the [`jni`](https://github.com/jni-rs/jni-rs) crate
and provides higher-level concepts to more easily deal with JNI. While the
`jni` crate implements low-level bindings to JNI, `jni-utils` is more focused
on higher-level constructs that get used frequently. Some of the features
provided by `jni-utils` include:

* Asynchronous calls to Java code using the `JFuture` and `JStream` types
* Conversion between various commonly-used Rust types and their corresponding
  Java types
* Emulation of `try`/`catch` blocks with the `try_block` function

The overriding principle of `jni-utils` is that switches between Rust and Java
code should be minimized, and that it is easier to call Java code from Rust
than it is to call Rust code from Java. Calling Rust from Java requires
creating a class with a `native` method and exporting it from Rust, either by a
combination of `#[nomangle]` and `extern "C"` to export the function as a
symbol in a shared library, or by calling `JNIEnv::register_native_methods()`.
In contrast, calling Java from Rust only requires calling
`JNIEnv::call_method()` (though you can cache the method ID and use
`JNIEnv::call_method_unchecked()` for a performance improvement.)

To that end, `jni-utils` seeks to minimize the number of holes that must be
poked through the Rust-Java boundary, and the number of `native`
exported-to-Java Rust functions that must be written. In particular, the async
API has been developed to minimize such exports by allowing Java code to wake
an `await` without creating a new `native` function.

Some features of `jni-utils` require the accompanying Java support library,
which includes some native methods. Therefore, `jni_utils::init()` should be
called before using `jni-utils`.

## Library History and Notes

While I (qDot/Kyle Machulis) am now maintaining this library, the original author of most of this was gedgygedgy. The original repo is at https://github.com/gedgygedgy/jni-utils-rs. The author disappeared in August 2021, and I've not been able to make contact with them. That said, the work they did on btleplug and other projects was mostly finished, and the license was permissive enough to redistribute, so I'm taking over maintainership to try to get it out to the world.

I'm by no means a JNI expert (though thanks to updating this for Tokio support, I now know way more than I did before. Or ever wanted to.), so while I'm happy to try and take PRs and fix bugs, I can't promise too much.

## Building

The crate and the Java support library can be built together or separately.

### Simple way

The crate includes a feature to automatically build the Java support library:

```console
$ cargo build --features=build-java-support
```

The Java support library JAR will be placed in `target/<config>/java/libs`.

### Advanced way

The crate and the Java support library can be built separately:

```console
$ cargo build
$ cd java
$ ./gradlew build
```

## Using

Your Rust crate will need to link against the `jni-utils` crate, and your Java
program will need an `implementation` dependency on the Java support library.
Add this to your `build.gradle`:

```gradle
dependencies {
    implementation 'io.github.gedgygedgy.rust:jni-utils:0.1.0'
}
```

## Using with Tokio

This library should work with the Tokio async runtime without changes. However, when adding any new
features to this crate, they will need to be included in the class cache, as creating new threads in
tokio seemed to have an issue with class caches not being updated.
