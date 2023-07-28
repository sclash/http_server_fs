
// mod urls;
// pub use urls::URL;

// mod URL;


use crate::URL::HttpProtocol;
use crate::URL::urls::URLFromString;

#[cfg(test)]
mod tests {
    use crate::URL::{HttpProtocol, URLFromString};

    #[test]
    fn test_it_works(){
        assert_eq!(2+2, 4);
    }

    #[test]
    fn get_protocol(){

        let c = HttpProtocol::from_string("http://example.com");
        println!("{:?}", c.as_ref().unwrap());
        assert_eq!(c.unwrap(), HttpProtocol::HTTP("http://".to_string()));
    }
}


