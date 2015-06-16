**NOTE: this is work in progress, and not suited for use yet**

Overview
--------

This library provides rust bindings to
[stb_image](https://github.com/nothings/stb/blob/master/stb_image.h), a
light-weight C library to load images. The original library's main benefit is
ease of linking, so there might be better libraries out there for Rust, which
are as easy to link against as this one.

Building
--------

Build by running

	cargo build

This builds both the C library and the rust library.

TODO
----

- Provide bindings for all the functions
