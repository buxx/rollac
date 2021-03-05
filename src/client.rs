use reqwest;
use reqwest::blocking::Response;

use crate::ac::{animated_corpse_from_value, AnimatedCorpse};
use crate::model;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum ClientError {
    NotFound { message: String },
    ClientSideError { message: String },
    ServerSideError { message: String },
    InternalError { message: String },
}

impl Error for ClientError {}

impl ClientError {
    pub fn get_message(client_error: &ClientError) -> String {
        return match client_error {
            ClientError::NotFound { message } => format!("Not found: {}", message).to_string(),
            ClientError::ClientSideError { message } => {
                format!("Client side error: {}", message).to_string()
            }
            ClientError::ServerSideError { message } => {
                format!("Server side error: {}", message).to_string()
            }
            ClientError::InternalError { message } => {
                format!("Internal error: {}", message).to_string()
            }
        };
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        Self::InternalError {
            message: format!("{}", err),
        }
    }
}

// impl From<NoneError> for ClientError {
//     fn from(_: NoneError) -> Self {
//         Self::InternalError
//     }
// }

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ClientError::get_message(&self))
    }
}

#[derive(Clone)]
pub struct Client {
    server_ip: String,
    server_port: u16,
    secure: bool,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(server_ip: &str, server_port: u16, secure: bool) -> Self {
        Self {
            server_ip: String::from(server_ip),
            server_port,
            secure,
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_base_path(&self) -> String {
        let protocol = if self.secure { "https" } else { "http" };
        return format!("{}://{}:{}", protocol, self.server_ip, self.server_port);
    }

    fn check_response(&self, response: Response) -> Result<Response, ClientError> {
        if response.status().as_u16() == 404 {
            return Err(ClientError::NotFound {
                message: "Not Found".to_string(),
            });
        }

        if response.status().is_client_error() {
            let error: ErrorResponse = response.json()?;
            return Err(ClientError::ClientSideError {
                message: error.message,
            });
        }

        if !response.status().is_success() {
            let error: ErrorResponse = response.json()?;
            return Err(ClientError::ServerSideError {
                message: error.message,
            });
        }

        Ok(response)
    }

    pub fn get_animated_corpses(
        &self,
        world_row_i: u32,
        world_col_i: u32,
    ) -> Result<Vec<Box<dyn AnimatedCorpse + Send + Sync>>, ClientError> {
        let url = format!(
            "{}/ac/?world_row_i={}&world_col_i={}",
            self.get_base_path(),
            world_row_i,
            world_col_i
        );
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;

        let value = response.json::<Value>()?;
        let mut animated_corpses: Vec<Box<dyn AnimatedCorpse + Send + Sync>> = vec![];
        for item in value.as_array().expect("No array found in response").iter() {
            match animated_corpse_from_value(item.clone()) {
                Ok(animated_corpse) => {
                    animated_corpses.push(animated_corpse);
                }
                Err(msg) => {
                    eprintln!("{}", msg)
                }
            }
        }
        Ok(animated_corpses)
    }

    pub fn get_zone_characters(
        &self,
        world_row_i: u32,
        world_col_i: u32,
    ) -> Result<Vec<model::Character>, ClientError> {
        let url = format!(
            "{}/zones/{}/{}/characters",
            self.get_base_path(),
            world_row_i,
            world_col_i
        );
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;

        Ok(response.json::<Vec<model::Character>>()?)
    }

    pub fn get_zone_builds(
        &self,
        world_row_i: u32,
        world_col_i: u32,
    ) -> Result<Vec<model::Build>, ClientError> {
        let url = format!(
            "{}/zones/{}/{}/builds",
            self.get_base_path(),
            world_row_i,
            world_col_i
        );
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;

        Ok(response.json::<Vec<model::Build>>()?)
    }

    pub fn get_world_source(&self) -> Result<String, ClientError> {
        let url = format!("{}/world/source", self.get_base_path(),);
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;

        Ok(response.text()?)
    }

    pub fn get_zone_source(
        &self,
        world_row_i: u32,
        world_col_i: u32,
    ) -> Result<String, ClientError> {
        let url = format!(
            "{}/zones/{}/{}",
            self.get_base_path(),
            world_row_i,
            world_col_i
        );
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;
        let response_value = response.json::<Value>()?;
        match response_value["raw_source"].as_str() {
            None => {
                return Err(ClientError::InternalError {
                    message: "Response do not contains raw_source key".to_string(),
                })
            }
            Some(raw_source) => Ok(raw_source.to_string()),
        }
    }

    pub fn get_tiles_data(&self) -> Result<Value, ClientError> {
        let url = format!("{}/zones/tiles", self.get_base_path());
        let response: Response = self.check_response(self.client.get(url.as_str()).send()?)?;

        Ok(response.json::<Value>()?)
    }
}
