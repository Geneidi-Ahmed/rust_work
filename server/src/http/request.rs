use super::method::Method;

pub struct Request{
    path: String,
    query: Option<String>,  /* option is an internal Data type provided by STD */
    method: Method,         /* to handle the absence of the value same way as we handle null pointer exceptions*/
}
