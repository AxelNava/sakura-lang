use lsp_server::{Connection, Message, Request, RequestId};
use lsp_types::{InitializeParams, ClientCapabilities, TextDocumentClientCapabilities};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Command, ChildStdin, ChildStdout};
use std::process::Stdio;

pub async fn _run_client() {
    // Ruta del servidor LSP compilado
    let server_path = "D:/lsp/lsp.exe";
    println!("Server path: {}", server_path);

    // Iniciar el servidor LSP
    let mut server = Command::new(server_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start server");

    let server_stdout: ChildStdout = server.stdout.take().unwrap();
    let server_stdin: ChildStdin = server.stdin.take().unwrap();

    let mut reader = BufReader::new(server_stdout).lines();
    let mut _writer = tokio::io::BufWriter::new(server_stdin);

    // Conectarse al servidor LSP
    let (connection, _io_threads) = Connection::stdio();

    // Enviar solicitud de inicialización
    let initialize_params = InitializeParams {
        capabilities: ClientCapabilities {
            text_document: Some(TextDocumentClientCapabilities {
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let initialize_id = RequestId::from(1);
    let initialize_request = Request {
        id: initialize_id.clone(),
        method: "initialize".to_string(),
        params: serde_json::to_value(initialize_params).unwrap(),
    };
    connection.sender.send(Message::Request(initialize_request)).unwrap();

    // Leer respuesta de inicialización
    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("capabilities") {
            println!("Initialize response: {}", line);
            break;
        }
    }

    // Enviar solicitud personalizada example/echo
    let echo_id = RequestId::from(2);
    let echo_request = Request {
        id: echo_id.clone(),
        method: "example/echo".to_string(),
        params: json!({ "message": "Hello, LSP!" }),
    };
    connection.sender.send(Message::Request(echo_request)).unwrap();

    // Leer respuesta a example/echo
    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("Hello, LSP!") {
            println!("Echo response: {}", line);
            break;
        }
    }

    // Esperar a que el servidor termine
    server.wait().await.unwrap();
}
