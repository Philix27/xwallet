use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn builder() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define the email
    let email = Message::builder()
        .from("Your Name <your.email@example.com>".parse().unwrap())
        .reply_to("your.email@example.com".parse().unwrap())
        .to("Recipient Name <recipient.email@example.com>"
            .parse()
            .unwrap())
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();
    // Set up the SMTP client
    let creds = Credentials::new(
        "Mailtrap_smtp_username".to_string(),
        "Mailtrap_smtp_password".to_string(),
    );
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("your_mailtrap_Host.io")?
        .credentials(creds)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
    Ok(())
}
