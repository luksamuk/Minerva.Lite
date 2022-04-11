// client.rs -- Uma parte de Minerva.Lite
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

//! Este aplicativo executa testes de stress no servidor,
//! abrindo múltiplas conexões remotas e utilizando os CRUDs para
//! avaliar a integração do programa.

use dotenv::dotenv;
use futures::StreamExt;
use rand::seq::SliceRandom;
use std::env;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;

use minerva_lite::minerva_client::MinervaClient;
use minerva_lite::*;

type ErrorImpl = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), ErrorImpl> {
    println!("Minerva.Lite gRPC v0.1.0 -- Test Client");

    dotenv().ok();
    let port = env::var("GRPC_PORT").unwrap();
    let str_addr = format!("http://127.0.0.1:{}", port);
    println!("Endereço do servidor: {}.", str_addr);

    // Sorteia um número de 15 a 50 como número de
    // testes simultâneos.
    let num = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(15..50)
    };

    println!("# Executando {} testes simultâneos...", num);

    debrief_wait();

    // Spawnar uma task por teste
    let mut tasks = vec![];
    for t in 0..num {
        let addr = str_addr.clone();
        tasks.push(tokio::spawn(async move {
            let _ = run_common_tests(t, addr).await;
        }));
    }

    // Aguardar o encerramento de todas as tasks
    for task in tasks {
        let _ = task.await;
    }

    println!("Teste finalizado.");

    Ok(())
}

/// Espera 3 segundos antes de continuar.
fn debrief_wait() {
    use std::thread;
    use std::time::Duration;
    let _ = thread::sleep(Duration::from_millis(3000));
}

/// Executa uma leva de testes.
async fn run_common_tests(t: u32, addr: String) -> Result<(), ErrorImpl> {
    let addr = Endpoint::from_shared(addr.clone())?;
    let mut client = MinervaClient::connect(addr).await?;

    // Ping
    let _ = client.ping(Request::new(())).await?;

    let cadastrados = teste_cadastro(t, &mut client).await?;
    teste_consulta(t, &mut client, &cadastrados).await?;
    teste_lista(t, &mut client).await?;
    teste_remocao(t, &mut client, cadastrados).await?;

    println!("*** T{}: Finalizado ***", t);

    Ok(())
}

/// Imprime os dados de um único cliente.
fn imprime_cliente(res: &ClienteResponse) {
    println!(
        "ID: {}\n\
         Tipo: {}\n\
         Nome: {}\n\
         Pessoa jurídica? {}\n\
         Documento: {}\n",
        res.id, res.tipo, res.nome, res.pj, res.docto
    );
}

/// Gera um vetor de clientes de teste para serem cadastrados.
fn gera_clientes() -> Vec<NovoClienteRequest> {
    vec![
        NovoClienteRequest {
            nome: "Beltrano de Souza".to_string(),
            pj: false,
            docto: "999.999.999-99".to_string(),
        },
        NovoClienteRequest {
            nome: "Fulano de Tal".to_string(),
            pj: false,
            docto: "888.888.888-88".to_string(),
        },
        NovoClienteRequest {
            nome: "Empresa S/A".to_string(),
            pj: true,
            docto: "99.999.999/9999-99".to_string(),
        },
        NovoClienteRequest {
            nome: "Ciclano da Silva".to_string(),
            pj: false,
            docto: "777.777.777-77".to_string(),
        },
        NovoClienteRequest {
            nome: "Outra Empresa LTDA".to_string(),
            pj: true,
            docto: "99.999.999/9999-99".to_string(),
        },
    ]
}

/// Testa a conexão, cadastrando os clientes de teste.
async fn teste_cadastro(
    t: u32,
    client: &mut MinervaClient<Channel>,
) -> Result<Vec<i32>, ErrorImpl> {
    let num = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(1..50)
    };

    let clientes = gera_clientes();

    println!(
        "## T{}: Cadastrando {} clientes...",
        t,
        num * clientes.len()
    );

    let mut cadastrados = vec![];

    for _ in 0..num {
        for c in &clientes {
            let request = Request::new(c.clone());
            let result = client.cadastra_cliente(request).await?;
            let response = result.into_inner();
            println!("   T{}: Cliente cadastrado com ID {}", t, response.id);
            cadastrados.push(response.id);
        }
    }

    println!();
    Ok(cadastrados)
}

/// Testa a conexão, procurando por um cliente através do ID.
async fn teste_consulta(
    t: u32,
    client: &mut MinervaClient<Channel>,
    cadastrados: &Vec<i32>,
) -> Result<(), ErrorImpl> {
    let num = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(1..cadastrados.len()).try_into().unwrap()
    };
    println!("## T{}: Mostrando dados de {} clientes...", t, num);

    for _ in 0..num {
        let id = cadastrados.choose(&mut rand::thread_rng()).unwrap();
        let _ = client
            .consulta_cliente(Request::new(IdClienteRequest { id: *id }))
            .await?
            .into_inner();
        println!("   T{}: Cliente {} recuperado.", t, id);
    }

    Ok(())
}

/// Testa a conexão, removendo clientes previamente cadastrados.
async fn teste_remocao(
    t: u32,
    client: &mut MinervaClient<Channel>,
    cadastrados: Vec<i32>,
) -> Result<(), ErrorImpl> {
    println!("## T{}: Removendo clientes...", t);

    for id in cadastrados {
        client
            .deleta_cliente(Request::new(IdClienteRequest { id: id }))
            .await?;
        println!("   T{}: Removido: Usuário #{}", t, id);
    }

    Ok(())
}

/// Lista todos os clientes cadastrados usando um stream.
async fn teste_lista(t: u32, client: &mut MinervaClient<Channel>) -> Result<(), ErrorImpl> {
    println!("## T{}: Mostrando dados de clientes via streaming...", t);

    let mut stream = client.lista_clientes(Request::new(())).await?.into_inner();

    let mut num_paginas = 0;
    let mut num_clientes = 0;

    while let Some(response) = stream.next().await {
        match response {
            Ok(page_response) => {
                num_paginas += 1;
                num_clientes += page_response.clientes.len();
                println!("## Página {}:", num_paginas);

                for c in page_response.clientes {
                    imprime_cliente(&c);
                }
            }
            Err(_) => {
                // Conexão encerrada de forma inesperada.
                break;
            }
        }
    }

    println!(
        "## T{}: Conexão encerrada pelo servidor remoto. \
         Páginas: {} \n\
         Clientes: {}",
        t, num_paginas, num_clientes,
    );

    Ok(())
}
