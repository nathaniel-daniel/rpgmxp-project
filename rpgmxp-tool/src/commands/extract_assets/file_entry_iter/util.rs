use crate::GameKind;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use object::pe::RT_VERSION;
use object::LittleEndian as LE;
use object::U16;
use object::U32;

#[derive(serde::Deserialize, Debug)]
#[expect(dead_code)]
pub struct AssemblyIdentity {
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "@processorArchitecture")]
    pub processor_architecture: Option<String>,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type")]
    pub type_: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Assembly {
    #[serde(rename = "assemblyIdentity")]
    pub assembly_identity: Option<AssemblyIdentity>,

    pub description: Option<Description>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug)]
#[expect(dead_code)]
struct VersionInfo {
    pub fixed_file_info: Option<FixedFileInfo>,
    pub string_file_info: Option<StringFileInfo>,
}

impl VersionInfo {
    /// See: https://learn.microsoft.com/en-us/windows/win32/menurc/vs-versioninfo
    fn parse<'data, R>(reader: R, offset: &mut u64, expected_size: u64) -> anyhow::Result<Self>
    where
        R: object::read::ReadRef<'data>,
    {
        let start_offset = *offset;

        let _length: U16<LE> = *reader.read(offset).ok().context("failed to read length")?;

        let value_length: U16<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read value length")?;

        let type_: U16<LE> = *reader.read(offset).ok().context("failed to read type")?;
        ensure!(type_.get(LE) == 0, "text version data is not supported");

        let expected_key = "VS_VERSION_INFO\0";
        let key: &[u16] = reader
            .read_slice(offset, expected_key.len())
            .ok()
            .context("failed to read key")?;
        let key = String::from_utf16(key)?;
        ensure!(expected_key == key);

        read_padding(reader, offset)?;

        let value_length_u64 = u64::from(value_length.get(LE));
        let fixed_file_info = if value_length_u64 != 0 {
            ensure!(value_length.get(LE) == 52);
            Some(FixedFileInfo::parse(reader, offset)?)
        } else {
            None
        };

        let read_size = *offset - start_offset;
        ensure!(read_size <= expected_size);
        if read_size == expected_size {
            return Ok(Self {
                fixed_file_info,
                string_file_info: None,
            });
        }

        let mut maybe_string_file_info: Option<Option<StringFileInfo>> = None;
        let string_file_info_key = "StringFileInfo\0";
        let var_file_info_key = "VarFileInfo\0";
        let key_peek_len = std::cmp::min(string_file_info_key.len(), var_file_info_key.len());
        loop {
            read_padding(reader, offset)?;

            let start_offset = *offset;

            let length: U16<LE> = *reader.read(offset).ok().context("failed to read length")?;
            let length = length.get(LE);

            let value_length: U16<LE> = *reader
                .read(offset)
                .ok()
                .context("failed to read value length")?;
            ensure!(value_length.get(LE) == 0);

            let type_: U16<LE> = *reader.read(offset).ok().context("failed to read type")?;
            ensure!(type_.get(LE) == 1);

            let key_bytes: &[u16] = reader
                .read_slice(offset, key_peek_len)
                .ok()
                .context("failed to read key bytes")?;
            let key = String::from_utf16(key_bytes)?;
            if key == string_file_info_key[..key_peek_len] {
                ensure!(maybe_string_file_info.is_none());

                let remaining_key_bytes: &[u16] = reader
                    .read_slice(offset, string_file_info_key.len() - key_peek_len)
                    .ok()
                    .context("failed to read remaining key bytes")?;
                let remaining_key_bytes = String::from_utf16(remaining_key_bytes)?;
                ensure!(string_file_info_key[key_peek_len..] == remaining_key_bytes);

                read_padding(reader, offset)?;

                let mut children = Vec::with_capacity(1);
                loop {
                    let table = StringTable::parse(reader, offset)?;
                    children.push(table);

                    let current_length = *offset - start_offset;
                    ensure!(current_length <= u64::from(length));
                    if current_length == u64::from(length) {
                        break;
                    }
                }

                let string_file_info = StringFileInfo { children };

                maybe_string_file_info = Some(Some(string_file_info));
            } else if key == var_file_info_key[..key_peek_len] {
                // TODO: Parse this
                break;
            } else {
                bail!("unknown key \"{key}\"");
            }
        }
        let string_file_info = maybe_string_file_info.unwrap();

        Ok(Self {
            fixed_file_info,
            string_file_info,
        })
    }
}

