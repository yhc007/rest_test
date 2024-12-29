use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use axum::{
    routing::post,
    Router,
    Json,
};

#[derive(Serialize, Deserialize, Debug)]
struct Alarm {
    #[serde(rename = "type")]
    alarm_type: String,
    alarm_code: String,
    alarm_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PathData {
    path: i32,
    spindle_load: f64,
    spindle_override: i32,
    spindle_speed: f64,
    feed_override: i32,
    aux_codes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MachineData {
    shop_id: i32,
    nc_id: String,
    timestamp: i64,
    part_count: i32,
    total_part_count: i32,
    status: String,
    main_pgm_nm: String,
    mode: String,
    alarms: Vec<Alarm>,
    path_data: Vec<PathData>,
}

// 데이터 생성 함수
fn create_machine_data() -> MachineData {
    let mut aux_codes = HashMap::new();
    aux_codes.insert("T".to_string(), "12".to_string());

    MachineData {
        shop_id: 1,
        nc_id: "1cab6abf-0056-4838-a1f1-251c1691ffca".to_string(),
        timestamp: 1644198063000,
        part_count: 15,
        total_part_count: 131,
        status: "START".to_string(),
        main_pgm_nm: "O0001".to_string(),
        mode: "MEMORY".to_string(),
        alarms: vec![Alarm {
            alarm_type: "nc".to_string(),
            alarm_code: "111".to_string(),
            alarm_message: "fatal error".to_string(),
        }],
        path_data: vec![PathData {
            path: 1,
            spindle_load: 11.1,
            spindle_override: 100,
            spindle_speed: 10.1,
            feed_override: 200,
            aux_codes,
        }],
    }
}

// HTTP POST 요청 전송 함수
async fn send_machine_data(data: &MachineData) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/data")
        .json(data)
        .send()
        .await?;
    
    let status = res.status();
    let response_text = res.text().await?;
    
    println!("응답 상태: {}", status);
    println!("응답 내용: {}", response_text);
    
    Ok(response_text)
}

// 테스트 서버의 핸들러 함수
async fn handle_data(Json(data): Json<MachineData>) -> Json<&'static str> {
    println!("받은 데이터: {:?}", data);
    Json("데이터를 성공적으로 받았습니다")
}

// 테���트 서버 시작 함수
async fn start_server() {
    let app = Router::new()
        .route("/data", post(handle_data));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("서버 시작: {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 테스트 서버를 별도의 태스크로 시작
    tokio::spawn(start_server());
    
    // 서버가 시작될 때까지 잠시 대기
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 데이터 전송 테스트
    let data = create_machine_data();
    send_machine_data(&data).await?;
    
    // 프로그램이 바로 종료되지 않도록 잠시 대기
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    Ok(())
} 