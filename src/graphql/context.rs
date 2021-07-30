use crate::auth;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Context {
    pub auth_manager: auth::Manager,
    pub user_name: Option<String>,
}

fn parse_bearer_token(token: &str) -> Option<&str> {
    if !token.starts_with("Bearer ") {
        return None;
    }

    Some((&token[7..]).trim())
}

impl Context {
    pub fn new(auth_manager: auth::Manager, bearer_token: Option<String>) -> Self {
        let user_name: Option<String> = bearer_token
            .as_deref()
            .and_then(parse_bearer_token)
            .and_then(|token| auth_manager.username_from_token(token).ok());

        Context {
            auth_manager,
            user_name,
        }
    }
}

impl juniper::Context for Context {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_beader_token() {
        assert_eq!(
            parse_bearer_token("72a2ea42-edae-4379-a7e0-f79cc5855b33"),
            None
        );
        assert_eq!(
            parse_bearer_token("Bearer 31343d47-99c7-46cc-a8bb-b7a300965063"),
            Some("31343d47-99c7-46cc-a8bb-b7a300965063")
        );
        assert_eq!(
            parse_bearer_token("Bearer    31343d47-99c7-46cc-a8bb-b7a300965063   "),
            Some("31343d47-99c7-46cc-a8bb-b7a300965063")
        );
    }
}
