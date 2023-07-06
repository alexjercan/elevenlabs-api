use elevenlabs_api::{
    tts::{TtsApi, TtsBody},
    *,
};

fn main() {
    let auth = Auth::from_env().unwrap();
    let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1/");

    let tts_body = TtsBody {
        model_id: None,
        text: "Hello world".to_string(),
        voice_settings: None,
    };

    let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
    let bytes = tts_result.unwrap();

    std::fs::write("tts.mp3", bytes).unwrap();
}
