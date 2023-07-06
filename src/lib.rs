#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

//!<div align="center">
//! <!-- Build -->
//! <img src="https://img.shields.io/github/actions/workflow/status/alexjercan/elevenlabs-api/rust.yml?style=flat-square"
//! alt="Github Worflow Status" />
//! <!-- Version -->
//! <a href="https://crates.io/crates/elevenlabs-api">
//!   <img src="https://img.shields.io/crates/v/elevenlabs-api?style=flat-square"
//!   alt="Crates.io version" />
//! </a>
//! <!-- Docs -->
//! <a href="https://docs.rs/elevenlabs-api">
//!   <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//!     alt="docs.rs docs" />
//! </a>
//! <!-- Downloads -->
//! <a href="https://crates.io/crates/elevenlabs-api">
//!   <img src="https://img.shields.io/crates/d/elevenlabs-api?style=flat-square"
//!     alt="Crates.io downloads" />
//! </a>
//! <!-- License -->
//! <a href="https://github.com/alexjercan/elevenlabs-api/blob/master/LICENSE">
//!   <img src="https://img.shields.io/github/license/alexjercan/elevenlabs-api?style=flat-square"
//!     alt="Crates.io downloads" />
//! </a>
//!</div>
//!
//! A very simple Rust library for Elevenlabs API, free from complex async operations and redundant dependencies. Inspired by [openai-api](https://github.com/openai-rs/openai-api).
//!
//! ## API
//!
//! Check the [official](https://docs.elevenlabs.io/api-reference/quick-start/introduction) API reference.
//!
//! |API|Support|
//! |---|---|
//! |Text to Speech|✔️|
//! |Text to Speech Stream|❌|
//! |Models|❌|
//! |Voices|❌|
//! |Samples|❌|
//! |History|❌|
//! |User|❌|
//!
//! ## Usage
//!
//! Install the library using the Cargo.toml file.
//!
//! Export your API key into the environment variables
//!
//! ```console
//! export ELEVENLABS_API_KEY=...
//! ```
//!
//! Then use the crate in your Rust code:
//!
//! ```no_run
//! # fn main() {
//!   // import the dependencies
//!   use elevenlabs_api::{
//!       tts::{TtsApi, TtsBody},
//!       *,
//!   };
//!
//!   // Load API key from environment ELEVENLABS_API_KEY.
//!   // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
//!   let auth = Auth::from_env().unwrap();
//!   let elevenlabs = Elevenlabs::new(auth, "https://api.elevenlabs.io/v1/");
//!
//!   // Create the tts body.
//!   let tts_body = TtsBody {
//!       model_id: None,
//!       text: "Hello world".to_string(),
//!       voice_settings: None,
//!   };
//!
//!   // Generate the speech for the text by using the voice with id yoZ06aMxZJJ28mfd3POQ.
//!   let tts_result = elevenlabs.tts(&tts_body, "yoZ06aMxZJJ28mfd3POQ");
//!   let bytes = tts_result.unwrap();
//!
//!   // Do what you need with the bytes.
//!   // The server responds with "audio/mpeg" so we can save as mp3.
//!   std::fs::write("tts.mp3", bytes).unwrap();
//! # }
//! ```
//!
//! Check the [examples](examples) folder.
//!

pub mod apis;
pub use apis::*;

pub mod elevenlabs;
pub use elevenlabs::*;
