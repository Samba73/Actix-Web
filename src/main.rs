use  actix_web::{guard, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc,Mutex};

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn scoped(cfg: &mut web::ServiceConfig) {
    cfg.service (
        web::resource("/service")
            .route(web::get().to(|| async {HttpResponse::Ok().body("service")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
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
            .service(
            web::scope("/app")
            // .guard(guard::Post())
            .route("/hey", web::get().to(manual_hello))
            .service(hello)
            .service(echo)
            .service(count)
            .service(counter)
            
        )
            .app_data(app_data.clone())
            // .app_data(web::Data::new(AppState{
            //     app_name: "My first actix-web app".to_string(),
            // }))
            .app_data(echo_string.clone()) // comment 1
            // .app_data(web::Data::new(String::from("ECHO")))
            .app_data(counter_data.clone())
            // .app_data(Arc::clone(a_counter_data)) // not required to use Arc::clone in web::Data
            .app_data(a_counter_data.clone())
            // .service(hello)
            // .service(echo)
            // .service(
            //     web::scope("/app")
            //         .route("/hey", web::get().to(manual_hello))
            // )

    })
    .bind(("127.0.0.1",8085))?
    .run()
    .await
}