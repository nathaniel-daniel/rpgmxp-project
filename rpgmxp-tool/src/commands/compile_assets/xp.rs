use super::generate_arraylike_rx_data;
use super::generate_map_infos_data;
use super::generate_ruby_data;
use super::generate_scripts_data;
use super::set_extension_str;
use super::FileSink;
use anyhow::Context;
use rpgmxp_types::Actor;
use rpgmxp_types::Animation;
use rpgmxp_types::Armor;
use rpgmxp_types::Class;
use rpgmxp_types::CommonEvent;
use rpgmxp_types::Enemy;
use rpgmxp_types::Item;
use rpgmxp_types::Skill;
use rpgmxp_types::State;
use rpgmxp_types::Tileset;
use rpgmxp_types::Troop;
use rpgmxp_types::Weapon;
use std::fs::File;
use std::path::Path;

pub fn compile(
    entry_path: &Path,
    entry_file_type: std::fs::FileType,
    relative_path: &Path,
    relative_path_components: Vec<&str>,
    file_sink: &mut FileSink,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let scripts_data = generate_scripts_data(entry_path)?;
            let size = u32::try_from(scripts_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*scripts_data)?;
        }
        ["Data", "Scripts.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "CommonEvents.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<CommonEvent>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "CommonEvents.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Actors.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Actor>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Actors.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Weapons.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Weapon>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Weapons.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Armors.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Armor>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Armors.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Skills.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Skill>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Skills.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "States.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<State>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "States.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Items.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Item>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Items.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Enemies.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Enemy>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Enemies.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Classes.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Class>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Classes.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Troops.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Troop>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Troops.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "Tilesets.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Tileset>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Tilesets.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "MapInfos.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_map_infos_data(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "MapInfos.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "System.json"] if entry_file_type.is_file() => {
            println!("packing \"{}\"", relative_path.display());

            let data = generate_ruby_data::<rpgmxp_types::System>(entry_path)?;
            let size = u32::try_from(data.len())?;

            let mut relative_path_components = relative_path_components.clone();
            *relative_path_components.last_mut().unwrap() = "System.rxdata";

            file_sink.write_file(&relative_path_components, size, &*data)?;
        }
        ["Data", "Animations.rxdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let rx_data = generate_arraylike_rx_data::<Animation>(entry_path)?;
            let size = u32::try_from(rx_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*rx_data)?;
        }
        ["Data", "Animations.rxdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", file] if crate::util::is_map_file_name(file, "json") => {
            println!("packing \"{}\"", relative_path.display());

            let map_data = generate_ruby_data::<rpgmxp_types::Map>(entry_path)?;
            let size = u32::try_from(map_data.len())?;

            let renamed_file = set_extension_str(file, "rxdata");
            let mut relative_path_components = relative_path_components.clone();
            *relative_path_components.last_mut().unwrap() = renamed_file.as_str();

            file_sink.write_file(&relative_path_components, size, &*map_data)?;
        }
        relative_path_components if entry_file_type.is_file() => {
            // Copy file by default
            println!("packing \"{}\"", relative_path.display());

            let input_file = File::open(entry_path).with_context(|| {
                format!(
                    "failed to open input file from \"{}\"",
                    entry_path.display()
                )
            })?;
            let metadata = input_file.metadata()?;
            let size = u32::try_from(metadata.len())?;

            file_sink.write_file(relative_path_components, size, input_file)?;
        }
        _ => {}
    }

    Ok(())
}
