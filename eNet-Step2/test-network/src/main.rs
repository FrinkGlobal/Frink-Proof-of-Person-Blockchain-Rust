extern crate enet;
use enet::*;

use node_network;
use node_network::Node;

fn main() {

    let mut enet = Enet::new().expect("could not initialize ENet");

    let mut node: Node = Node::new(8590);

    node.init();
    node.start_service(&mut enet);
    
}
