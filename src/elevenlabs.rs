use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Auth {
    pub api_key: String,
}

impl Auth {
    pub fn new(api_key: &str) -> Auth {
        Auth {
            api_key: api_key.to_string(),
        }
    }

    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("ELEVENLABS_API_KEY")
            .map_err(|_| "Missing ELEVENLABS_API_KEY".to_string())?;
        Ok(Self { api_key })
    }
}

#[derive(Clone, Debug)]
pub struct Elevenlabs {
    pub auth: Auth,
    pub api_url: String,
    pub(crate) agent: Agent,
}

impl Elevenlabs {
    pub fn new(auth: Auth, api_url: &str) -> Elevenlabs {
        Elevenlabs {
            auth,
            api_url: api_url.to_string(),
            agent: AgentBuilder::new().build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test_elevenlabs() {
        let auth = Auth::from_env().unwrap();
        let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1");

        assert_eq!(elevenlabs.api_url, "https://api.elevenlabs.io/v1");
    }
}
