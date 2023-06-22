use rusqlite::{params, Connection};
use warp::{Filter, Reply};

#[tokio::main]
fn main() {
    // Set up the in-memory database connection
    let conn = Connection::open_in_memory().unwrap();
    create_table(&conn).unwrap();

    // Define the route for retrieving the JSON object
    let object_route = warp::path!("org" / String)
        .and(warp::get())
        .and(warp::any().map(move || conn))
        .and_then(get_object);

    // Start the Warp server
    warp::serve(object_route).run(([127, 0, 0, 1], 8080)).await;
}

async fn get_object(org_id: String, conn: Connection) -> Result<impl Reply, warp::Rejection> {
    // Check if the object exists in the database
    let result = conn.query_row("SELECT json_object FROM objects WHERE org_id = ?1", params![org_id], |row| {
        row.get::<_, String>(0)
    });

    match result {
        Ok(json) => {
            // Object found in the database, return it as JSON
            Ok(warp::reply::json(&json))
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Object not found in the database, search the external API
            let api_url = format!("https://data.brreg.no/enhetsregisteret/api/enheter/{}/roller", org_id);
            let response = reqwest::get(&api_url).await.unwrap();
            let json = response.json::<serde_json::Value>().await.unwrap();

            // Insert the retrieved object into the database
            conn.execute("INSERT INTO objects (org_id, json_object) VALUES (?1, ?2)", params![org_id, json.to_string()]).unwrap();

            // Return the retrieved object as JSON
            Ok(warp::reply::json(&json))
        },
        Err(_) => {
            // Error occurred while querying the database
            Err(warp::reject())
        }
    }
}

fn create_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS objects (
            org_id TEXT PRIMARY KEY,
            json_object TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

