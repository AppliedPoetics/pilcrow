#[path = "networking/tcp.rs"] mod tcp;

fn main() {
    tcp::bind();
}
