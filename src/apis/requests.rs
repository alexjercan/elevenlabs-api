use crate::{ApiResult, Elevenlabs, Error, Json};
use log::{debug, error, info};

pub trait Requests {
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Vec<u8>>;
}

impl Requests for Elevenlabs {
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Vec<u8>> {
        info!("===> 🚀\n\tPost api: {sub_url}, body: {body}");

        let req = self
            .agent
            .post(&(self.api_url.clone() + sub_url))
            .set("accept", "audio/mpeg")
            .set("xi-api-key", &self.auth.api_key)
            .set("Content-Type", "application/json");

        println!("{:?}", req);

        let response = req.send_json(&body);

        handle_response(response, sub_url)
    }
}

fn handle_response(
    response: Result<ureq::Response, ureq::Error>,
    sub_url: &str,
) -> ApiResult<Vec<u8>> {
    match response {
        Ok(resp) => {
            debug!("<== ✔️\n\tDone api: {sub_url}");

            assert!(resp.has("Content-Length"));
            let len = resp
                .header("Content-Length")
                .unwrap()
                .parse::<usize>()
                .map_err(|e| Error::RequestError(e.to_string()))?;

            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            resp.into_reader()
                .read_to_end(&mut bytes)
                .map_err(|e| Error::RequestError(e.to_string()))?;
            return Ok(bytes);
        }
        Err(err) => {
            error!("<== ❌\n\tError api: {sub_url}");
            return Err(Error::from(err));
        }
    }
}
