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
use futures::Stream;
use minerva_lite::minerva_server::{Minerva, MinervaServer};
use minerva_lite::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use diesel::PgConnection;
use std::env;

pub type ConnectionPool = Pool<DieselConnectionManager<PgConnection>>;

use minerva_lite::controller::cliente as controller;

const DB_ERR_MSG: &str = "Impossível conectar ao banco de dados";

/// Estrutura básica do serviço MinervaLite.
/// A estrutura básica possui apenas um pool de conexões ao
/// PostgreSQL.
pub struct MinervaLiteService {
    pool: ConnectionPool,
}

/// Função auxiliar de log assíncrono.
/// A função cria uma task que imprimirá o log de quando a requisição foi
/// feita, mas apenas quando isso for possível. Feito dessa forma para evitar
/// gargalos em respostas a requisições.
fn log(msg: &str) {
    let msg = msg.to_string();
    let time = chrono::offset::Local::now();
    tokio::spawn(async move {
        println!("{} :: {}", time, msg);
    });
}

/// Recupera o endereço remoto a partir de uma requisição.
fn get_address<T>(req: &Request<T>) -> SocketAddr {
    req.remote_addr()
        .unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
}

#[tonic::async_trait]
impl Minerva for MinervaLiteService {
    /// Resposta à requisição de ping.
    async fn ping(&self, req: Request<()>) -> Result<Response<()>, Status> {
        log(&format!("Ping(Empty) -> (Empty) @ {:?}", get_address(&req)));
        Ok(Response::new(()))
    }

    /// Resposta à requisição de cadastro do cliente.
    async fn cadastra_cliente(
        &self,
        req: Request<NovoClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        log(&format!(
            "CadastraCliente(NovoCliente) -> (Cliente) @ {:?}",
            get_address(&req)
        ));

        let conn = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal(DB_ERR_MSG))?;

        controller::cadastra(&conn, req.into_inner().into())
            .map_err(|_| Status::invalid_argument("Usuário não cadastrado"))
            .map(|result| Response::new(result.into()))
    }

    /// Resposta à requisição de consulta de um único cliente.
    async fn consulta_cliente(
        &self,
        req: Request<IdClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        let id = req.get_ref().id;
        log(&format!(
            "ConsultaCliente(IdCliente) -> (Cliente) :: ID = {} @ {:?}",
            id,
            get_address(&req)
        ));

        let conn = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal(DB_ERR_MSG))?;

        controller::consulta(&conn, id)
            .map_err(|_| Status::not_found("Usuário não encontrado"))
            .map(|result| Response::new(result.into()))
    }

    // REQUERIDO: Tipo para o stream das páginas de cliente, que serão enviadas.
    //
    // Trata-se de um elemento boxed, atrelado a um endereço fixo de memória,
    // que implementa o trait Stream e o trait Send. Cada item enviado pelo
    // Stream poderá ser uma resposta contendo as páginas de clientes, ou
    // um status próprio para erros do gRPC.
    type ListaClientesStream =
        Pin<Box<dyn Stream<Item = Result<ClientePageResponse, Status>> + Send>>;

    /// Retorna um stream por onde será enviada a lista de todos os
    /// clientes cadastrados.
    async fn lista_clientes(
        &self,
        req: Request<()>,
    ) -> Result<Response<Self::ListaClientesStream>, Status> {
        log(&format!(
            "ListaClientes(Empty) -> (stream ClientePage) @ {:?}",
            get_address(&req)
        ));

        let pool = self.pool.clone();

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut page_number = 0;
            loop {
                let conn = match pool.get().await {
                    Ok(c) => c,
                    Err(_) => {
                        // Impossível recuperar conexão com o BD
                        break;
                    }
                };

                let page: Vec<ClienteResponse> = match controller::lista(&conn, page_number) {
                    Ok(page) => page.iter().map(|c| c.clone().into()).collect(),
                    Err(_) => {
                        // Impossível recuperar página de usuários
                        break;
                    }
                };

                if page.is_empty() {
                    // Nada a ser enviado
                    break;
                }

                let response = ClientePageResponse { clientes: page };
                match tx.send(Result::<_, Status>::Ok(response)).await {
                    Ok(_) => {
                        // Página enfileirada; ir para a próxima
                        page_number += 1;
                    }
                    Err(_) => {
                        // Stream de saída foi encerrado
                        break;
                    }
                }
            }
        });

        // Retorna o stream em si
        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ListaClientesStream
        ))
    }

    /// Resposta à requisição de remoção de um cliente.
    async fn deleta_cliente(&self, req: Request<IdClienteRequest>) -> Result<Response<()>, Status> {
        let id = req.get_ref().id;
        log(&format!(
            "DeletaCliente(IdCliente) -> (Empty) :: ID = {} @ {:?}",
            id,
            get_address(&req)
        ));

        let conn = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal(DB_ERR_MSG))?;

        controller::remove(&conn, id)
            .map_err(|_| Status::not_found("Usuário não encontrado"))
            .map(|_| Response::new(()))
    }
}

/// Cria uma pool com no máximo 15 conexões disponíveis com o PostgreSQL.
/// Depende da variável de ambiente `DATABASE_URL`.
async fn connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não foi definido");
    let manager = DieselConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .expect("Impossível criar pool de conexões com PostgreSQL")
}

/// Cria um serviço do Minerva.Lite. Este serviço deverá ser atrelado
/// ao servidor gRPC no ponto de entrada da aplicação.
pub async fn make_service() -> MinervaServer<MinervaLiteService> {
    MinervaServer::new(MinervaLiteService {
        pool: connection_pool().await,
    })
}
