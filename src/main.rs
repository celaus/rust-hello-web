
extern crate rouille;


///
/// Returns the default response string for the web server
///
/// # Examples
///
/// ```rust
/// assert_eq!(get_default_response(), "hello world");
/// ```
fn get_default_response() -> &'static str {
    return "hello world";
}

///
/// Returns the port where the web server listens at
///
/// # Examples
///
/// ```rust
/// assert_eq!(get_port(), 8080);
/// ```
fn get_port() -> i32 {
    return 8080
}

fn main() {
    let port = get_port();
    println!("Now listening on 0.0.0.0:{}", port);
    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        rouille::Response::text(get_default_response())
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_response_is_hello_world() {
        assert_eq!(get_default_response(), "hello world");
    }
    #[test]
    fn port_is_8080() {
        assert_eq!(get_port(), 8080);
    }



}