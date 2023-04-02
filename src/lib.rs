use std::collections::HashMap;

pub struct RouteMatcher {
    routes: Vec<RouteNode>,
}

struct RouteNode {
    path: String,
    segments: Vec<Segment>,
}

#[derive(PartialEq)]
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
        RouteMatcher { routes: vec![] }
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
        self.routes.push(RouteNode {
            path: path.to_string(),
            segments,
        });
    }

    pub fn match_route(&self, url: &str) -> Option<MatchedRoute> {
        let (path, query_string) = url.split_at(url.find('?').unwrap_or_else(|| url.len()));
        let segments = path
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        for route in &self.routes {
            if let Some(parameters) = route.match_segments(&segments) {
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
                return Some(MatchedRoute {
                    path: route.path.clone(),
                    parameters,
                    url_parameters,
                });
            }
        }

        None
    }
}

impl RouteNode {
    fn match_segments(&self, segments: &[&str]) -> Option<HashMap<String, String>> {
        if self.segments.len() != segments.len() && !self.segments.contains(&Segment::Wildcard) {
            return None;
        }

        let mut parameters = HashMap::new();
        let mut wildcard = false;

        for (route_segment, segment) in self.segments.iter().zip(segments.iter()) {
            match route_segment {
                Segment::Static(s) => {
                    if s != segment {
                        return None;
                    }
                }
                Segment::Parameter(param) => {
                    parameters.insert(param.clone(), (*segment).to_string());
                }
                Segment::Wildcard => {
                    wildcard = true;
                    break;
                }
            }
        }

        if wildcard {
            Some(parameters)
        } else if self.segments.len() == segments.len() {
            Some(parameters)
        } else {
            None
        }
    }
}
