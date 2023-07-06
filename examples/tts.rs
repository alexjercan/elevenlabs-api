// import the dependencies
use elevenlabs_api::{
    tts::{TtsApi, TtsBody},
    *,
};

fn main() {
    // Load API key from environment ELEVENLABS_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::from_env().unwrap();
    let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1/");

    // Create the tts body.
    let tts_body = TtsBody {
        model_id: None,
        text: "Hello world".to_string(),
        voice_settings: None,
    };

    // Generate the speech for the text by using the voice with id yoZ06aMxZJJ28mfd3POQ.
    let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
    let bytes = tts_result.unwrap();

    // Do what you need with the bytes.
    // The server responds with "audio/mpeg" so we can save as mp3.
    std::fs::write("tts.mp3", bytes).unwrap();
}
