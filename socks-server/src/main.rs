use fast_socks5::server::Socks5Server;

fn main() {
    let future = async { Socks5Server::bind(("localhost", 8080)).await.unwrap() };
    let socks_server = futures::executor::block_on(future);
    loop {
        let packet =async { socks_server.incoming().await};
    }
}
