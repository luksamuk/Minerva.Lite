// service/clientes.rs -- Uma parte de Minerva.Lite
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

//! Este módulo implementa o serviço gRPC do CRUD de Clientes do Minerva.Lite.
//! Este CRUD envolve protocolos para criação, remoção, consulta, listagem e
//! atualização de usuários.

use super::{db, utils};
use futures::Stream;
use minerva_lite::minerva_clientes_server::{MinervaClientes, MinervaClientesServer};
use minerva_lite::*;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use minerva_lite::controller::cliente as controller;

const DB_ERR_MSG: &str = "Impossível conectar ao banco de dados";

/// Estrutura do serviço de clientes do MinervaLite.
/// A estrutura possui apenas um pool de conexões ao PostgreSQL.
pub struct MinervaLiteClientesService {
    pool: db::ConnectionPool,
}

#[tonic::async_trait]
impl MinervaClientes for MinervaLiteClientesService {
    /// Tipo para o stream das páginas de cliente, que serão enviadas.
    ///
    /// Trata-se de um elemento boxed, atrelado a um endereço fixo de memória,
    /// que implementa o trait Stream e o trait Send. Cada item enviado pelo
    /// Stream poderá ser uma resposta contendo as páginas de clientes, ou
    /// um status próprio para erros do gRPC.
    type ListaStream = Pin<Box<dyn Stream<Item = Result<ClientePageResponse, Status>> + Send>>;

    /// Resposta à requisição de cadastro do cliente.
    async fn cadastra(
        &self,
        req: Request<NovoClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        utils::log(utils::get_address(&req), "Clientes::Cadastra");

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
    async fn consulta(
        &self,
        req: Request<IdClienteRequest>,
    ) -> Result<Response<ClienteResponse>, Status> {
        let id = req.get_ref().id;
        utils::log(
            utils::get_address(&req),
            &format!("Clientes::Consulta (ID = {})", id),
        );

        let conn = self
            .pool
            .get()
            .await
            .map_err(|_| Status::internal(DB_ERR_MSG))?;

        controller::consulta(&conn, id)
            .map_err(|_| Status::not_found("Usuário não encontrado"))
            .map(|result| Response::new(result.into()))
    }

    /// Retorna um stream por onde será enviada a lista de todos os
    /// clientes cadastrados.
    async fn lista(&self, req: Request<()>) -> Result<Response<Self::ListaStream>, Status> {
        let destino = utils::get_address(&req);
        utils::log(destino, "Clientes::Lista (Stream)");

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
                utils::log(destino, &format!("Clientes::Lista (Pág {})", page_number));
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
        Ok(Response::new(Box::pin(output_stream) as Self::ListaStream))
    }

    /// Resposta à requisição de remoção de um cliente.
    async fn deleta(&self, req: Request<IdClienteRequest>) -> Result<Response<()>, Status> {
        let id = req.get_ref().id;
        utils::log(
            utils::get_address(&req),
            &format!("Clientes::Deleta (ID = {})", id),
        );

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

/// Cria um serviço de clientes Minerva.Lite.
/// Este serviço deverá ser atrelado ao servidor gRPC no ponto de entrada
/// da aplicação.
pub async fn make_service() -> MinervaClientesServer<MinervaLiteClientesService> {
    MinervaClientesServer::new(MinervaLiteClientesService {
        pool: db::make_connection_pool().await,
    })
}
