use tracing::{Level, info_span};
use tracing_subscriber::prelude::*;

mod hello;

fn main() {
    setup_log_with_target_cli();
    tracing::info!("Starting");
    logtesterlib::say_hello();
    
    hello::say_hello();

    {
        let model="AUCTION";

        let hello_span = info_span!("hello_span", model);
        let _enter = hello_span.enter();
        tracing::debug!("Hello world D");
        tracing::info!("Hello world I");
        tracing::warn!("Hello world W");
    }
    

    tracing::info!("Ending");
}

fn setup_log_with_env_filter_json() {
    let filter = tracing_subscriber::filter::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .parse_lossy("WARN,logtestermain=INFO,logtestermain::hello=WARN");
    tracing_subscriber::fmt::fmt()
        .with_env_filter(filter)        
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(true)
        .init();
}

fn setup_log_with_target_json() {
    let target = tracing_subscriber::filter::Targets::new()
        .with_default(Level::WARN)
        .with_target("logtestermain", Level::INFO)
        .with_target("logtestermain::hello", Level::WARN);
    let fmt = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(true);
    tracing_subscriber::registry()
        .with(fmt)
        .with(target)
        .init();
}

fn setup_log_with_target_cli() {
    let target = tracing_subscriber::filter::Targets::new()
        .with_default(Level::WARN)
        .with_target("logtestermain", Level::INFO)
        .with_target("logtestermain::hello", Level::WARN);
    let fmt = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(fmt)
        .with(target)
        .init();
}