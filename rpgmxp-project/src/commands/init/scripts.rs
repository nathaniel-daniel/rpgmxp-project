use anyhow::anyhow;
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
        let script: &ruby_marshal::ArrayValue =
            ruby_marshal::FromValue::from_value(arena, handle, visited)?;
        let script = script.value();

        if script.len() != 3 {
            return Err(ruby_marshal::FromValueError::Other {
                error: anyhow!("script data array len is not 3").into(),
            });
        }

        let id: i32 = ruby_marshal::FromValue::from_value(arena, script[0], visited)?;
        let name: &ruby_marshal::StringValue =
            ruby_marshal::FromValue::from_value(arena, script[1], visited)?;
        let name = std::str::from_utf8(name.value()).map_err(|error| {
            ruby_marshal::FromValueError::Other {
                error: error.into(),
            }
        })?;
        let data: &ruby_marshal::StringValue =
            ruby_marshal::FromValue::from_value(arena, script[2], visited)?;
        let data = data.value();
        let mut decoder = ZlibDecoder::new(data);
        let mut data = String::new();
        decoder
            .read_to_string(&mut data)
            .map_err(|error| ruby_marshal::FromValueError::Other {
                error: error.into(),
            })?;

        Ok(Self {
            id,
            name: name.into(),
            data,
        })
    }
}
