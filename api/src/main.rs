use poem::get;
use poem::handler;
use poem::post;
use poem::web::Data;
use poem::EndpointExt;
use poem::{listener::TcpListener, web::Json, web::Path, Route, Server};
use std::sync::Arc;
use std::sync::Mutex;
use store::store::Store;

use crate::request_inputs::CreateUserInput;
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateUserOutput;
use crate::request_outputs::CreateWebsiteOutput;
use crate::request_outputs::GetWebsiteOutput;
use crate::request_outputs::SignInOutput;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut lock_s = s.lock().unwrap();

    let website = lock_s.get_website(id).unwrap();

    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let mut lock_s = s.lock().unwrap();

    let website_id: String = lock_s.sign_up(data.username, data.password).unwrap();

    let response: CreateUserOutput = CreateUserOutput { id: website_id };

    Json(response)
}

#[handler]
fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SignInOutput> {
    let mut lock_s = s.lock().unwrap();

    let _exists = lock_s
        .sign_in(data.username, data.password)
        .unwrap_or(false);

    let response = SignInOutput {
        jwt: String::from("jwt"),
    };

    Json(response)
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut lock_s = s.lock().unwrap();
    let website = lock_s
        .create_website(
            String::from("998f53de-b430-40a3-8a6a-eb2ab251b5b8"),
            data.url,
        )
        .unwrap();

    let response: CreateWebsiteOutput = CreateWebsiteOutput { id: website.id };

    Json(response)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::default().unwrap()));
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
