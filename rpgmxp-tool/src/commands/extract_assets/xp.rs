use super::extract_arraylike;
use super::extract_map_infos;
use super::extract_ruby_data;
use super::extract_scripts;
use super::FileEntry;
use super::Options;
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
use std::path::PathBuf;

pub fn extract(
    options: &Options,
    entry: &mut FileEntry<'_>,
    relative_path_components: Vec<&str>,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rxdata"] if !options.skip_extract_scripts => {
            extract_scripts(entry, output_path)?;
        }
        ["Data", "CommonEvents.rxdata"] if !options.skip_extract_common_events => {
            extract_arraylike::<CommonEvent>(entry, output_path)?;
        }
        ["Data", "Actors.rxdata"] if !options.skip_extract_actors => {
            extract_arraylike::<Actor>(entry, output_path)?;
        }
        ["Data", "Weapons.rxdata"] if !options.skip_extract_weapons => {
            extract_arraylike::<Weapon>(entry, output_path)?;
        }
        ["Data", "Armors.rxdata"] if !options.skip_extract_armors => {
            extract_arraylike::<Armor>(entry, output_path)?;
        }
        ["Data", "Skills.rxdata"] if !options.skip_extract_skills => {
            extract_arraylike::<Skill>(entry, output_path)?;
        }
        ["Data", "States.rxdata"] if !options.skip_extract_states => {
            extract_arraylike::<State>(entry, output_path)?;
        }
        ["Data", "Items.rxdata"] if !options.skip_extract_items => {
            extract_arraylike::<Item>(entry, output_path)?;
        }
        ["Data", "Enemies.rxdata"] if !options.skip_extract_enemies => {
            extract_arraylike::<Enemy>(entry, output_path)?;
        }
        ["Data", "Classes.rxdata"] if !options.skip_extract_classes => {
            extract_arraylike::<Class>(entry, output_path)?;
        }
        ["Data", "Troops.rxdata"] if !options.skip_extract_troops => {
            extract_arraylike::<Troop>(entry, output_path)?;
        }
        ["Data", "Tilesets.rxdata"] if !options.skip_extract_tilesets => {
            extract_arraylike::<Tileset>(entry, output_path)?;
        }
        ["Data", "MapInfos.rxdata"] if !options.skip_extract_map_infos => {
            extract_map_infos(entry, output_path)?;
        }
        ["Data", "System.rxdata"] if !options.skip_extract_system => {
            extract_ruby_data::<rpgmxp_types::System>(entry, output_path)?;
        }
        ["Data", "Animations.rxdata"] if !options.skip_extract_animations => {
            extract_arraylike::<Animation>(entry, output_path)?;
        }
        ["Data", file]
            if !options.skip_extract_maps && crate::util::is_map_file_name(file, "rxdata") =>
        {
            extract_ruby_data::<rpgmxp_types::Map>(entry, output_path)?;
        }
        _ => {
            let temp_path = nd_util::with_push_extension(&output_path, "temp");
            // TODO: Lock?
            // TODO: Drop delete guard for file?
            let mut output_file = File::create(&temp_path)
                .with_context(|| format!("failed to open file at \"{}\"", output_path.display()))?;

            std::io::copy(entry, &mut output_file)?;
            std::fs::rename(&temp_path, &output_path)?;
        }
    }

    Ok(())
}
