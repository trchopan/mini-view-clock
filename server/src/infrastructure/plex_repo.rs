#[derive(Debug)]
pub enum PlexRepoInsertError {
    DbError,
}

#[derive(Debug)]
pub enum PlexRepoSelectError {
    NotFound,
}

pub struct PlexRepo {
    token: String,
    ignore_addresses: Vec<String>,
}

impl PlexRepo {
    pub fn new(token: String, ignore_addresses: Vec<String>) -> Self {
        let mut ignore_addresses = ignore_addresses;
        ignore_addresses.sort();
        Self {
            token,
            ignore_addresses,
        }
    }

    pub fn check_token(&self, token: &str) -> bool {
        token == self.token
    }

    pub fn check_ignore_address(&self, addr: &str) -> bool {
        self.ignore_addresses
            .binary_search(&addr.to_string())
            .is_ok()
    }
}
