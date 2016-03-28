use Error;
use ErrorKind;
use Transport;
use Album;
use std::collections::HashMap;

pub struct GetArtists<'a> {
    transport: &'a Transport,
    query_params: HashMap<String, String>,
}
