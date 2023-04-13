```
.
├── README.md
└── rust_txt_project
    ├── log_txt_c
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── cbindgen.toml
    │   ├── README.md
    │   └── src
    │       ├── build.rs
    │       └── lib.rs
    ├── log_txt_py
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── pyproject.toml
    │   ├── README.md
    │   └── src
    │       └── lib.rs
    └── rust_txt_core
        ├── Cargo.lock
        ├── Cargo.toml
        └── src
            └── lib.rs
```

The idea of this repo is to agregate the concepts and tools to develop a multiplataform drivers writen in rust.

For a starting proof of concept, we have a rust lib "rust_txt_core" that edit and read a txt file.

Then this crate is imported to the following binding projects, for C and Python.

https://dev.to/apollolabsbin/rust-ffi-and-cbindgen-integrating-embedded-rust-code-in-c-59f8
https://michael-f-bryan.github.io/rust-ffi-guide/overview.html
https://developers.redhat.com/articles/2022/09/01/3-essentials-writing-linux-system-library-rust
https://github.com/cathay4t/librabc

#Bindings

### C:

https://github.com/eqrion/cbindgen/

### Python:

https://github.com/PyO3/pyo3

### wasm

What is FFI

Starting Rust
https://apollolabsblog.hashnode.dev/58-rust-resources-every-learner-should-know-in-2023
https://www.youtube.com/watch?v=MsocPEZBd-M&t=1820s

https://cxx.rs/

https://github.com/PyO3/maturin

https://ultrasaurus.com/2020/01/writing-c-library-in-rust/
https://github.com/ultrasaurus/rust-clib

https://doc.rust-lang.org/nomicon/ffi.html
https://karroffel.gitlab.io/post/2019-05-15-rust/
https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/
https://www.joshmatthews.net/blog/2015/10/creating-a-c-api-for-a-rust-library/
https://www.reddit.com/r/rust/comments/6u7y3q/rust_wrappers_and_c_callbacks/
https://users.rust-lang.org/t/stubbing-extern-c-functions-for-tests/15976/2

https://svartalf.info/posts/2019-03-01-exposing-ffi-from-the-rust-library/

https://doc.rust-lang.org/cargo/reference/build-scripts.html
https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

https://developers.redhat.com/articles/2022/07/15/3-essentials-writing-linux-system-library-rust
https://developers.redhat.com/articles/2022/09/05/how-create-c-binding-rust-library#elements_of_a_c_binding
https://developers.redhat.com/articles/2022/08/11/how-create-python-binding-rust-library
https://developers.redhat.com/articles/2022/08/21/build-trust-continuous-integration-your-rust-library

https://github.com/ctypesgen/ctypesgen

https://slint-ui.com/blog/rust-and-cpp.html

to check:
https://github.com/robamu-org/rpi-rs-crosscompile
https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/
https://developer.arm.com/downloads/-/gnu-a
