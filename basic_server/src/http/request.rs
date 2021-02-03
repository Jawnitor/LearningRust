use super::method::E_Method;

pub struct Request {
	path: String,
	query_string: Option<String>, //posible NULL,
	method: E_Method,
}
