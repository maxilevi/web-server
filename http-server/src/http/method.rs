pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,

}

impl Method {
    pub fn parse(method_str: &str) -> Result<Method, &'static str> {
        match method_str {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "DELETE" => Ok(Method::Delete),
            _ => {
                Err("Not implemented")
            }
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get() {
        assert_eq!(
            Method::Get.to_string(),
            Method::parse("GET").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_post() {
        assert_eq!(
            Method::Post.to_string(),
            Method::parse("POST").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_put() {
        assert_eq!(
            Method::Put.to_string(),
            Method::parse("PUT").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_patch() {
        assert_eq!(
            Method::Patch.to_string(),
            Method::parse("PATCH").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_delete() {
        assert_eq!(
            Method::Delete.to_string(),
            Method::parse("DELETE").unwrap().to_string()
        );
    }
}
