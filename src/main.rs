use slack_hook::{PayloadBuilder, Slack};
use std::env;

fn main() {
    match env::var("SLACK_DEPLOY_HOOK") {
        Ok(hook) => {
            let slack = Slack::new(hook).unwrap();
            let p = PayloadBuilder::new()
                .text("body text")
                .channel("#build-notifications")
                .username("Helm Chart Installer")
                .icon_emoji(":chart_with_upwards_trend:")
                .build()
                .unwrap();
        }
        _ => (),
    };
}
