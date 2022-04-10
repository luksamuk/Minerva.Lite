// service.rs -- Uma parte de Minerva.Lite
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

//! Este módulo implementa o serviço gRPC do Minerva.Lite.
//! Este serviço atualmente é constituído apenas de um CRUD para
//! clientes, e não possui autenticação.

use chrono;
use minerva_lite::minerva_server::{Minerva, MinervaServer};
use minerva_lite::*;
use tonic::{Request, Response, Status};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

use minerva_lite::controller::cliente as controller;

const DB_ERR_MSG: &str = "Impossível conectar ao banco de dados";

/// Estrutura básica do serviço MinervaLite.
/// A estrutura básica possui apenas um pool de conexões ao
/// PostgreSQL.
pub struct MinervaLiteService {
    pool: ConnectionPool,
}

/// Função auxiliar de log.
fn log(msg: &str) {
    println!("{} :: {}", chrono::offset::Utc::now(), msg);
}

#[tonic::async_trait]
impl Minerva for MinervaLiteService {
    /// Resposta à requisição de ping.
    async fn ping(&self, _: Request<()>) -> Result<Response<()>, Status> {
        log("Ping(Empty) -> (Empty)");
        Ok(Response::new(()))
    }

    /// Resposta à requisição de cadastro do cliente.
    async fn cadastra_cliente(
        &self,
        req: Request<NovoClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        log("CadastraCliente(NovoCliente) -> (Cliente)");

        let conn = {
            let pool = self.pool.clone();
            pool.get().map_err(|_| Status::internal(DB_ERR_MSG))?
        };

        controller::cadastra(&conn, req.into_inner().into())
            .map_err(|_| Status::invalid_argument("Usuário não cadastrado"))
            .map(|result| Response::new(result.into()))
    }

    /// Resposta à requisição de consulta de um único cliente.
    async fn consulta_cliente(
        &self,
        req: Request<IdClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        let id = req.into_inner().id;
        log(&format!(
            "ConsultaCliente(IdCliente) -> (Cliente) :: ID = {}",
            id
        ));

        let conn = {
            let pool = self.pool.clone();
            pool.get().map_err(|_| Status::internal(DB_ERR_MSG))?
        };

        controller::consulta(&conn, id)
            .map_err(|_| Status::not_found("Usuário não encontrado"))
            .map(|result| Response::new(result.into()))
    }

    /// Resposta à requisição de remoção de um cliente.
    async fn deleta_cliente(&self, req: Request<IdClienteRequest>) -> Result<Response<()>, Status> {
        let id = req.into_inner().id;
        log(&format!(
            "DeletaCliente(IdCliente) -> (Empty) :: ID = {}",
            id
        ));

        let conn = {
            let pool = self.pool.clone();
            pool.get().map_err(|_| Status::internal(DB_ERR_MSG))?
        };

        controller::remove(&conn, id)
            .map_err(|_| Status::not_found("Usuário não encontrado"))
            .map(|_| Response::new(()))
    }
}

/// Cria uma pool com no máximo 15 conexões disponíveis com o PostgreSQL.
/// Depende da variável de ambiente `DATABASE_URL`.
fn connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não foi definido");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Impossível criar pool de conexões com PostgreSQL")
}

/// Cria um serviço do Minerva.Lite. Este serviço deverá ser atrelado
/// ao servidor gRPC no ponto de entrada da aplicação.
pub fn make_service() -> MinervaServer<MinervaLiteService> {
    MinervaServer::new(MinervaLiteService {
        pool: connection_pool(),
    })
}
