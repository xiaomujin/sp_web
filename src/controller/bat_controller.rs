use salvo::{handler, Request, Response, Router};
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::instrument;

#[instrument]
pub fn init() -> Router {
    Router::with_path("bat").get(execute_bat_file)
}

/**
 * 执行 windows bat 文件
*/
#[handler]
async fn execute_bat_file(req: &mut Request, res: &mut Response) {
    let bat_name = req.query::<String>("name").unwrap_or_default();
    let bat_file_path = "D:\\Work\\xyx_gecao_front\\";
    let full_bat_path = format!("{}{}.bat", bat_file_path, bat_name);
    tracing::info!("full_bat_path: {}", full_bat_path);
    if !Path::new(&full_bat_path).exists() {
        res.write_body("File not found").unwrap();
        return;
    }

    // 异步执行 bat 文件
    let Ok(Ok(output)) = tokio::task::spawn_blocking(move || {
        Command::new("cmd")
            .arg("/C")
            .arg(&full_bat_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    else {
        tracing::error!("Task failed");
        res.write_body("Task failed").unwrap();
        return;
    };

    if output.status.success() {
        res.write_body(output.stdout).unwrap();
    } else {
        tracing::error!("Command failed with status: {}", output.status);
        res.write_body(output.stderr).unwrap();
    }
}
