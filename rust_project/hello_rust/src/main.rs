use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;
use std::env;

async fn index() -> HttpResponse {
    // URL de l'image
    let image_url = "https://static.wikia.nocookie.net/weeky-shonen-jump/images/f/f4/WSJ_Issue_2025_01_Cover.png/revision/latest/scale-to-width-down/268?cb=20241201151432";

    // Contenu HTML avec l'image
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="fr">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Affichage de l'image</title>
        </head>
        <body>
            <h1>Image affichée depuis Rust avec Actix-web</h1>
            <img src="{}" alt="WSJ Image"/>
            <p>L'URL de l'image : <a href="{}" target="_blank">{}</a></p>
        </body>
        </html>
        "#, image_url, image_url, image_url);

    // Retourne une réponse HTTP avec le contenu HTML
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Récupérer le port à partir de l'environnement
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Lancer le serveur web Actix
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))  // Route pour afficher la page
    })
    .bind(format!("0.0.0.0:{}", port))?  // Écouter sur toutes les interfaces réseau
    .run()
    .await
}
