


mod URL;

use URL::HttpProtocol;

use URL::urls::URLFromString;


fn main() {
    println!("Hello, world!");
    let c = HttpProtocol::from_string("https://cazzzo");

    println!("{:?}",c);

    let res = c.unwrap();
    println!("{:?}", res);
}



#[cfg(test)]
mod tests {
    #[path = "URL_test/URL_test.rs"]
    mod URL_test;
    #[path = "URL_test/PROTOCOL_test.rs"]
    mod PROTOCOL_test;
}
