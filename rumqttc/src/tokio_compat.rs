pub mod runtime {
    use std::future::Future;

    use smol::block_on;

    pub struct Builder {}
    impl Builder {
        pub fn new_current_thread() -> Self {
            Builder {}
        }
        pub fn enable_all(&mut self) -> &mut Self {
            self
        }
        pub fn build(&mut self) -> std::io::Result<Runtime> {
            Ok(Runtime {})
        }
    }
    pub struct Runtime {}
    impl Runtime {
        pub fn block_on<F: Future>(&self, future: F) -> F::Output {
            block_on(future)
        }
    }
}

pub mod task {
    pub use smol::spawn;
}

pub mod tls {
    #[cfg(feature = "tls")]
    pub use rustls;
}

pub mod net {
    pub use smol::net::unix::UnixStream;
    pub use smol::net::TcpStream;
}

pub mod io {
    pub use smol::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

    use bytes::BufMut;

    pub trait AsyncReadExt: AsyncRead {
        fn read_buf<'a, B>(
            &'a mut self,
            buf: &'a mut B,
        ) -> crate::tokio_read_buf::ReadBuf<'a, Self, B>
        where
            Self: Sized + Unpin,
            B: BufMut,
        {
            crate::tokio_read_buf::read_buf(self, buf)
        }
    }

    impl<R: AsyncRead + ?Sized> AsyncReadExt for R {}
}
