use tide::{Request, Response, Error};
use sqlx::query;

pub fn register_route(app: &mut tide::Server<crate::State>) {
    app.at("/hello").post(handler);
}

pub async fn handler(mut req: Request<crate::State>) -> tide::Result {
    let greeting: crate::Greeting = req.body_json().await?;
    let db_pool = req.state().db_pool.clone();

    query!(
        r#"
        INSERT INTO "tide-lambda-example-greetings" (name, country_code) VALUES
        ($1, $2) returning name, country_code
        "#,
        greeting.name,
        greeting.country_code
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        match e.as_database_error() {
            Some(_) => {
                Error::from_str(400, "You already say hi!")
            },
            None => Error::new(409, e)
        }
    })?;

    let mut res = Response::new(201);

    res.set_body(format!("Hello {}, welcome to this tide lambda example.", greeting.name ));
    Ok(res)
}
