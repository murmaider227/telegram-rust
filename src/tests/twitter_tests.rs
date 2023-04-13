#[cfg(test)]
mod tests {
    use crate::tools::parse_twitter::parse_twitter_links;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_parse_twitter_links_valid() {
        dotenv().ok();
        let text = "Check out this Twitter account: https://twitter.com/elonmusk";
        let result = parse_twitter_links(text).await.unwrap();
        assert!(result.contains("Twitter"));
        assert!(result.contains("elonmusk"));
    }

    #[tokio::test]
    async fn test_parse_twitter_links_invalid() {
        dotenv().ok();
        let text = "This is not a valid Twitter link: https://twitter.com/not-a-real-user";
        let result = parse_twitter_links(text).await.unwrap_or_default();
        assert!(!result.contains("Twitter"));
        assert!(!result.contains("not-a-real-user"));
    }

    #[tokio::test]
    async fn test_parse_twitter_links_no_links() {
        dotenv().ok();
        let text = "There are no Twitter links in this text.";
        let result = parse_twitter_links(text).await;
        assert!(result.is_none());
    }
}
