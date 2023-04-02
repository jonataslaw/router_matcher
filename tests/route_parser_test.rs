#[cfg(test)]
mod tests {
    use ::router_matcher::RouteMatcher;

    #[test]
    fn test_route_matching() {
        let mut route_matcher = RouteMatcher::new();

        route_matcher.add_route("/users");
        route_matcher.add_route("/users/:id");
        route_matcher.add_route("/posts");
        route_matcher.add_route("/posts/:id");

        let result1 = route_matcher.match_route("/users");
        let t = result1.unwrap();

        assert_eq!(t.path, "/users");
        assert!(t.parameters.is_empty());

        let result2 = route_matcher.match_route("/users/123");
        let t = result2.unwrap();
        assert_eq!(t.path, "/users/:id");
        assert_eq!(t.parameters.get("id"), Some(&"123".to_string()));

        let result3 = route_matcher.match_route("/posts");
        let t = result3.unwrap();
        assert_eq!(t.path, "/posts");
        assert!(t.parameters.is_empty());

        let result4 = route_matcher.match_route("/posts/456");
        let t = result4.unwrap();
        assert_eq!(t.path, "/posts/:id");
        assert_eq!(t.parameters.get("id"), Some(&"456".to_string()));

        let result5 = route_matcher.match_route("/nonexistent");
        assert!(result5.is_none());
    }

    #[test]
    fn test_route_matching_with_url_parameters() {
        let mut route_matcher = RouteMatcher::new();

        route_matcher.add_route("/users");
        route_matcher.add_route("/users/:id");
        route_matcher.add_route("/posts");
        route_matcher.add_route("/posts/:id");

        let result1 = route_matcher.match_route("/users?foo=bar");
        let t = result1.unwrap();
        assert_eq!(t.path, "/users");
        assert!(t.parameters.is_empty());
        assert_eq!(t.url_parameters.get("foo"), Some(&"bar".to_string()));

        let result2 = route_matcher.match_route("/users/123?foo=bar");
        let t = result2.unwrap();
        assert_eq!(t.path, "/users/:id");
        assert_eq!(t.parameters.get("id"), Some(&"123".to_string()));
        assert_eq!(t.url_parameters.get("foo"), Some(&"bar".to_string()));

        let result3 = route_matcher.match_route("/posts?foo=bar");
        let t = result3.unwrap();
        assert_eq!(t.path, "/posts");
        assert!(t.parameters.is_empty());
        assert_eq!(t.url_parameters.get("foo"), Some(&"bar".to_string()));

        let result4 = route_matcher.match_route("/posts/456?foo=bar");
        let t = result4.unwrap();
        assert_eq!(t.path, "/posts/:id");
        assert_eq!(t.parameters.get("id"), Some(&"456".to_string()));
        assert_eq!(t.url_parameters.get("foo"), Some(&"bar".to_string()));

        let result5 = route_matcher.match_route("/nonexistent?foo=bar");
        assert!(result5.is_none());
    }

    #[test]
    fn test_adds_route_to_routing_tree() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id");
        assert!(matcher.match_route("user/123").is_some());

        matcher.add_route("profile/followers");
        assert!(matcher.match_route("profile/followers").is_some());

        matcher.add_route("home/feed/photos");
        assert!(matcher.match_route("home/feed/photos").is_some());
    }

    #[test]
    fn test_matches_route_and_extracts_parameters() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id");
        let result = matcher.match_route("user/123");
        assert_eq!(result.as_ref().unwrap().path, "user/:id");
        assert_eq!(
            result.unwrap().parameters.get("id"),
            Some(&"123".to_string())
        );
    }

    #[test]
    fn test_matches_route_with_url_parameters() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("/user");
        let result = matcher.match_route("user?foo=bar");
        assert_eq!(result.as_ref().unwrap().path, "/user");
        assert_eq!(
            result.unwrap().url_parameters.get("foo"),
            Some(&"bar".to_string())
        );
    }

    #[test]
    fn test_matches_route_with_root_path() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("/");
        assert!(matcher.match_route("/").is_some());
    }

    #[test]
    fn test_does_not_match_route_missing_segments() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id");
        assert!(matcher.match_route("product/abc").is_none());

        matcher.add_route("home");
        assert!(matcher.match_route("product").is_none());
    }

    #[test]
    fn test_does_not_match_route_not_exist() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id");
        assert!(matcher.match_route("product/abc").is_none());

        matcher.add_route("home");
        assert!(matcher.match_route("settings").is_none());
    }

    #[test]
    fn test_does_not_match_route_missing_segment() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/settings");
        assert!(matcher.match_route("user/").is_none());
    }

    #[test]
    fn test_does_not_match_route_more_segments() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user");
        assert!(matcher.match_route("user/settings").is_none());
    }

    #[test]
    fn test_does_not_match_route_wrong_path() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id/comments");
        assert!(matcher.match_route("user/1234/feed").is_none());
    }

    #[test]
    fn test_matches_route_with_query_parameters() {
        let mut matcher = RouteMatcher::new();
        matcher.add_route("user/:id");
        let result = matcher.match_route("user/123?foo=bar&baz=qux").unwrap();
        assert_eq!(result.path, "user/:id");
        assert_eq!(result.parameters.get("id"), Some(&"123".to_string()));
        assert_eq!(result.url_parameters.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.url_parameters.get("baz"), Some(&"qux".to_string()));
    }
}
