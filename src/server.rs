use tide::Request;
#[path = "handler.rs"]
mod handler;

const ENTRY_POINT: &'static str = "/:word";
const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8080";

pub async fn start() -> tide::Result<()> {
  let mut app = tide::new();
  app.at(ENTRY_POINT).get(receive_request);
  app.listen(DEFAULT_ADDRESS).await?;
  Ok(())
}

async fn receive_request(req: Request<()>) -> tide::Result {
  let word = req.param("word").unwrap_or("error");
  let answer = handler::make_responce_from_db(word);
  Ok(format!("{}", answer).into())
}
