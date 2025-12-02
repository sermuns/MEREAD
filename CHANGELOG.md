# Changelog

## [0.3.1](https://github.com/sermuns/MEREAD/compare/v0.3.0..0.3.1) - 2025-12-02

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
