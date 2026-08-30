use anyhow::Context;
use env_logger::Env;
use fireplace::auth::FirebaseAuthClient;
use fireplace::auth::models::NewUser;
use ulid::Ulid;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .is_test(false)
        .init();

    let auth_client = FirebaseAuthClient::emulator("http://127.0.0.1:9099", None)
        .expect("Failed to create auth client");

    let uid = Ulid::generate();
    let new_user = NewUser::builder()
        .username(format!("dessalines-{}", uid.to_string()))
        .email(format!("dessalines-{}@example.com", uid.to_string()))
        .password("hello123".to_string())
        .email_verified(true)
        .build();

    auth_client
        .create_user(new_user.clone())
        .await
        .context("Failed to create user")?;

    let users = auth_client
        .get_all_users()
        .await
        .context("Failed to get u sers")?;

    println!(" Here are the list of users {:#?}", users);
    let auth_claims = auth_client
        .login_with_password(new_user.email.as_str(), new_user.password.as_str(), true)
        .await
        .context("Failed to authenticate password credentials")?;

    println!("User logged in successfully {:?}", auth_claims);

    Ok(())
}
