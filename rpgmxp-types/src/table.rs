use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::UserDefinedValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TableFromValueError {
    TooShort { len: usize },
    OddSizedPayload { len: usize },
    ItemSizeMismatch { expected: i32, actual: usize },
}

impl std::fmt::Display for TableFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TooShort { len } => {
                write!(f, "the table payload (len {len}) is too short")
            }
            Self::OddSizedPayload { len } => {
                write!(f, "the table payload (len {len}) is not a multiple of 2")
            }
            Self::ItemSizeMismatch { expected, actual } => {
                write!(f, "the item array length is mismatched, expected {expected} bytes but got {actual}")
            }
        }
    }
}

impl std::error::Error for TableFromValueError {}

#[derive(Debug)]
pub enum TableIntoValueError {
    TooManyItems {
        len: usize,
        error: std::num::TryFromIntError,
    },
}

impl std::fmt::Display for TableIntoValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TooManyItems { len, .. } => {
                write!(f, "there are too many table items in table of len {len}")
            }
        }
    }
}

impl std::error::Error for TableIntoValueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TooManyItems { error, .. } => Some(error),
            // _ => None,
        }
    }
}

const HEADER_SIZE: usize = 4 * 5;

const USER_DEFINED_NAME: &[u8] = b"Table";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Table {
    pub dimensions: i32,
    pub x_size: i32,
    pub y_size: i32,
    pub z_size: i32,
    pub items: Vec<i16>,
}

impl<'a> FromValue<'a> for Table {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, FromValueError> {
        let user_defined: &UserDefinedValue = FromValue::from_value(arena, handle, visited)?;
        let name = user_defined.name();
        let name = arena
            .get_symbol(name)
            .ok_or(FromValueError::InvalidValueHandle {
                handle: name.into(),
            })?
            .value();

        if name != USER_DEFINED_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let value = user_defined.value();

        let value_len = value.len();
        if value_len < HEADER_SIZE {
            return Err(FromValueError::new_other(TableFromValueError::TooShort {
                len: value_len,
            }));
        }
        if value_len % 2 != 0 {
            return Err(FromValueError::new_other(
                TableFromValueError::OddSizedPayload { len: value_len },
            ));
        }

        let (dimensions, value) = value.split_at(4);
        let dimensions = i32::from_le_bytes(dimensions.try_into().unwrap());

        let (x_size, value) = value.split_at(4);
        let x_size = i32::from_le_bytes(x_size.try_into().unwrap());

        let (y_size, value) = value.split_at(4);
        let y_size = i32::from_le_bytes(y_size.try_into().unwrap());

        let (z_size, value) = value.split_at(4);
        let z_size = i32::from_le_bytes(z_size.try_into().unwrap());

        let (size, value) = value.split_at(4);
        let size = i32::from_le_bytes(size.try_into().unwrap());

        let value_len = value.len();
        if i32::try_from(value_len)
            .ok()
            .map_or(true, |value_len| value_len != 2 * size)
        {
            return Err(FromValueError::new_other(
                TableFromValueError::ItemSizeMismatch {
                    expected: 2 * size,
                    actual: value_len,
                },
            ));
        }

        let items_len = usize::try_from(size).unwrap();
        let mut items = Vec::with_capacity(items_len);
        for chunk in value.chunks(2) {
            items.push(i16::from_le_bytes(chunk.try_into().unwrap()));
        }

        Ok(Self {
            dimensions,
            x_size,
            y_size,
            z_size,
            items,
        })
    }
}

impl IntoValue for Table {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let name = arena.create_symbol(USER_DEFINED_NAME.into());
        let mut value = Vec::with_capacity(HEADER_SIZE + self.items.len());

        value.extend(self.dimensions.to_le_bytes());
        value.extend(self.x_size.to_le_bytes());
        value.extend(self.y_size.to_le_bytes());
        value.extend(self.z_size.to_le_bytes());

        let items_len = self.items.len();
        let size = i32::try_from(items_len).map_err(|error| {
            IntoValueError::new_other(TableIntoValueError::TooManyItems {
                len: items_len,
                error,
            })
        })?;
        value.extend(size.to_le_bytes());

        for item in self.items.iter() {
            value.extend(item.to_le_bytes());
        }

        let user_defined = arena.create_user_defined(name, value);
        Ok(user_defined.into())
    }
}
