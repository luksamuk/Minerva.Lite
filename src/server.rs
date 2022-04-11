// server.rs -- Uma parte de Minerva.Lite
// Copyright (C) 2022 Lucas S. Vieira
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod service;

use dotenv::dotenv;
use std::env;
use tonic::transport::Server;

type ErrorImpl = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), ErrorImpl> {
    println!("Minerva.Lite gRPC v0.1.0 -- Server");

    dotenv().ok();
    let port = env::var("GRPC_PORT").expect("Impossível ler porta gRPC");
    let addr = format!("0.0.0.0:{}", port).parse()?;

    let server = Server::builder()
        .add_service(service::base::make_service().await)
        .add_service(service::clientes::make_service().await)
        .serve(addr);

    println!("Escutando em {}.", addr);
    println!("Use Ctrl+C para sair.");

    server.await?;

    Ok(())
}
