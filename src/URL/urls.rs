// use std::error::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


pub trait URLFromString <T>{
  fn from_string(s: &str) -> Result<T, Box<dyn std::error::Error>>;
}


pub enum HTTP_protocol{
  HTTP(String),
  HTTPS(String)
}


#[derive(Debug)]
struct ProtocolParserError;

impl std::fmt::Display for ProtocolParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid protocol format")
    }
}

impl std::error::Error for ProtocolParserError {}

impl URLFromString<HTTP_protocol> for HTTP_protocol{


  fn from_string(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
      let parts: Vec<_> = s.split("://").collect();
      match parts.get(0){
        Some(&"http") => Ok(HTTP_protocol::HTTP("http://".to_string())),
        Some(&"https") => Ok(HTTP_protocol::HTTPS("https://".to_string())),
        _ => Err(Box::new(ProtocolParserError))
      }

  }
}

impl HTTP_protocol{

}




pub struct URL {
    pub url: String,
    pub protocol: Option<HTTP_protocol>,
    pub host: Option<IpAddr>,
    pub port: String,
    pub domain: String,

  }

impl URLFromString<URL> for URL{

  fn from_string(s: &str) -> Result<URL, Box<dyn std::error::Error>> {
      Err(Box::new(ProtocolParserError))
  }
}

impl URL{

  pub fn from_string(s: &str) -> Self{
    URL{
      url: String::new(),
      protocol: Some(HTTP_protocol::HTTP("http://".to_string())),
      host:Some(IpAddr::V4(Ipv4Addr::new(0,0,0,0))),
      port:String::new(),
      domain:String::new()
    }

  
  }




}