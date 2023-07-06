// Converts text into speech using a voice of your choice and returns audio.
// See: https://docs.elevenlabs.io/api-reference/text-to-speech

//! TTS API

use crate::{ApiResult, Elevenlabs};
use serde::{Deserialize, Serialize};

use super::{requests::Requests, TEXT_TO_SPEECH};

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsBody {
    /// Identifier of the model that will be used.
    /// Defaults to "eleven_monolingual_v1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// The text that will get converted into speech.
    pub text: String,
    /// Voice settings overriding stored setttings for the given voice.
    /// They are applied only on the given TTS request.
    /// Defaults to None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Similarity Boost
    pub stability: f32,
    /// Stability
    pub similarity_boost: f32,
}

pub trait TtsApi {
    /// Creates a bytes array containing the audio of the text.
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
            voice_settings: None,
        };

        let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
        let bytes = tts_result.unwrap();
        assert!(bytes.len() > 0);
    }
}
