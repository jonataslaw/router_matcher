use std::collections::HashMap;

pub struct RouteMatcher {
    root: TrieNode,
}

struct TrieNode {
    segment: Segment,
    children: Vec<TrieNode>,
    path: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
enum Segment {
    Static(String),
    Parameter(String),
    Wildcard,
}

pub struct MatchedRoute {
    pub path: String,
    pub parameters: HashMap<String, String>,
    pub url_parameters: HashMap<String, String>,
}

impl RouteMatcher {
    pub fn new() -> RouteMatcher {
        RouteMatcher {
            root: TrieNode::new_root(),
        }
    }

    pub fn add_route(&mut self, path: &str) {
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with(':') {
                    Segment::Parameter(s[1..].to_string())
                } else if s == "*" {
                    Segment::Wildcard
                } else {
                    Segment::Static(s.to_string())
                }
            })
            .collect::<Vec<_>>();
        self.root.add_route(path, &segments);
    }

    pub fn match_route(&self, url: &str) -> Option<MatchedRoute> {
        let (path, query_string) = url.split_at(url.find('?').unwrap_or_else(|| url.len()));
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if let Some((path, parameters)) = self.root.match_segments(&segments, 0) {
            let url_parameters = query_string
                .trim_start_matches('?')
                .split('&')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut parts = s.split('=');
                    (
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap_or("").to_string(),
                    )
                })
                .collect::<HashMap<_, _>>();
            Some(MatchedRoute {
                path,
                parameters,
                url_parameters,
            })
        } else {
            None
        }
    }
}

impl TrieNode {
    fn new_root() -> TrieNode {
        TrieNode {
            segment: Segment::Static("".to_string()),
            children: Vec::new(),
            path: None,
        }
    }

    fn add_route(&mut self, path: &str, segments: &[Segment]) {
        if segments.is_empty() {
            self.path = Some(path.to_string());
            return;
        }

        for child in &mut self.children {
            if child.segment == segments[0] {
                child.add_route(path, &segments[1..]);
                return;
            }
        }

        let mut new_node = TrieNode {
            segment: segments[0].clone(),
            children: Vec::new(),
            path: None,
        };
        new_node.add_route(path, &segments[1..]);
        self.children.push(new_node);
    }

    fn match_segments(
        &self,
        segments: &[&str],
        start: usize,
    ) -> Option<(String, HashMap<String, String>)> {
        if start == segments.len() {
            return self.path.clone().map(|path| (path, HashMap::new()));
        }

        for child in &self.children {
            match &child.segment {
                Segment::Static(s) => {
                    if s == segments[start] {
                        if let Some(result) = child.match_segments(segments, start + 1) {
                            return Some(result);
                        }
                    }
                }
                Segment::Parameter(param) => {
                    let mut parameters = HashMap::new();
                    parameters.insert(param.clone(), segments[start].to_string());
                    if let Some((path, child_params)) = child.match_segments(segments, start + 1) {
                        parameters.extend(child_params);
                        return Some((path, parameters));
                    }
                }
                Segment::Wildcard => {
                    if let Some((path, params)) = child.match_segments(segments, segments.len()) {
                        return Some((path, params));
                    }
                }
            }
        }

        None
    }
}
