// controller/cliente.rs -- Uma parte de Minerva.Lite
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

//! Este módulo engloba as estruturas do controller do cliente.
//! O CRUD básico e a aplicação de regras de negócio do cliente poderão ser
//! encontradas aqui.

use crate::model::cliente::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

/// Número máximo de clientes mostrados em uma página de listagem.
pub const CLIENTE_PAGE_SIZE: i64 = 100;

/// Realiza o cadastro de um único cliente, de acordo com os dados básicos
/// necessários para cadastro. Requer uma conexão com o banco, e o cliente
/// recém-cadastrado será retornado, em caso de sucesso.
pub fn cadastra(conn: &PgConnection, dados: NovoCliente) -> Result<Cliente, Error> {
    diesel::insert_into(crate::model::schema::cliente::table)
        .values(&dados)
        .get_result::<Cliente>(conn)
}

/// Consulta os dados de um único cliente, através do ID requisitado.
/// Em caso de sucesso, retorna uma estrutura única contendo tais dados.
pub fn consulta(conn: &PgConnection, req_id: i32) -> Result<Cliente, Error> {
    use crate::model::schema::cliente::dsl::*;
    cliente
        .filter(id.eq(&req_id))
        .load::<Cliente>(conn)
        .map(|v| v.first().unwrap().clone())
}

/// Retorna uma lista de clientes, por ordem de ID, de acordo com a página
/// requisitada.
///
/// As páginas começam a serem contadas a partir de 0. Em caso de sucesso,
/// retorna um `Vec` contendo um número `CLIENTE_PAGE_SIZE` de clientes.
pub fn lista(conn: &PgConnection, pagina: i64) -> Result<Vec<Cliente>, Error> {
    use crate::model::schema::cliente::dsl::*;

    let offset = (pagina * CLIENTE_PAGE_SIZE) + 1;

    cliente
        .order(id)
        .limit(CLIENTE_PAGE_SIZE)
        .offset(offset)
        .load::<Cliente>(conn)
}

/// Remove um cliente, através do ID requisitado, caso o mesmo exista
/// no banco de dados.
pub fn remove(conn: &PgConnection, req_id: i32) -> Result<(), Error> {
    use crate::model::schema::cliente::dsl::*;
    diesel::delete(cliente.filter(id.eq(&req_id)))
        .execute(conn)
        .map(|_| ())
}
