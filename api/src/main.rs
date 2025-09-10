use poem::get;
use poem::handler;
use poem::post;
use poem::{listener::TcpListener, web::Json, web::Path, Route, Server};
use store::store::Store;

use crate::request_inputs::CreateUserInput;
use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateUserOutput;
use crate::request_outputs::CreateWebsiteOutput;
use crate::request_outputs::SignInOutput;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::default().unwrap();
    let website = s.get_website(id).unwrap();

    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s: Store = Store::default().unwrap();
    let id: String = s.sign_up(data.username, data.password).unwrap();

    let response: CreateUserOutput = CreateUserOutput { id: id };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SignInOutput> {
    let mut s: Store = Store::default().unwrap();
    let exists = s.sign_in(data.username, data.password).unwrap_or(false);

    let response = SignInOutput {
        jwt: String::from("jwt"),
    };

    Json(response)
}

#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s: Store = Store::default().unwrap();
    let website = s
        .create_website(
            String::from("998f53de-b430-40a3-8a6a-eb2ab251b5b8"),
            data.url,
        )
        .unwrap();

    let response: CreateWebsiteOutput = CreateWebsiteOutput { id: website.id };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app: Route = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
