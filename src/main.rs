
use modbus::{Client, Coil};
use modbus::tcp;

fn main() {
    let mut cfg = tcp::Config::default();
    let mut client = tcp::Transport::new_with_cfg("127.0.0.1", cfg).unwrap();

    client.write_single_coil(1, Coil::On).unwrap();
    client.write_single_coil(3, Coil::On).unwrap();
    client.write_single_register(0, 23).unwrap();

    let res = client.read_coils(0, 5).unwrap();
    let resReg = client.read_holding_registers(0, 5).unwrap();

    println!("{:?}",res);
    println!("{:?}",resReg);
}
