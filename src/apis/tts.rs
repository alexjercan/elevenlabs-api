use crate::{ApiResult, Elevenlabs};
use serde::{Deserialize, Serialize};

use super::{requests::Requests, TEXT_TO_SPEECH};

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    pub text: String,
    pub voice_settings: VoiceSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceSettings {
    pub stability: f32,
    pub similarity_boost: f32,
}

pub trait TtsApi {
    fn tts(&self, voice_settings: &TtsBody, voice_id: impl AsRef<str>) -> ApiResult<Vec<u8>>;
}

impl TtsApi for Elevenlabs {
    fn tts(&self, tts_body: &TtsBody, voice_id: impl AsRef<str>) -> ApiResult<Vec<u8>> {
        let request_body = serde_json::to_value(tts_body).unwrap();
        self.post(
            format!("{}/{}", TEXT_TO_SPEECH, voice_id.as_ref()).as_str(),
            request_body,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tts::*;
    use crate::*;

    #[test]
    fn test_tts() {
        let auth = Auth::from_env().unwrap();
        let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1/");

        let tts_body = TtsBody {
            model_id: None,
            text: "Hello world".to_string(),
            voice_settings: VoiceSettings {
                stability: 0.5,
                similarity_boost: 0.5,
            },
        };

        let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
        let bytes = tts_result.unwrap();
        assert!(bytes.len() > 0);
    }
}
