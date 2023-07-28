// use std::error::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


pub trait URLFromString <T>{
  fn from_string(s: &str) -> Result<T, Box<dyn std::error::Error>>;
}


pub enum HttpProtocol{
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

impl URLFromString<HttpProtocol> for HttpProtocol{

  fn from_string(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
      let parts: Vec<_> = s.split("://").collect();
      match parts.get(0){
        Some(&"http") => Ok(HttpProtocol::HTTP("http://".to_string())),
        Some(&"https") => Ok(HttpProtocol::HTTPS("https://".to_string())),
        _ => Err(Box::new(ProtocolParserError))
      }

  }
}

impl HttpProtocol{

}




pub struct URL {
    pub url: String,
    pub protocol: Option<HttpProtocol>,
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
      protocol: Some(HttpProtocol::HTTP("http://".to_string())),
      host:Some(IpAddr::V4(Ipv4Addr::new(0,0,0,0))),
      port:String::new(),
      domain:String::new()
    }

  
  }




}