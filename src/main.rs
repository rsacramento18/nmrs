use crate::networks::nmcli::{get_available_networks, get_active_network};
mod networks;

fn main() {

    get_active_network();
    println!("{:?}", get_available_networks());

}
