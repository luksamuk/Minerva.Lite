// model.rs -- Uma parte de Minerva.Lite
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

//! Este módulo engloba os models das entidades e também os schemas de banco de
//! dados para tais estruturas.
//!
//! O schema é gerado automaticamente pela biblioteca Diesel, sendo diretamente
//! copiado, para maior facilidade, do programa Minerva.rs. Assim, este arquivo
//! será melhor aproveitado caso o banco seja criado, primeiramente, a partir do
//! projeto Minerva.rs.

pub mod cliente;
pub mod schema;
