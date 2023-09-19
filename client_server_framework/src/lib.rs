use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
// use bytes::{BufMut, BytesMut};
use bytes::BytesMut;
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client { stream })
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<BytesMut, std::io::Error> {
        let mut buf = BytesMut::new();
        self.stream.read_buf(&mut buf).await?;
        Ok(buf)
    }
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { listener })
    }

    pub async fn accept(&mut self) -> Result<Client, std::io::Error> {
        let (stream, _) = self.listener.accept().await?;
        Ok(Client { stream })
    }
}