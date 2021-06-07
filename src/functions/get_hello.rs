use tide::{Request, Response, Error};
use sqlx::query_as;

pub fn register_route(app: &mut tide::Server<crate::State>) {
    app.at("/hello/:name").get(handler);
}

pub async fn handler(req: Request<crate::State>) -> tide::Result {
    let name =  req.param("name")?;

    let db_pool = req.state().db_pool.clone();
    let row = query_as!(
        crate::Greeting,
        r#"
        SELECT name, country_code FROM "tide-lambda-example-greetings"
        WHERE name = $1
        "#,
        name
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    let res =match row {
        None => Response::new(404),
        Some(row) => {
            let mut r = Response::new(200);
            r.set_body(format!("Hi again {}, nice to see you.", row.name));
            r
        }
    };

    Ok(res)
}
