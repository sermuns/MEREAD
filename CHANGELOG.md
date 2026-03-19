# Changelog

## [0.5.1](https://github.com/sermuns/MEREAD/compare/v0.5.0..0.5.1) - 2026-03-19

### 🚀 Features

- nixpkgs support ❄️ (#11) by @juneb125 in [#11](https://github.com/sermuns/MEREAD/pull/11)
- begin adding support for alerts #5 by @sermuns in [39e2a68](https://github.com/sermuns/MEREAD/commit/39e2a68c12ea3b53be01b82bd8c2788e8aff2bec)
- fully implemented alerts, with icons by @sermuns in [e5ae1b2](https://github.com/sermuns/MEREAD/commit/e5ae1b2151d326b3436c8efd2a46aca2b7e4b6a6)

### 🐛 Bug Fixes

- idiotic bug with wrong path to font assets... by @sermuns in [99daf5a](https://github.com/sermuns/MEREAD/commit/99daf5af5ef9a3b5330d2c62a69176d54c117639)

### 🚜 Refactor

- simplify comrak config creation by @sermuns in [921634c](https://github.com/sermuns/MEREAD/commit/921634c6446415041d6f129bcef957ca51b23342)
- merge imports, fix bug when targeting file by @sermuns in [65136fb](https://github.com/sermuns/MEREAD/commit/65136fbf4d7fc7d477aa3a3d1ad5513d66441f49)
- simplify asset get logic by @sermuns in [9075602](https://github.com/sermuns/MEREAD/commit/907560239dc2050861b08dc6909ac919aa237826)
- move export into own mod by @sermuns in [2ec86ca](https://github.com/sermuns/MEREAD/commit/2ec86ca81c98bc719073d6c23904322af989014b)
- minor changes to main by @sermuns in [e4db101](https://github.com/sermuns/MEREAD/commit/e4db10171e68de2c34b038e3ce7662fb59a6b716)
- use singlethreaded tokio by @sermuns in [082223b](https://github.com/sermuns/MEREAD/commit/082223bdec645cbdf446c6be201adbe347a97371)

### ⚙️ Miscellaneous Tasks

- move themes to own directory by @sermuns in [8926aaf](https://github.com/sermuns/MEREAD/commit/8926aaf73980efc10ce0419a01b4adeffd39ab6c)
- update deps, cut down on deps by @sermuns in [d47c696](https://github.com/sermuns/MEREAD/commit/d47c6966563d89d5821e683baa97182216bf8390)
- install color-eyre by @sermuns in [ccb1a8c](https://github.com/sermuns/MEREAD/commit/ccb1a8cdb1950eb813bc6497ba595e48be7fe911)
- sort deps by @sermuns in [62174c4](https://github.com/sermuns/MEREAD/commit/62174c43c0459694c416316f9f74b2ec205139f9)
- update lockfile by @sermuns in [2c4a615](https://github.com/sermuns/MEREAD/commit/2c4a615d4cbd76c20b37e7ea563a4c8c61008f53)
- remove unused bacon.toml by @sermuns in [50ea7df](https://github.com/sermuns/MEREAD/commit/50ea7df1f651d2836221dae8839ffdcb347bf46c)
- remove unused import by @sermuns in [7beb692](https://github.com/sermuns/MEREAD/commit/7beb692033dc3cdc8a83158fe1358ef96ddd6ebd)
## [v0.5.0](https://github.com/sermuns/MEREAD/compare/v0.4.0..v0.5.0) - 2026-01-16

### 🚀 Features

- add manpage generation, closing #4 by @sermuns in [230e612](https://github.com/sermuns/MEREAD/commit/230e6129a2e0574631370a131bbfd9706cac2232)

### 📚 Documentation

- document manpage generation by @sermuns in [d5318b9](https://github.com/sermuns/MEREAD/commit/d5318b9a4b4481c4f45991282ed4b59df8dfda11)

### ⚙️ Miscellaneous Tasks

- Release meread version 0.5.0 by @sermuns in [b434f48](https://github.com/sermuns/MEREAD/commit/b434f48d0fbd97259d1fb7333de81cf007023324)
## [v0.4.0](https://github.com/sermuns/MEREAD/compare/v0.3.2..v0.4.0) - 2026-01-09

### 🚀 Features

- add math support by @sermuns in [db58536](https://github.com/sermuns/MEREAD/commit/db58536fd67ecb2ece3dc65bd5c40f1e6cb20c74)

### 💼 Other

- **(deps)** bump dependencies by @sermuns in [817ee8b](https://github.com/sermuns/MEREAD/commit/817ee8b2e29da126ec826ab65e92fd6ae030df8c)

### 🚜 Refactor

- change from anyhow->color_eyre, deny unwrap_used, minor cleanup by @sermuns in [3572a95](https://github.com/sermuns/MEREAD/commit/3572a9506be4ad876e780e6ee163d520db649f49)
- try to fix past sins. by @sermuns in [d98f173](https://github.com/sermuns/MEREAD/commit/d98f173350d4fd3de79e235c077947810691524b)

### ⚡ Performance

- more optimization in release build by @sermuns in [e334f4c](https://github.com/sermuns/MEREAD/commit/e334f4c78d30bc07c2404e03d9c907fd8df9a1d8)

### 🎨 Styling

- use lib.rs file, reorder imports by @sermuns in [b0694d5](https://github.com/sermuns/MEREAD/commit/b0694d5018da5a51b26dd28c571426c88428f69a)

### ⚙️ Miscellaneous Tasks

- remove unused imports by @sermuns in [2113826](https://github.com/sermuns/MEREAD/commit/2113826615622f3e2375b48173f33f3a5b45dfd5)
- Release meread version 0.4.0 by @sermuns in [290864d](https://github.com/sermuns/MEREAD/commit/290864d1c5d5d302e4b94816af003a812d1286b1)
## [v0.3.2](https://github.com/sermuns/MEREAD/compare/v0.3.1..v0.3.2) - 2025-12-02

### 💼 Other

- add docker and publish action by @sermuns in [9d2bd84](https://github.com/sermuns/MEREAD/commit/9d2bd84076e45cf3fb5ddaf114670ec03ecd6cec)

### ⚙️ Miscellaneous Tasks

- Release meread version 0.3.2 by @sermuns in [8f8c78a](https://github.com/sermuns/MEREAD/commit/8f8c78aac6b650441e511330078b77f6508f000b)
## [v0.3.1](https://github.com/sermuns/MEREAD/compare/v0.3.0..v0.3.1) - 2025-12-02

### 💼 Other

- add more config for git cliff/release by @sermuns in [f0e3bfa](https://github.com/sermuns/MEREAD/commit/f0e3bfa23aa3416c2c7b234623c63d217928ae4e)

### 🚜 Refactor

- Split up stuff by purpose, so that it's not all in `src/main.rs` by @juneb125 in [6b5e7bd](https://github.com/sermuns/MEREAD/commit/6b5e7bde87d01ed336bbfb64372043a5e3d3d1a3)
- Move all reloading logic to `src/reload.rs` by @juneb125 in [ae78409](https://github.com/sermuns/MEREAD/commit/ae78409eb1737a38685b5eca2165d646cdab69d2)
- Remove `pub` keyword from things that don't need it by @juneb125 in [a2de080](https://github.com/sermuns/MEREAD/commit/a2de0806732fc9b99cc44cf7309a77a652bf2a89)

### ⚡ Performance

- Move imports to the lowest scope they need (I think this fits under the 'perf' [aka performance] type) by @juneb125 in [20a29b5](https://github.com/sermuns/MEREAD/commit/20a29b5ee319f2c7e0092f972cd697ac904f1e23)

### 🎨 Styling

- Be explicit with crate imports in `src/main.rs` by @juneb125 in [8fb2a44](https://github.com/sermuns/MEREAD/commit/8fb2a4427fefb439366338e541d3c597ba477967)

### ⚙️ Miscellaneous Tasks

- remove commented code by @sermuns in [1930e97](https://github.com/sermuns/MEREAD/commit/1930e97bb0e00a2552cbffde77949e745ad1c9b0)
- Release meread version 0.3.1 by @sermuns in [1c464db](https://github.com/sermuns/MEREAD/commit/1c464db80ee28765e6d59279557ded3aa2da1d9c)
## [v0.3.0](https://github.com/sermuns/MEREAD/compare/v0.2.1..v0.3.0) - 2025-07-29

### 🚀 Features

- Add (and default to-) darkmode. Can toggle to lightmode with flag. by @sermuns in [2faa721](https://github.com/sermuns/MEREAD/commit/2faa7219055ee77726c8cbdb80208bcfd170b127)

### ⚙️ Miscellaneous Tasks

- Release meread version 0.3.0 by @sermuns in [7ea667c](https://github.com/sermuns/MEREAD/commit/7ea667c2304e9fcde6f621cfc33a9df1a527e085)
## [v0.2.1](https://github.com/sermuns/MEREAD/compare/v0.2.0..v0.2.1) - 2025-07-29

### 🐛 Bug Fixes

- embedded assets now working again by @sermuns in [b50c45f](https://github.com/sermuns/MEREAD/commit/b50c45f63bbb74ae884e6f0a76f409aaaa05ebd7)

### ⚙️ Miscellaneous Tasks

- Release meread version 0.2.1 by @sermuns in [811adb4](https://github.com/sermuns/MEREAD/commit/811adb4f7a87e95ae3b43b713c475cf2d5947d3f)
## [v0.2.0](https://github.com/sermuns/MEREAD/compare/v0.1.4..v0.2.0) - 2025-07-28

### 🚀 Features

- Add syntax highlighting. General refactoring. by @sermuns in [8a3e7d6](https://github.com/sermuns/MEREAD/commit/8a3e7d6d8bde19dbb2615218914a893e2581505c)

### 🚜 Refactor

- add annotations to commands. Add --force to export by @sermuns in [d40b57b](https://github.com/sermuns/MEREAD/commit/d40b57be123f7a1e041697fe72f307c1a77c7aa0)
## [v0.0.1] - 2025-07-05
