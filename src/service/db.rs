// service/db.rs -- Uma parte de Minerva.Lite
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

//! Este módulo implementa utilitários diretamente atrelados à conexão com o
//! banco de dados, mais especificamente com a geração da pool de conexões
//! para possibilitar conexões assíncronas ao banco.

use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use diesel::PgConnection;
use std::env;

/// Representação de um tipo de pool de conexões com o banco de dados.
pub type ConnectionPool = Pool<DieselConnectionManager<PgConnection>>;

/// Quantidadde máxima de conexões com o banco de dados abertas para
/// uso na pool assíncrona de conexões com o mesmo.
pub const MAX_DATABASE_CONNECTIONS: u32 = 15;

/// Cria uma pool com no máximo `MAX_DATABASE_CONNECTIONS` conexões
/// disponíveis com o banco de dados.
///
/// Depende da variável de ambiente `DATABASE_URL` para realizar conexão.
pub async fn make_connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não foi definido");
    let manager = DieselConnectionManager::<PgConnection>::new(&database_url);

    Pool::builder()
        .max_size(MAX_DATABASE_CONNECTIONS)
        .build(manager)
        .await
        .expect("Impossível criar pool de conexões com o banco de dados")
}
