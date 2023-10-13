use std::{collections::HashMap, sync::Mutex};

use actix_web::{get, Responder, HttpResponse, HttpServer, App, web, post};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct UserBalances {
    table: HashMap<String, i32>
} 

struct AppData {
    balances: Mutex<UserBalances>
}

#[derive(Deserialize)]
struct UserList {
    users: Vec<String>
}



// Возвращает список таблицу пользователей с их балансами
//
// >>> requests.get('http://127.0.0.1:8080/balances').content
// {"table": {"User1":0, "User2":1000, "User3":-100}}
#[get("/balances")]
async fn balances(data: web::Data<AppData>) -> impl Responder {
    let table = data.balances.lock().unwrap();
    HttpResponse::Ok().json(table.clone())
}


// Обновляет балансы для указанных пользователей
//
// >>> requests.post('http://127.0.0.1:8080/update', json={'table': {'User1': 10000, 'User400': 400}})
#[post("/update")]
async fn update(data: web::Data<AppData>, info: web::Json<UserBalances>) -> impl Responder {
    let mut table = data.balances.lock().unwrap();
    table.table.extend(info.0.table);
    HttpResponse::Ok()
}


// Удаляет указанных пользователей
//
// >>> rerequests.post('http://127.0.0.1:8080/delete', json={'users': ['User2']})
#[post("delete")]
async fn delete(data: web::Data<AppData>, users: web::Json<UserList>) -> impl Responder {
    let mut table = data.balances.lock().unwrap();
    for user in users.0.users {
        table.table.remove(&user);
    }
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut data = HashMap::new();
    data.insert("User1".to_string(), 0);
    data.insert("User2".to_string(), 1000);
    data.insert("User3".to_string(), -100);
    
    let app_data = web::Data::new(AppData {balances: Mutex::new(UserBalances {table: data})});

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(balances)
            .service(update)
            .service(delete)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}