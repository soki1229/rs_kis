mod token;
pub use token::{AccessToken, TokenProvider};

// Define a trait for credential-related operations
pub trait CredentialProvider {
    fn app_key(&self) -> &str;
    fn app_secret(&self) -> &str;
    fn account_num(&self) -> &str;
    fn approval_key_mut(&mut self) -> &mut String;
    fn access_token_mut(&mut self) -> &mut AccessToken;
    fn token(&self) -> &str;
}

pub struct Credentials {
    app_key: String,
    app_secret: String,
    account_num: String,
    approval_key: String,
    access_token: AccessToken,
}

impl Credentials {
    pub fn new(app_key: String, app_secret: String, account_num: String) -> Self {
        Self {
            app_key,
            app_secret,
            account_num,
            approval_key: String::new(),
            access_token: AccessToken::new(),
        }
    }
}

// Implement the CredentialProvider trait for Credentials
impl CredentialProvider for Credentials {
    fn app_key(&self) -> &str {
        &self.app_key
    }

    fn app_secret(&self) -> &str {
        &self.app_secret
    }

    fn account_num(&self) -> &str {
        &self.account_num
    }

    fn approval_key_mut(&mut self) -> &mut String {
        &mut self.approval_key
    }

    fn access_token_mut(&mut self) -> &mut AccessToken {
        &mut self.access_token
    }

    fn token(&self) -> &str {
        &self.access_token.access_token()
    }
}