#[derive(Debug)]
struct StringFileInfo {
    pub children: Vec<StringTable>,
}

fn read_padding<'data, R>(reader: R, offset: &mut u64) -> anyhow::Result<()>
where
    R: object::read::ReadRef<'data>,
{
    let padding_size = 4 - (*offset % 4);
    if padding_size != 4 {
        let padding = reader
            .read_bytes(offset, padding_size)
            .ok()
            .context("failed to read padding")?;
        ensure!(padding.iter().all(|b| *b == 0));
    }

    Ok(())
}

fn read_utf16_nul_string<'data, R>(reader: R, offset: &mut u64) -> anyhow::Result<String>
where
    R: object::read::ReadRef<'data>,
{
    let mut raw = Vec::new();
    while raw.is_empty() || *raw.last().unwrap() != 0 {
        let value: U16<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read wide char")?;
        raw.push(value.get(LE));
    }

    let value = String::from_utf16(&raw)?;

    Ok(value)
}

#[derive(Debug)]
#[expect(dead_code)]
struct FixedFileInfo {
    struct_version: u32,
    file_version: u64,
    product_version: u64,
    file_flags_mask: u32,
    file_flags: u32,
    file_os: u32,
    file_type: u32,
    file_subtype: u32,
    file_date: u64,
}

