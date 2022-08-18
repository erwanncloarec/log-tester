
pub fn say_hello() {
    tracing::debug!("Hello world D");
    tracing::info!("Hello world I");
    tracing::warn!("Hello world W");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
