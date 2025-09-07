use poem::{listener::TcpListener, web::Path,web::Json, Route, Server};
use poem::handler;
use poem::post;
use poem::get;
use store::Store;

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateWebsiteOutput;

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("Website ID: {}", website_id)
}

#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>) -> 
Json<CreateWebsiteOutput> {

    let s: Store = Store{};
    let id: String = s.create_website();

    let response: CreateWebsiteOutput = CreateWebsiteOutput {
        id
    };

    Json(response)
}



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app: Route = Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website));
Server::new(TcpListener::bind("0.0.0.0:3000"))
     .run(app)
     .await
}
