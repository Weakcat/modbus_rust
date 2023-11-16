// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Asynchronous TCP client example

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::*;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Vec<bool>>(32);

    let handle = tokio::spawn(async move {
        let socket_addr = "192.168.1.188:502".parse().unwrap();
        let mut ctx = tcp::connect_slave(socket_addr, Slave(1)).await.unwrap();

        loop {
            let coil_value = rx.recv().await.unwrap();
            match ctx.write_multiple_coils(0x0000, &coil_value).await {
                Ok(()) => {}
                Err(err) => eprintln!("write_multiple_coils Error:{}", err),
            }

            sleep(Duration::from_millis(10)).await;

            let coil_value = [false, false, false, false, false, false, false, false];
            match ctx.write_multiple_coils(0x0000, &coil_value).await {
                Ok(()) => {}
                Err(err) => eprintln!("write_multiple_coils Error:{}", err),
            };
        }
    });

    let mut coil_vec: Vec<bool> = vec![true, false, false, false, false, false, false, false];
    loop {
        handle.is_finished();
        coil_vec.rotate_left(1);
        tx.send(coil_vec.clone()).await.unwrap();
        sleep(Duration::from_secs(10)).await;
    }
}
