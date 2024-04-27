# rpgxp-project

## rpgmxp-tool
`rpgmxp-tool` is a CLI to read and write RPGMaker XP game files.
This includes rgssad and rxdata files.
Currently, it is NOT capable of byte-for-byte round-tripping game files, but repacked files work normally in games.
This is due to differences in compression and the fact that this library does not dedupe game assets as much as it needs to.

## Notes
The following objects can be deduped inside an archive:
 * String (what cases?)
 * MoveCommands
 
## Resources
 * https://github.com/selectivepaperclip/rpgm2renpy/blob/ff847ff9f9a00cabd6f6c894be4c72711d0c76fd/game/rpgm_constants.rpy
 * https://github.com/cstrahan/open-rpg-maker
 * https://www.rpg-maker.fr/dl/monos/aide/xp/index.html