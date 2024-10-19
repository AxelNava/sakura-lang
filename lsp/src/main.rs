//----External Libs----
use lsp_server::{Connection, Message, Request, Response, RequestId};
use lsp_types::{InitializeResult, ServerCapabilities};
// use lsp_types::InitializeParams;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//----Testings----
mod tests;
mod benchmarks;

//----Structs----
#[derive(Serialize, Deserialize)]
struct ExampleParams {
    message: String,
}


//----Methods----
#[tokio::main]
async fn main() {
    // Establecer conexión con el cliente (editor)
    let (connection, io_threads) = Connection::stdio();

    // Capacidades del servidor
    let _server_capabilities = serde_json::to_value(ServerCapabilities::default()).unwrap();

    // Inicializar la conexión


    // Esperar el mensaje de inicialización del cliente
    let (initialize_id, _initialize_value): (RequestId, Value) = connection.initialize_start().unwrap();
    // let initialize_params: InitializeParams = serde_json::from_value(initialize_value).unwrap();

    // Responder al mensaje de inicialización
    let capabilities = ServerCapabilities::default();
    let initialize_result = InitializeResult {
        capabilities,
        ..Default::default()
    };
    let result_value = serde_json::to_value(initialize_result).unwrap();
    connection.initialize_finish(initialize_id, result_value).unwrap();

    // Procesar los mensajes recibidos del cliente
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    break;
                }
                handle_request(&connection, req).await;
            }
            Message::Response(_) => {}
            Message::Notification(_) => {}
        }
    }

    io_threads.join().unwrap();
}

async fn handle_request(connection: &Connection, req: Request) {
    match req.method.as_str() {
        "example/echo" => {
            let params: ExampleParams = serde_json::from_value(req.params).unwrap();
            let response = Response::new_ok(req.id.clone(), params.message);
            connection.sender.send(Message::Response(response)).unwrap();
            println!("Bruhn't")
        }
        _ => {
            let response = Response::new_err(req.id.clone(), -32601, "Method not found".to_string());
            connection.sender.send(Message::Response(response)).unwrap();
            println!("Bruh")
        }
    }
}
