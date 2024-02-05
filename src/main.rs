use  actix_web::{error, guard, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use std::sync::{Arc,Mutex};
use serde_derive::Deserialize;
use actix_web::web::PathConfig;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

fn scoped(cfg: &mut web::ServiceConfig) {
    cfg.service (
        web::resource("/service")
            .route(web::get().to(|| async {HttpResponse::Ok().body("service")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[get("/users/{user_id}/{friend}")]
//async fn index(path: web::Path<(i32, String)>) -> Result<String> { // opt 1
//async fn index(info: web::Path<Info>) -> Result<String> {  // opt 2
 // async fn index(info: web::Path<Info>) -> Result<String> { // didnt work
async fn index(req: HttpRequest) -> Result<String> {
    //let (user_id, friend) = path.into_inner(); // opt 1
    let user_id: i32 = req.match_info().get("user_id").unwrap().parse().unwrap();
    let friend = req.match_info().get("friend").unwrap();
    Ok(format!("Welcome {}, with ID {}!", friend, user_id))
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name =  &data.app_name;
    HttpResponse::Ok().body(format!("{}",app_name))
}

#[post("/echo")]
async fn echo(req_body: web::Data<String>) -> impl Responder {
    println!("{}", &*req_body);
    HttpResponse::Ok().body(format!("{}",&*req_body))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there, how are you!!!")
}
#[get("/count")]
async fn count(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter_1 = data.counter.lock().unwrap();
    *counter_1+=1;
    HttpResponse::Ok().body(format!("{}", *counter_1))
}
#[get("/counter")]
async fn counter(data: web::Data<Arc<Mutex<i32>>>) -> impl Responder {
    // println!("{:#?}",data);
    let mut counter_2 = data.lock().unwrap();
    *counter_2+=1;
    HttpResponse::Ok().body(format!("The count is {}", *counter_2))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState { app_name: "My First actix-web app".to_string(),});
    let echo_string = web::Data::new(String::from("ECHO")); //comment 1

    let counter_data = web::Data::new(AppStateWithCounter { counter: Mutex::new(0)});

    let a_counter_data = web::Data::new(Arc::new(Mutex::new(0)));

    HttpServer::new(move || {
        App::new().service(
            
            web::scope("/api").configure(scoped))
            // .service(
            //     web::resource("/users/{user_id}/{friend}")
            //         .app_data(PathConfig::default().error_handler(|err, req| {
            //             error::InternalError::from_response(
            //                 err,
            //                 HttpResponse::Conflict().into(),
            //             )
            //             .into()
            //         }))
            //         .route(web::get().to(index)),
            // )
            .service(
            web::scope("/app")
            // .guard(guard::Post())
            .route("/hey", web::get().to(manual_hello))
            .service(hello)
            .service(echo)
            .service(count)
            .service(counter)
            .service(index)
        )
            .app_data(app_data.clone())

            .app_data(echo_string.clone()) // comment 1

            .app_data(counter_data.clone())

            .app_data(a_counter_data.clone())
        

    })
    .bind(("127.0.0.1",8085))?
    .run()
    .await
}