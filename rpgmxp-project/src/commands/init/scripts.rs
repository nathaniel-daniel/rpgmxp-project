use flate2::bufread::ZlibDecoder;
use std::collections::HashSet;
use std::io::Read;

#[derive(Debug)]
pub struct Script {
    #[allow(dead_code)]
    pub id: i32,
    pub name: String,
    pub data: String,
}

impl<'a> ruby_marshal::FromValue<'a> for Script {
    fn from_value(
        arena: &'a ruby_marshal::ValueArena,
        handle: ruby_marshal::ValueHandle,
        visited: &mut HashSet<ruby_marshal::ValueHandle>,
    ) -> Result<Self, ruby_marshal::FromValueError> {
        let script: rpgmxp_types::Script =
            ruby_marshal::FromValue::from_value(arena, handle, visited)?;

        let mut decoder = ZlibDecoder::new(&*script.data);
        let mut data = String::new();
        decoder
            .read_to_string(&mut data)
            .map_err(|error| ruby_marshal::FromValueError::Other {
                error: error.into(),
            })?;

        Ok(Self {
            id: script.id,
            name: script.name,
            data,
        })
    }
}
