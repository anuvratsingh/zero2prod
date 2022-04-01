use zero2prod::configuration::get_configuration;

use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

/// Compose multiple layers into a `tacing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid hacing to spell out the actual type of the returned subsriber, which is indeed complex.
/// We need to explicitly call out that the returned subscriber is `Send` and `Sync` to make it possbile to pass it to `init_subscriber` later on.

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");

    let applicaiton = Application::build(configuration).await?;
    applicaiton.run_until_stopped().await?;

    Ok(())
}
