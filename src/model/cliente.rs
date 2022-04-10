// model/cliente.rs -- Uma parte de Minerva.rs, adaptado para o Minerva.Lite
// Copyright (C) 2021-2022 Lucas S. Vieira
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

//! Utilitários de modelagem de usuário do sistema para banco de dados e
//! regras de negócio.
//!
//! Este módulo define estruturas para o tráfego de dados de usuários do sistema
//! entre as partes respectivas do mesmo. Estes usuários serão as entidades que
//! efetuam operações no sistema.
//!
//! # Lista de modificações com relação ao Minerva.rs
//!
//! - Modificação da documentação para se adaptar ao Minerva.Lite;
//! - Remoção da estrutura `UsuarioRecv` e de seu bloco `impl`;
//! - Adição de traits para conversão de `Cliente` para `ClienteResponse`;
//! - Adição de traits para conversão de `NovoClienteRequest` para `NovoCliente`.

use crate::model::schema::cliente;
use crate::{ClienteResponse, NovoClienteRequest};

/// Representa a estrutura de um elemento da tabela `cliente` do banco de dados.
#[derive(Queryable, Clone)]
pub struct Cliente {
    /// Id do cliente no banco.
    pub id: i32,
    /// Tipo do cliente. Definido como 0 por padrão.
    pub tipo: i16,
    /// Nome do cliente.
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Caso seja, assume valor
    /// verdadeiro. Caso seja pessoa física, assume valor falso.
    pub pj: bool,
    /// Documento do cliente. Caso seja pessoa física, será seu CPF com onze
    /// dígitos, pontos e hífens. Caso seja pessoa jurídica, será seu CNPJ
    /// com catorze dígitos, pontos, hífens e barras.
    pub docto: String,
    /// Determina se o cliente está ativo. Um cliente pode ser definido como
    /// inativo se sua remoção não for conveniente.
    pub ativo: bool,
    /// Determina se o cliente está bloqueado. Um cliente bloqueado não poderá
    /// ter operações feitas em seu nome.
    pub bloqueado: bool,
}

impl From<Cliente> for ClienteResponse {
    fn from(cliente: Cliente) -> ClienteResponse {
        ClienteResponse {
            id: cliente.id,
            tipo: cliente.tipo as i32,
            nome: cliente.nome,
            pj: cliente.pj,
            docto: cliente.docto,
            ativo: cliente.ativo,
            bloqueado: cliente.bloqueado,
        }
    }
}

/// Representa os dados de um cliente a serem inseridos na criação de um novo
/// cliente no banco de dados.
#[derive(Insertable, Default)]
#[table_name = "cliente"]
pub struct NovoCliente {
    /// Tipo do cliente. Ver [`Cliente::id`].
    pub tipo: i16,
    /// Nome do cliente. Ver [`Cliente::nome`].
    pub nome: String,
    /// Determina se o cliente é uma pessoa jurídica. Ver [`Cliente::pj`].
    pub pj: bool,
    /// Documento do cliente. Ver [`Cliente::docto`].
    pub docto: String,
    /// Determina se o cliente está ativo. Ver [`Cliente::ativo`].
    pub ativo: bool,
    /// Determina se o cliente está bloqueado. Ver [`Cliente::bloqueado`].
    pub bloqueado: bool,
}

impl From<NovoClienteRequest> for NovoCliente {
    fn from(req: NovoClienteRequest) -> NovoCliente {
        Self {
            tipo: 0,
            nome: req.nome,
            pj: req.pj,
            docto: req.docto,
            ativo: true,
            bloqueado: false,
        }
    }
}
