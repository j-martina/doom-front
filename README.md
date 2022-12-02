# `doom-front` and `doom-lsp`

## About

`doom-front` is meant to be a comprehensive suite of Rust frontends for the myriad domain-specific languages used by the collective ecosystem of Doom source ports, including those of the [ZDoom](https://zdoom.org/index) family, [Eternity Engine](https://eternity.youfailit.net/wiki/Main_Page), DeHackEd, and UMAPINFO.

`doom-lsp` is meant to build off of `doom-front` to create a [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)-compliant language server for assisting Doom development and modification.

The former is also intended to help in facilitating the development of future, backwards-compatible source ports in Rust.

## Licensing

All crates herein may be used under either the [Apache 2.0 License](./LICENSE-APACHE) or the [MIT License](./LICENSE-MIT), at your choosing.

## Attribution

ZDoom and GZDoom are the creations of the ZDoom and GZDoom teams, principally led by Marisa Heit ([@rheit](https://github.com/rheit)) and Christoph Oelckers ([@coelckers](https://github.com/coelckers)). See https://github.com/ZDoom/gzdoom/graphs/contributors for more information.

The Eternity Engine is the creation of Team Eternity, principally led by James "Quasar" Haley ([@haleyjd](https://github.com/haleyjd)), Ioan Chera ([@ioan-chera](https://github.com/ioan-chera)), and Max Waine ([@Altazimuth](https://github.com/Altazimuth)). See https://github.com/team-eternity/eternity/graphs/contributors for more information.

DeHackEd is the creation of Greg "Tree" Lewis.

UMAPINFO is the creation of the following authors:
- Christoph Oelckers
- Xaser Acheron ([@XaserAcheron](https://github.com/XaserAcheron))
- [@JadingTsunami](https://github.com/JadingTsunami)
- Fabian Greffrath ([@fabiangreffrath](https://github.com/fabiangreffrath))
- Roman Fomin ([@rfomin](https://github.com/rfomin))
- [@Shadow-Hog](https://github.com/Shadow-Hog)

---

- bitflags
	- By Ashley Mannix ([@KodrAus](https://github.com/KodrAus)) et. al.
	- Provided under the [Apache 2.0 License](https://github.com/bitflags/bitflags/blob/main/LICENSE-APACHE) and [MIT License](https://github.com/bitflags/bitflags/blob/main/LICENSE-MIT).
	- https://docs.rs/bitflags/latest/bitflags/
- pest
	- By the [pest-parser](https://github.com/pest-parser) organisation, Dragoș Tiselice ([@dragostis](https://github.com/dragostis)), et. al.
	- Provided under the [Apache 2.0 License](https://github.com/pest-parser/pest/blob/master/LICENSE-APACHE) and [MIT License](https://github.com/pest-parser/pest/blob/master/LICENSE-MIT).
	- https://docs.rs/pest/latest/pest/
