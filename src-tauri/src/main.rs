#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{State, async_runtime::Mutex};
use tcp_smart_socket::*;

struct ManagedState {
    client: Option<SmartSocketClient>,
}
unsafe impl Send for ManagedState {}
impl ManagedState {
    async fn turn_on(&mut self) -> Result<ExecutionResult, String> {
        if let Some(mut client) = self.client.take() {
            let res = client.turn_on().await;
            self.client = Some(client);
            res
        } else {
            Ok(ExecutionResult::Error(CustomError::CommandExecutionFailure("Client not connected to remote socket. Connect first".into())))
        }
    }
    async fn connect_client(&mut self, addr: impl tokio::net::ToSocketAddrs + std::fmt::Debug) -> Result<String, String> {
        if self.client.is_some() { return Ok("Already connected. Disconnect first".to_string())}
        match SmartSocketClient::with_addr(&addr).await {
            Ok(client) => {self.client = Some(client); Ok("Connected".into())},
            Err(e) => Ok(e)
        }

    }
        async fn turn_off(&mut self) -> Result<ExecutionResult, String> {
        if let Some(mut client) = self.client.take() {
            let res = client.turn_off().await;
            if res.is_ok() {self.client = Some(client);}
                        println!("res: {:?}", res);

            res
        } else {
            Ok(ExecutionResult::Error(CustomError::CommandExecutionFailure("Client not connected to remote socket. Connect first".into())))
        }
    }
        async fn status(&mut self) -> Result<ExecutionResult, String> {
        if let Some(mut client) = self.client.take() {
            let res = client.get_status().await;
            if res.is_ok() {self.client = Some(client);}
            println!("res: {:?}", res);
            res
        } else {
            Ok(ExecutionResult::Error(CustomError::CommandExecutionFailure("Client not connected to remote socket. Connect first".into())))
        }
    }
}

fn main() {
    let state = ManagedState { client: None };

    tauri::Builder::default()
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![turn_on, connect_client, turn_off, status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn turn_on(state: State<'_, Mutex<ManagedState>>) -> Result<ExecutionResult, String> {
    state.lock().await.turn_on().await

}
#[tauri::command]
async fn connect_client(addr: &str, state: State<'_, Mutex<ManagedState>>) -> Result<String, String> {
    state.lock().await.connect_client(addr).await
}


#[tauri::command]
async fn turn_off(state: State<'_, Mutex<ManagedState>>) -> Result<ExecutionResult, String> {
    state.lock().await.turn_off().await
}
#[tauri::command]
async fn status(state: State<'_, Mutex<ManagedState>>) -> Result<ExecutionResult, String> {
    state.lock().await.status().await
}