impl FixedFileInfo {
    fn parse<'data, R>(reader: R, offset: &mut u64) -> anyhow::Result<Self>
    where
        R: object::read::ReadRef<'data>,
    {
        let signature: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read signature")?;
        ensure!(signature.get(LE) == 0xFEEF04BD);

        let struct_version: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read struct version")?;
        let struct_version = struct_version.get(LE);

        let file_version_ms: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file version ms")?;
        let file_version_ls: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file version ls")?;
        let file_version =
            (u64::from(file_version_ms.get(LE)) << 32) | u64::from(file_version_ls.get(LE));

        let product_version_ms: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read product version ms")?;
        let product_version_ls: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read product version ls")?;
        let product_version =
            (u64::from(product_version_ms.get(LE)) << 32) | u64::from(product_version_ls.get(LE));

        let file_flags_mask: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file flags mask")?;
        let file_flags_mask = file_flags_mask.get(LE);

        let file_flags: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file flags")?;
        let file_flags = file_flags.get(LE);

        let file_os: U32<LE> = *reader.read(offset).ok().context("failed to read file os")?;
        let file_os = file_os.get(LE);

        let file_type: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file type")?;
        let file_type = file_type.get(LE);

        let file_subtype: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file subtype")?;
        let file_subtype = file_subtype.get(LE);

        let file_date_ms: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file date ms")?;

        let file_date_ls: U32<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read file date ls")?;
        let file_date = (u64::from(file_date_ms.get(LE)) << 32) | u64::from(file_date_ls.get(LE));

        Ok(Self {
            struct_version,
            file_version,
            product_version,
            file_flags_mask,
            file_flags,
            file_os,
            file_type,
            file_subtype,
            file_date,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct StringTable {
    pub key: String,
    pub children: Vec<StringStruct>,
}

impl StringTable {
    fn parse<'data, R>(reader: R, offset: &mut u64) -> anyhow::Result<Self>
    where
        R: object::read::ReadRef<'data>,
    {
        let start_offset = *offset;

        let length: U16<LE> = *reader.read(offset).ok().context("failed to read length")?;
        let length = length.get(LE);

        let value_length: U16<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read value length")?;
        ensure!(value_length.get(LE) == 0);

        let type_: U16<LE> = *reader.read(offset).ok().context("failed to read type")?;
        ensure!(type_.get(LE) == 1);

        let key: &[u16] = reader
            .read_slice(offset, 8)
            .ok()
            .context("failed to read key")?;
        let key = String::from_utf16(key)?;
        ensure!(key.bytes().all(|b| b.is_ascii_hexdigit()));
        ensure!(key.len() == 8);

        read_padding(reader, offset)?;

        let mut children = Vec::new();
        loop {
            let string = StringStruct::parse(reader, offset)?;
            children.push(string);

            let current_length = *offset - start_offset;
            ensure!(current_length <= u64::from(length));
            if current_length == u64::from(length) {
                break;
            }

            read_padding(reader, offset)?;
        }

        Ok(Self { key, children })
    }

    /*
    /// Get the language code
    pub fn language(&self) -> u16 {
        u16::from_str_radix(&self.key[..4], 16).unwrap()
    }

    /// Get the code page
    pub fn code_page(&self) -> u16 {
        u16::from_str_radix(&self.key[4..], 16).unwrap()
    }
    */
}

#[derive(Debug)]
struct StringStruct {
    pub key: String,
    pub value: Vec<u16>,
}

impl StringStruct {
    fn parse<'data, R>(reader: R, offset: &mut u64) -> anyhow::Result<Self>
    where
        R: object::read::ReadRef<'data>,
    {
        let start_offset = *offset;

        let length: U16<LE> = *reader.read(offset).ok().context("failed to read length")?;
        let length = length.get(LE);

        let value_length: U16<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read value length")?;
        let value_length = value_length.get(LE);

        let type_: U16<LE> = *reader
            .read(offset)
            .ok()
            .context("failed to read value length")?;
        let type_ = type_.get(LE);
        ensure!(type_ == 1, "unsupported string struct type {type_}");

        let key = read_utf16_nul_string(reader, offset)?;

        read_padding(reader, offset)?;

        let value: &[U16<LE>] = reader
            .read_slice(offset, value_length.into())
            .ok()
            .context("failed to read value")?;
        let value: Vec<u16> = value.iter().map(|value| value.get(LE)).collect();

        ensure!(*offset - start_offset == u64::from(length));

        Ok(Self { key, value })
    }
}

fn guess_from_version_entry(
    game_exe: &[u8],
    section_table: object::read::pe::SectionTable<'_>,
    resource_directory: object::read::pe::ResourceDirectory<'_>,
    root: &object::read::pe::ResourceDirectoryTable<'_>,
) -> anyhow::Result<Option<GameKind>> {
    let entry = root
        .entries
        .iter()
        .find(|entry| entry.name_or_id().id() == Some(RT_VERSION));
    let entry = match entry {
        Some(entry) => entry,
        None => return Ok(None),
    };

    let data = entry.data(resource_directory)?;
    let table = data.table().context("object VERSION data is not a table")?;

    let data = table
        .entries
        .first()
        .context("object VERSION table missing entry 0")?
        .data(resource_directory)?;
    let table = data
        .table()
        .context("object VERSION table entry 0 is not a table")?;

    let data = table
        .entries
        .first()
        .context("object VERSION table entry 0 table missing entry 0")?
        .data(resource_directory)?
        .data()
        .context("object VERSION table entry 0 table entry 0 is not data")?;
    let offset = data.offset_to_data.get(LE);
    let size = usize::try_from(data.size.get(LE))?;
    // let code_page = data.code_page.get(LE);

    let (offset, _) = section_table
        .pe_file_range_at(offset)
        .context("section missing version offset address")?;
    let mut offset = u64::from(offset);
    let version_info = VersionInfo::parse(game_exe, &mut offset, u64::try_from(size)?)?;

    let string_file_info = match version_info.string_file_info.as_ref() {
        Some(string_file_info) => string_file_info,
        None => return Ok(None),
    };

    for table in string_file_info.children.iter() {
        for string in table.children.iter() {
            if string.key != "FileDescription\0" {
                continue;
            }

            // TODO: Can this ever not be UTF16?
            let value = String::from_utf16(&string.value)?;
            match value.as_str() {
                "RGSS Player\0" => return Ok(Some(GameKind::Xp)),
                "RGSS2 Player\0" => return Ok(Some(GameKind::Vx)),
                "RGSS3 Player\0" => return Ok(Some(GameKind::VxAce)),
                _ => {}
            }
        }
    }

    Ok(None)
}

fn guess_from_manifest_entry(
    game_exe: &[u8],
    section_table: object::read::pe::SectionTable<'_>,
    resource_directory: object::read::pe::ResourceDirectory<'_>,
    root: &object::read::pe::ResourceDirectoryTable<'_>,
) -> anyhow::Result<Option<GameKind>> {
    use object::pe::RT_MANIFEST;
    use object::LittleEndian as LE;

    let manifest_entry = root
        .entries
        .iter()
        .find(|entry| entry.name_or_id().id() == Some(RT_MANIFEST));
    let manifest_entry = match manifest_entry {
        Some(manifest_entry) => manifest_entry,
        None => return Ok(None),
    };

    let manifest_entry_data = manifest_entry.data(resource_directory)?;
    let manifest_entry_table = manifest_entry_data
        .table()
        .context("object MANIFEST data is not a table")?;

    let manifest_entry_table_entry_data = manifest_entry_table
        .entries
        .first()
        .context("object MANIFEST table missing entry 0")?
        .data(resource_directory)?;
    let manifest_entry_table_entry_data_table = manifest_entry_table_entry_data
        .table()
        .context("object MANIFEST table entry 0 is not a table")?;

    let manifest_entry_table_entry_data_table_entry_data = manifest_entry_table_entry_data_table
        .entries
        .first()
        .context("object MANIFEST table entry 0 table missing entry 0")?
        .data(resource_directory)?
        .data()
        .context("object MANIFEST table entry 0 table entry 0 is not data")?;
    let manifest_offset = manifest_entry_table_entry_data_table_entry_data
        .offset_to_data
        .get(LE);
    let manifest_size = usize::try_from(
        manifest_entry_table_entry_data_table_entry_data
            .size
            .get(LE),
    )?;
    let code_page = manifest_entry_table_entry_data_table_entry_data
        .code_page
        .get(LE);

    let bytes = &section_table
        .pe_data_at(game_exe, manifest_offset)
        .context("failed to get object manifest bytes")?
        .get(..manifest_size)
        .context("object manifest smaller than declared")?;

    let manifest_string = match code_page {
        0 => {
            // This isn't a real LCID from what I can tell,
            // but rather a null value. Assume ASCII for now.
            // TODO: Detect encoding dynamically?

            std::str::from_utf8(bytes)?.to_string()
        }
        1252 => {
            let (value, _encoding, malformed) = encoding_rs::WINDOWS_1252.decode(bytes);
            ensure!(!malformed);

            value.into()
        }
        _ => bail!("unknown MANIFEST LCID {code_page}"),
    };

    let manifest: Assembly =
        quick_xml::de::from_str(&manifest_string).context("failed to parse manifest string")?;
    if manifest
        .assembly_identity
        .is_some_and(|assembly_identity| assembly_identity.name == "Enterbrain.RGSS.Game")
        && manifest
            .description
            .as_ref()
            .map(|description| description.value.as_str())
            == Some("RGSS Player")
    {
        return Ok(Some(GameKind::Xp));
    }

    Ok(None)
}

/// See: https://learn.microsoft.com/en-us/windows/win32/menurc/resource-types
/// See: https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/6c085406-a698-4e12-9d4d-c3b0ee3dbc4a
pub fn guess_game_kind_from_exe(game_exe: &[u8]) -> anyhow::Result<Option<GameKind>> {
    use object::read::File;

    let file = File::parse(game_exe)?;
    let (section_table, data_directories) = match file {
        File::Pe32(file) => (file.section_table(), file.data_directories()),
        File::Pe64(file) => (file.section_table(), file.data_directories()),
        _ => bail!("unknown object file format {:?}", file.format()),
    };

    let resource_directory = data_directories.resource_directory(game_exe, &section_table)?;
    let resource_directory = match resource_directory {
        Some(resource_directory) => resource_directory,
        None => return Ok(None),
    };

    let root = resource_directory.root()?;

    if let Some(game_kind) =
        guess_from_version_entry(game_exe, section_table, resource_directory, &root)?
    {
        return Ok(Some(game_kind));
    }

    if let Some(game_kind) =
        guess_from_manifest_entry(game_exe, section_table, resource_directory, &root)?
    {
        return Ok(Some(game_kind));
    }

    Ok(None)
}
