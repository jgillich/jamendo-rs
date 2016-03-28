use Error;
use ErrorKind;
use Transport;
use Album;
use std::collections::HashMap;

pub struct GetTracks<'a> {
    transport: &'a Transport,
    query_params: HashMap<String, String>,
}
