use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};

#[derive(Clone)]
struct Config {
    host: String,
    port: u16,
    napcat_url: String,
    napcat_token: String,
    group_id: i64
}

async fn deal_js(text:&str,character:&str,conf:&Config) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let js = serde_json::json!({
        "group_id":conf.group_id,
        "character":character,
        "text":text
    });
    let token_encode = urlencoding::encode(&conf.napcat_token);
    let req_url = format!("{}/get_ai_record?access_token={}",conf.napcat_url,token_encode);
    let req = client.post(req_url)
                .header("Content-Type", "application/json")
                .body(js.to_string());
    let json_text = req.send().await?.text().await?;
    let js:serde_json::Value = serde_json::from_str(&json_text)?;
    let retcode = js["retcode"].as_i64().ok_or("`retcode` not found in napcat response json")?;
    if retcode != 0 {
        return Err(format!("napcat return error: {}", retcode).into());
    }
    let silk_url = js["data"].as_str().ok_or("`data` not found in napcat response json")?;
    let client = reqwest::Client::new();
    let data = client.get(silk_url).send().await?.bytes().await?;
    let wav = silk2wav::silk_to_wav(&data,24000)?;
    Ok(wav)
}


async fn read_config() -> Result<Config, Box<dyn std::error::Error>> {

    // 判断文件是否存在
    if !tokio::fs::metadata("config.json").await.is_ok() {
        let js = serde_json::json!({
            "host":"127.0.0.1",
            "port":7788,
            "napcat_url":"http://127.0.0.1:3000",
            "napcat_token":"",
            "group_id":0
        });
        tokio::fs::write("config.json", js.to_string()).await?;
        return Err("config.json not found, create a new one, please edit it".into());
    }

    let file_data = tokio::fs::read("config.json").await?;
    let js:serde_json::Value = serde_json::from_slice(&file_data)?;
    let host = js["host"].as_str().unwrap_or("127.0.0.1");
    let port = js["port"].as_i64().unwrap_or(7788);
    let napcat_url = js["napcat_url"].as_str().unwrap_or("http://127.0.0.1:3000");
    let napcat_token = js["napcat_token"].as_str().unwrap_or("");
    let group_id = js["group_id"].as_i64().ok_or("the group_id in config.json not set")?;
    Ok(Config {
        host:host.to_string(),
        port:port as u16,
        napcat_url:napcat_url.to_string(),
        napcat_token:napcat_token.to_string(),
        group_id
    })
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    log::info!("欢迎使用 NAPCAT-TTS-SERVER version 0.1.0 by super1207！");

    log::info!("这个软件可以利用napcat来暴露QQ的TTS接口！");

    log::info!("软件返回wav格式的音频文件");

    log::info!("使用例子如下：");

    log::info!("http://127.0.0.1:7788/tts?character=lucy-voice-suxinjiejie&text=这是一段测试文本");

    log::info!("The natcat-tts-server is starting...");

    let conf = read_config().await.unwrap();
    
    let host_port = format!("{}:{}",conf.host,conf.port);

    let app = Router::new().route("/tts", get(|query:Query<HashMap<String, String>>| async move {
            
        let text;
        match query.get("text") {
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    "the `text` param not found",
                ).into_response();
            },
            Some(t) => {
                if t == "" {
                    return (
                        StatusCode::BAD_REQUEST,
                        "the len of `text` param too short",
                    ).into_response();
                }
                text = t.as_str();
            }
        }
        let mut character = "lucy-voice-m8";
        if let Some(c) =  query.get("character") {
            if c != "" {
                character = c.as_str();
            }
        }

        log::info!("text:{} character:{}",text,character);

        match deal_js(text,character,&conf).await {
            Ok(wav_data) => {
                return (
                    [("Content-Type", "audio/wav")],
                    wav_data,
                ).into_response();
            },
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("err:{err:?}"),
                ).into_response();
            },
        }
    }));

    log::info!("The natcat-tts-server is running on:{host_port}");
    
    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}