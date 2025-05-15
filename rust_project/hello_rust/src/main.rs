use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};
use rand::{Rng, rngs::ThreadRng}; // Updated rand imports

// Struct to deserialize Supabase response
#[derive(Deserialize, Debug)]
struct Auteur {
    nom: String,
}

// Shared state for the "Random Name"
struct AppState {
    random_name: Arc<Mutex<String>>,
}

// Fetch all authors from Supabase
async fn fetch_all_auteurs() -> Vec<String> {
    println!("Fetching all auteurs...");

    // Supabase API URL and Key
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    let client = Client::new();

    // Fetch all rows from the table
    let response = client
        .get(format!("{}/rest/v1/auteurs?select=nom", supabase_url))
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send()
        .await
        .expect("Failed to fetch auteurs");

    // Log the raw response body
    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // Log the raw response

    // Deserialize the response body into a list of authors
    let auteurs: Vec<Auteur> = serde_json::from_str(&response_body).unwrap_or_else(|_| {
        println!("Failed to deserialize response");
        vec![]
    });

    println!("Deserialized auteurs: {:?}", auteurs);

    // Extract the names of the authors
    auteurs.into_iter().map(|auteur| auteur.nom).collect()
}

// Update the random name every 15 minutes
async fn update_random_name(state: Arc<Mutex<String>>) {
    loop {
        println!("Starting update_random_name task...");

        // Wait for 15 minutes
        time::sleep(Duration::from_secs(15 * 60)).await;
        println!("15 minutes elapsed. Fetching a new random name...");

        // Fetch all authors from Supabase
        let all_auteurs = fetch_all_auteurs().await;

        // Log fetched authors
        println!("Fetched auteurs: {:?}", all_auteurs);

        // Select a random author if the list is not empty
        if !all_auteurs.is_empty() {
            let mut rng = rand::thread_rng(); // Corrected RNG initialization
            let random_index = rng.gen_range(0..all_auteurs.len()); // Generate a random index using `gen_range`
            let random_author = all_auteurs[random_index].clone();

            // Update the shared state
            let mut random_name = state.lock().unwrap();
            *random_name = random_author;

            println!("Updated Random Name: {}", *random_name);
        } else {
            println!("No authors found in the database.");
        }
    }
}

// Route to handle the index page
async fn index() -> HttpResponse {
    println!("Handling request to index route...");

    // Fetch all authors from Supabase
    let all_auteurs = fetch_all_auteurs().await;

    // Random author for display
    let random_nom = if !all_auteurs.is_empty() {
        let mut rng = rand::thread_rng(); // Corrected RNG initialization
        let random_index = rng.gen_range(0..all_auteurs.len());
        let random_author = all_auteurs[random_index].clone();
        println!("Random selected author: {}", random_author);
        Some(random_author)
    } else {
        None
    };

    // Log the random name
    if let Some(name) = &random_nom {
        println!("Random Name: {}", name);
    } else {
        println!("No authors available for random selection.");
    }

    // HTML content with a Tailwind CSS-styled table and "Random Name" instead of "Artist of the Week"
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="fr">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>WSJ Auteur List</title>
            <script src="https://cdn.tailwindcss.com"></script>
            <script>
                function toggleTable() {{
                    const table = document.getElementById('auteursTable');
                    if (table.style.display === 'none') {{
                        table.style.display = 'table';
                    }} else {{
                        table.style.display = 'none';
                    }}
                }}
            </script>
        </head>
        <body class="bg-gray-100">
            <div class="container mx-auto mt-10">
                <h2 class="text-xl font-semibold text-center mb-4">
                    The artist of the week is: <span class="text-blue-500">{}</span>
                </h2>
                <h1 class="text-3xl font-bold text-center mb-6">Liste des auteurs</h1>
                <div class="text-center mb-6">
                    <button onclick="toggleTable()" class="bg-blue-200 hover:bg-blue-400 text-white font-bold py-2 px-4 rounded">
                        Display Auteurs
                    </button>
                </div>
                <div class="overflow-x-auto">
                    <table id="auteursTable" class="table-auto border-separate border border-gray-300 w-full text-sm rounded-lg" style="display: none;">
                        <thead>
                            <tr class="bg-gray-200">
                                <th class="border border-gray-300 px-4 py-2 text-left rounded-tl-lg">#</th>
                                <th class="border border-gray-300 px-4 py-2 text-left rounded-tr-lg">Nom</th>
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
            </div>
        </body>
        </html>
        "#,
        random_nom.unwrap_or_else(|| "No random name".to_string()), // Display the random name or a default message if none
        all_auteurs
            .into_iter()
            .enumerate()
            .map(|(i, nom)| format!("<tr><td class=\"border border-gray-300 px-4 py-2 rounded-l-lg\">{}</td><td class=\"border border-gray-300 px-4 py-2 rounded-r-lg\">{}</td></tr>", i + 1, nom))
            .collect::<String>()
    );

    // Return an HTTP response with the HTML content
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the port from the environment
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Starting Actix web server on port {}", port);

    // Shared state for the "Random Name"
    let random_name = Arc::new(Mutex::new(String::new()));
    let state = web::Data::new(AppState {
        random_name: random_name.clone(),
    });

    // Start the background task to update the "Random Name"
    tokio::spawn(update_random_name(random_name));

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index)) // Route to display the page
    })
    .bind(format!("0.0.0.0:{}", port))? // Listen on all network interfaces
    .run()
    .await
}
