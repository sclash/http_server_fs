
mod URL;
// mod test;



use URL::HttpProtocol;


fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    #[path = "URL_test/URL_test.rs"]
    mod URL_test;
    #[path = "URL_test/PROTOCOL_test.rs"]
    mod PROTOCOL_test;
}

// Expose the test module in the public API
// #[cfg(test)]
// pub use tests::test;

// #[cfg(test)]
// mod tests {


//     #[test]
//     fn it_works(){
//         // HttpProtocol
//     }
// }

// #[cfg(test)]
// mod test;