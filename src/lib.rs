// Copyright 2016-2017 The Perceptia Project Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This crate supplements `skylane` crate with bindings `Wayland` protocol automatically
//! generated from XML protocol description files.
//!
//! This crate is planed to provide implementation for all (stable and unstable) protocols. If
//! something is missing, let us know.
//!
//! ## Implementation
//!
//! Each protocol description file contains requests (from client to server) and events (from
//! server to client). In server part requests are translated to:
//! - interfaces - traits describing methods of a Wayland protocol object
//! - dispatchers - structures translating socket data to calls to methods on objects implementing
//!   appropriate interface.
//! while events to stand-alone functions. On client side vice-versa.
//!
//! `Handler` structure helps bind `Dispatcher` with implementation of its `Interface` to
//! register it in `server::Connection` from `skylane` crate.
//!
//! ## Server examples
//!
//! TODO: Add examples for server.
//!
//! ## Client examples
//!
//! TODO: Add examples for client.

// TODO: Add comments to protocols.
// #![warn(missing_docs)]

extern crate skylane;
extern crate byteorder;

// -------------------------------------------------------------------------------------------------

mod private {
    use std::io::Cursor;
    use skylane::server::{Bundle, Header, Object, SkylaneError, Task};

    /// This trait is implemented by `Dispatcher`s in protocol definitions generated from XML
    /// files. Every object defined in protocol has its own `Dispatcher` which takes buffer
    /// data, parses it and calls appropriate method of given object which implements attached
    /// interface.
    pub trait Dispatcher<I> {
        fn new() -> Self;
        fn dispatch(&mut self,
                    object: &mut I,
                    bundle: &mut Bundle,
                    header: &Header,
                    bytes_buf: &mut Cursor<&[u8]>,
                    fds_buf: &mut Cursor<&[u8]>)
                    -> Result<Task, SkylaneError>;
    }

    /// Binds `Dispatcher` with object implementing `Interface` trait from protocol definition.
    /// Only objects implementing `Interfaces` corresponding to `Dispatcher` can be bound
    /// together.
    pub struct Handler<I, D>
        where D: Dispatcher<I>
    {
        object: I,
        dispatcher: D,
    }

    impl<I, D> Handler<I, D>
        where D: Dispatcher<I>
    {
        pub fn new(object: I) -> Self {
            Handler {
                object: object,
                dispatcher: D::new(),
            }
        }
    }

    impl<I, D> Object for Handler<I, D>
        where D: Dispatcher<I>
    {
        fn dispatch(&mut self,
                    bundle: &mut Bundle,
                    header: &Header,
                    bytes_buf: &mut Cursor<&[u8]>,
                    fds_buf: &mut Cursor<&[u8]>)
                    -> Result<Task, SkylaneError> {
            self.dispatcher.dispatch(&mut self.object, bundle, header, bytes_buf, fds_buf)
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub mod server {
    use skylane::server::{Bundle, Header, ObjectId, SkylaneError, Socket, Task};
    use private::Dispatcher;
    pub use private::Handler;

    pub mod wayland {
        include!(concat!(env!("OUT_DIR"), "/wayland_server.rs"));
    }
    pub mod xdg_shell_unstable_v6 {
        include!(concat!(env!("OUT_DIR"), "/xdg_shell_unstable_v6_server.rs"));
    }
    pub mod weston_screenshooter {
        include!(concat!(env!("OUT_DIR"), "/weston_screenshooter_server.rs"));
    }
}

// -------------------------------------------------------------------------------------------------

pub mod client {
    use skylane::server::{Bundle, Header, ObjectId, SkylaneError, Socket, Task};
    use private::Dispatcher;
    pub use private::Handler;

    pub mod wayland {
        include!(concat!(env!("OUT_DIR"), "/wayland_client.rs"));
    }
    pub mod xdg_shell_unstable_v6 {
        include!(concat!(env!("OUT_DIR"), "/xdg_shell_unstable_v6_client.rs"));
    }
    pub mod weston_screenshooter {
        include!(concat!(env!("OUT_DIR"), "/weston_screenshooter_client.rs"));
    }
}

// -------------------------------------------------------------------------------------------------
