// service/base.rs -- Uma parte de Minerva.Lite
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

//! Este módulo implementa o serviço gRPC básico do Minerva.Lite, que responde
//! apenas a informações muito básicas e que não devem atrapalhar as regras
//! de negócios.

use super::utils;
use minerva_lite::minerva_server::{Minerva, MinervaServer};
use tonic::{Request, Response, Status};

/// Estrutura básica do serviço MinervaLite.
#[derive(Clone, Default)]
pub struct MinervaLiteService;

#[tonic::async_trait]
impl Minerva for MinervaLiteService {
    /// Resposta à requisição de ping.
    async fn ping(&self, req: Request<()>) -> Result<Response<()>, Status> {
        utils::log(utils::get_address(&req), "Ping(Empty) -> (Empty)");
        Ok(Response::new(()))
    }
}

/// Cria um serviço base do Minerva.Lite. Este serviço deverá ser atrelado
/// ao servidor gRPC no ponto de entrada da aplicação.
pub async fn make_service() -> MinervaServer<MinervaLiteService> {
    MinervaServer::new(MinervaLiteService::default())
}
