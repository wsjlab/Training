use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Get environment variables
    let email = env::var("EMAIL").expect("EMAIL must be set");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
    let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    // Email content
    let subject = "Artist of the Week";
    let body = format!(
        "Hello,\n\nCheck out the Artist of the Week here: {}\n\nBest regards,\nYour Rust App",
        app_url
    );

    // Create the email message
    let email_message = Message::builder()
        .from(format!("Rust App <{}>", email).parse().unwrap())
        .to(email.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    // Set up the SMTP transport
    let creds = Credentials::new(email.clone(), email_password); // Use the correct email string
    let mailer = SmtpTransport::relay("smtp.office365.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email_message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}