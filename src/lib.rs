// lib.rs -- Uma parte de Minerva.Lite
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

//! Esta é a biblioteca principal do Minerva.Lite.
//! Os módulos aqui presentes dizem respeito ao backend da aplicação -- mais
//! especificamente, esquemas e modelos para acessar entidades no Banco de Dados,
//! e também controllers para regras de negócio.

//! Adicionalmente, este módulo também implementa estruturas globais de acordo
//! com o padrão gRPC. Estes dados podem ser melhor interpretados através do
//! arquivo "minerva.proto".

#[macro_use]
extern crate diesel;

// Módulos extras
pub mod controller;
pub mod model;

// Inclui o arquivo minerva.proto e gera código
// relativo ao protobuf, no módulo atual
tonic::include_proto!("minerva");
