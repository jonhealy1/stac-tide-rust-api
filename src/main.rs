use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct GeoJSON {
    geojson_type: String,
    coordinates: Vec<i32>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/name/:name").get(hello_name);
    app.at("/add/geojson").post(add_geojson);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello_name(req: Request<()>) ->  tide::Result<String> {
    let name: String = req.param("name")?.to_string();
    let msg = format!("Hello, {}!", name).into();
    Ok(msg)
}

async fn add_geojson(mut req: Request<()>) -> tide::Result {
    let GeoJSON { geojson_type, coordinates } = req.body_json().await?;
    Ok(format!("Type: {}, Coordiantes: {:?}", geojson_type, coordinates).into())
}