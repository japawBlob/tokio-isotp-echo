use tokio_socketcan_isotp::{IsoTpSocket, StandardId, Error};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let mut rx = IsoTpSocket::open(
        "vcan0",
        StandardId::new(0x123).expect("Invalid src id"),
        StandardId::new(0x321).expect("Invalid src id")
            )?;

            
    let _ = rx.write_packet(vec![1,2,3])?.await;

    println!("1 2 3 sent");

    while let Ok(mut packet) = rx.read_packet()?.await {
        println!("{:?}", packet);
        let blob = rx.write_packet(packet)?.await;
    }
    Ok(())
}