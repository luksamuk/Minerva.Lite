// service/utils.rs -- Uma parte de Minerva.Lite
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

//! Utilitários variados para uso em serviços.

use chrono;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::Request;

/// Função auxiliar de log assíncrono.
/// A função cria uma task que imprimirá o log de quando a requisição foi
/// feita, mas apenas quando isso for possível. Feito dessa forma para evitar
/// gargalos em respostas a requisições.
pub fn log(addr: SocketAddr, msg: &str) {
    let msg = msg.to_string();
    let time = chrono::offset::Local::now();
    tokio::spawn(async move {
        println!("{} :: {:?} :: {}", time, addr, msg);
    });
}

/// Recupera o endereço remoto a partir de uma requisição.
pub fn get_address<T>(req: &Request<T>) -> SocketAddr {
    req.remote_addr()
        .unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
}
