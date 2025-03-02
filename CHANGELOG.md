# Changelog

## [0.8.1](https://github.com/midi2-dev/bl-midi2-rs/compare/0.8.0..0.8.1) - 2025-03-02

### 🐛 Fixes

- Midi2 crate has no readme - ([ca05654..282e1f8](https://github.com/midi2-dev/bl-midi2-rs/compare/ca056540bcb27b57040d8d0860aa4f97a02e0eeb..282e1f8fd938b663be0bc663eabad989b0b3f377))


## [0.8.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.7.0..0.8.0) - 2025-02-28

### ✨ Features

- Adds workflows releases branches - ([c2a40f3](https://github.com/midi2-dev/bl-midi2-rs/compare/97c22bc5ab13aa9cad8ea9f4f900ca7bac6b8dc3..1c662098f0391d5d101432a381db8453d9d5fe6f))
- Adds git cliff config - ([374770c](https://github.com/midi2-dev/bl-midi2-rs/commit/374770c79aefbf6b2514ae86e29de26d38695202))
- Adds cargo deny config (passing) - ([a0f50a8](https://github.com/midi2-dev/bl-midi2-rs/commit/a0f50a85f42b47dda618afc9a7ad150c3d06acb8))
- [**breaking**] Messages implement Copy when the underlying buffer does too - ([9d501ac](https://github.com/midi2-dev/bl-midi2-rs/commit/9d501aca5a6f2be9a21a0a06b251e9efde68dc9b))
- Messages implement Copy when the underlying buffer does too - ([bb3888e](https://github.com/midi2-dev/bl-midi2-rs/commit/bb3888e338abfc88035cc3ea84b49c494e9d466a))
- Rebuffer from exclusive to shared slice for aggregates - ([05af663](https://github.com/midi2-dev/bl-midi2-rs/commit/05af663f4c01c652891b5610871ce44def16c411))
- Rebuffer slice from mut slice - ([cb58c00](https://github.com/midi2-dev/bl-midi2-rs/commit/cb58c00dbe1d2f10860f7a611f5d5ca238007138))
- Adds clippy ci check - ([3444e9b](https://github.com/midi2-dev/bl-midi2-rs/commit/3444e9bd564600a8c61108044913287f5f3e27be))
- Fix typos automatically in the codespell hook - ([1627c4b](https://github.com/midi2-dev/bl-midi2-rs/commit/1627c4bfa0ee18b8e13a3ae197a225df73e87e0e))
- Replace rust git hooks with ci-friendly alternative - ([134cdac](https://github.com/midi2-dev/bl-midi2-rs/commit/134cdac38fe956bc6acbb2cf62fc3b094cf3583a))

### 🐛 Fixes

- Comment typo - ([8d6ddd9](https://github.com/midi2-dev/bl-midi2-rs/commit/8d6ddd9fb98a9dbb92a4416c526b6b83fbc3211c))
- Apply latest clippy lint fixes - ([6e2058e](https://github.com/midi2-dev/bl-midi2-rs/commit/6e2058e1a11432cce003f462c80e937bf99ab0e2))
- Comment typo - ([3537b05](https://github.com/midi2-dev/bl-midi2-rs/commit/3537b0513f98ae11cedeb33d27cdcb8ad471a8da))

### 🛠️ Refactor

- Github actions - ([8d7c599](https://github.com/midi2-dev/bl-midi2-rs/commit/8d7c599185ca879c280f4b73dc0207cc4b2b7cad))
- Restructure the workspace - ([e46ac36](https://github.com/midi2-dev/bl-midi2-rs/commit/e46ac360c50182b2b64e813c6d2c788e8a3f00ca))

### 📚 Documentation

- Correct docs for default features - ([b84e8e5](https://github.com/midi2-dev/bl-midi2-rs/commit/b84e8e5adbf944a8b195c635392106bde141b4a7))
- Add readme badges - ([9d59452](https://github.com/midi2-dev/bl-midi2-rs/commit/9d594520bcbe663c365692329fc41d1c9fccd30d))

## New Contributors ❤️

* @pre-commit-ci[bot] made their first contribution in [#41](https://github.com/midi2-dev/bl-midi2-rs/pull/41)
* @Philipp-M made their first contribution in [#38](https://github.com/midi2-dev/bl-midi2-rs/pull/38)

## [0.7.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.6.5..0.7.0) - 2025-01-18

### 🐛 Fixes

- [**breaking**] Packet from long slice buffers - ([2522b67](https://github.com/midi2-dev/bl-midi2-rs/commit/2522b67f770b277d8d0cbea35205b7c660389526))

### 📚 Documentation

- Update changelog - ([30ea119](https://github.com/midi2-dev/bl-midi2-rs/commit/30ea119582aaad344bf755dfc94e204d22eedb16))


## [0.6.5](https://github.com/midi2-dev/bl-midi2-rs/compare/0.6.4..0.6.5) - 2025-01-16

### 🐛 Fixes

- Compile errors on no default features - ([1f0ae07](https://github.com/midi2-dev/bl-midi2-rs/commit/1f0ae0749fa290b5176ac282b3a51363e6d2b978))

### 📚 Documentation

- Update changelog - ([2fe7c6e](https://github.com/midi2-dev/bl-midi2-rs/commit/2fe7c6ecf3e5cf83d37df879c29d5e8ddae87b6f))
- Improve readme example - ([099229a](https://github.com/midi2-dev/bl-midi2-rs/commit/099229a8576b0bf709d4fdd53f794bf9135cfcec))
- Improve readme example - ([06b6148](https://github.com/midi2-dev/bl-midi2-rs/commit/06b61482fc58a02cf76869bad99cd248f46237f7))


## [0.6.4](https://github.com/midi2-dev/bl-midi2-rs/compare/0.6.3..0.6.4) - 2025-01-13

### 🐛 Fixes

- *(system_common)* Fix realtime messages buffer length - ([f553e5f](https://github.com/midi2-dev/bl-midi2-rs/commit/f553e5fa0d7bf9b4a793624f4311b20afe6a9908))
- *(system_common)* Fix realtime messages buffer length - ([96a7ecb](https://github.com/midi2-dev/bl-midi2-rs/commit/96a7ecbe294f1c6eac5c98eee013121394f748a5))

### 📚 Documentation

- Update changelog - ([22091c3](https://github.com/midi2-dev/bl-midi2-rs/commit/22091c394c690a63f56bd4698098f9d42d3be2bb))


## [0.6.3](https://github.com/midi2-dev/bl-midi2-rs/compare/0.6.2..0.6.3) - 2024-11-25

### ✨ Features

- [**breaking**] Packet types - ([972f639](https://github.com/midi2-dev/bl-midi2-rs/commit/972f639d5faf325b4e1b61c20f1ef2854125176b))

### 🐛 Fixes

- Correct delta clock stamp status code - ([e3433ea](https://github.com/midi2-dev/bl-midi2-rs/commit/e3433ea6c9dec1f7becde8944eae453ace08e7d3))
- Correct delta clock stamp status code - ([e8f2d42](https://github.com/midi2-dev/bl-midi2-rs/commit/e8f2d4268f54eb13ac609c44068a27ae54f9324d))

### 📚 Documentation

- Update changelog - ([040ed0f](https://github.com/midi2-dev/bl-midi2-rs/commit/040ed0fcbd754bc46a40487340e338818911934e))


## [0.6.2](https://github.com/midi2-dev/bl-midi2-rs/compare/0.6.1..0.6.2) - 2024-09-02

### ✨ Features

- Allow messages to be created with external backing buffers - ([99f2135](https://github.com/midi2-dev/bl-midi2-rs/commit/99f213563a4e6f5967fc1eadde849812262cdb1e))

### 🐛 Fixes

- Incorrect status of delta clockstamp tpq message - ([48041a0](https://github.com/midi2-dev/bl-midi2-rs/commit/48041a03a4d187a8e1012f4b79cf36fe9ca0c29d))
- Incorrect status of delta clockstamp tpq message - ([dd5214b](https://github.com/midi2-dev/bl-midi2-rs/commit/dd5214b95f13a10d442c7d7493891ca52a677cf2))
- Dirty buffer not cleared on construction - ([eeac969](https://github.com/midi2-dev/bl-midi2-rs/commit/eeac969bfaa6ae0370f57eff0cb4c52e33c41aac))

### 📚 Documentation

- Update changelog - ([0da2535](https://github.com/midi2-dev/bl-midi2-rs/commit/0da2535d952b9496986dd431e6395f430dd4d7c7))
- Improve warning formatting in top level readme - ([f13966b](https://github.com/midi2-dev/bl-midi2-rs/commit/f13966b41e0e173f404dd1022e517feb0b4dbc2b))


## [0.6.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.5.4..0.6.0) - 2024-07-18

### 🐛 Fixes

- Fixes sysex next impl broken with 'empty' packets - ([72e5eb8](https://github.com/midi2-dev/bl-midi2-rs/commit/72e5eb86b5c41d981ca37e5f7b23caace6777986))
- Fix further clippy warnings - ([4dddc34](https://github.com/midi2-dev/bl-midi2-rs/commit/4dddc34d172135af23d3ba64a169854694aaf95c))
- A couple of clippy warnings - ([2a46540](https://github.com/midi2-dev/bl-midi2-rs/commit/2a465404535c39c1f2485a6c9710600f88f614a5))

### 🛠️ Refactor

- [**breaking**] Rename *note properties to note_number - ([7f18e1f](https://github.com/midi2-dev/bl-midi2-rs/commit/7f18e1fb19a83d8f52023ab9b067408dd95fbb1f))
- [**breaking**] Pitch7_9 and pitch7_25 used fixed number type - ([eaaf529](https://github.com/midi2-dev/bl-midi2-rs/commit/eaaf529e0bec60e96e80d718ea612c19166bb312))

### 📚 Documentation

- Update changelog - ([ab8bd82](https://github.com/midi2-dev/bl-midi2-rs/commit/ab8bd829db4f6e4a328fbf09f22f87b1ff38cd49))


## [0.5.4](https://github.com/midi2-dev/bl-midi2-rs/compare/0.5.3..0.5.4) - 2024-06-05

### ✨ Features

- Generate MIDI CI messages - ([53abf5b](https://github.com/midi2-dev/bl-midi2-rs/commit/53abf5b8103ba53401dd6585c72d19e418686438))

### 🐛 Fixes

- Deiscovery property type - ([2367796](https://github.com/midi2-dev/bl-midi2-rs/commit/23677960dee53e9db57460cb2647b176580a287b))

### 🛠️ Refactor

- *(proc)* Improve gen ci supported version attributes - ([e6e3ad9](https://github.com/midi2-dev/bl-midi2-rs/commit/e6e3ad9b8d603374d5183f1aa1944ca8fc27c822))
- [**breaking**] Ci is no longer a default feature - ([a9c15d1](https://github.com/midi2-dev/bl-midi2-rs/commit/a9c15d1bb1056e7c252d84ca49bebd3883567dc2))
- Further improvements to ci version implementations - ([296aa83](https://github.com/midi2-dev/bl-midi2-rs/commit/296aa833ca96dc175077caa97e7b1db6c1658702))

### 📚 Documentation

- Update changelog - ([464b15e](https://github.com/midi2-dev/bl-midi2-rs/commit/464b15e7ae34fb77d579ba5102af764da0f0c1e5))


## [0.5.3](https://github.com/midi2-dev/bl-midi2-rs/compare/0.5.2..0.5.3) - 2024-05-17

### 📚 Documentation

- Update changelog - ([e4a6bf5](https://github.com/midi2-dev/bl-midi2-rs/commit/e4a6bf5abe5fd7756b0f72cdff6f0b41770a4284))


## [0.5.1](https://github.com/midi2-dev/bl-midi2-rs/compare/0.5.0..0.5.1) - 2024-05-17

### 🐛 Fixes

- Incorrect flex_data deserialisation - ([9c99de5](https://github.com/midi2-dev/bl-midi2-rs/commit/9c99de50af09fc020fd483bee1a855d10b154c31))

### 📚 Documentation

- Update changelog - ([f7b26c3](https://github.com/midi2-dev/bl-midi2-rs/commit/f7b26c3b1cc78b9dd4ec561545054fd17d703b1b))
- Flex data module docs - ([8634e74](https://github.com/midi2-dev/bl-midi2-rs/commit/8634e74ed73fa2f025f566f51e5841f610e60df7))
- Adds system common module docs - ([6bfcfc0](https://github.com/midi2-dev/bl-midi2-rs/commit/6bfcfc0d8e53e55fd19ccde594f716b2e2cdb1b8))
- Channel voice 2 module docs - ([134b4f4](https://github.com/midi2-dev/bl-midi2-rs/commit/134b4f4641c5f5083a6cb43c55c4af7340743822))


## [0.5.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.4.0..0.5.0) - 2024-05-16

### ✨ Features

- Infallible rebuffer to array buffer for fixed size aggregates - ([dd301f0](https://github.com/midi2-dev/bl-midi2-rs/commit/dd301f079c3be8d61b7fb7c51a54a55cdd653040))
- New Packets trait implemented by all ump messages - ([a8558f5](https://github.com/midi2-dev/bl-midi2-rs/commit/a8558f586932319c34739d0efc81fc700de2e5d8))
- Ump <-> bytes conversions infallible for array backed messages - ([d3dd8a5](https://github.com/midi2-dev/bl-midi2-rs/commit/d3dd8a5ec33182785e50e370941ea8a244b58b2b))
- Rebuffer trait infallible for array types greater than min size - ([0fbbe6a](https://github.com/midi2-dev/bl-midi2-rs/commit/0fbbe6a98b3dd0e9afd508b78f6c1467e675574e))
- [**breaking**] Improve array constructors - ([c985e55](https://github.com/midi2-dev/bl-midi2-rs/commit/c985e55618f1ae6a5612d49da3f433dca0ed1c5c))
- Improved array new impls - ([d04ca57](https://github.com/midi2-dev/bl-midi2-rs/commit/d04ca57a5c4ec175627c205c4018f5363304120d))
- Min size uses const eval - ([738c955](https://github.com/midi2-dev/bl-midi2-rs/commit/738c9557aeacd640793f702138dffbe36b04158f))

### 🐛 Fixes

- Flex data text bytes iterator is public - ([92696d7](https://github.com/midi2-dev/bl-midi2-rs/commit/92696d7ac80a613ded754fc248cfc2918b667f72))

### 🛠️ Refactor

- [**breaking**] Remove aggregate error type and result - ([b14a368](https://github.com/midi2-dev/bl-midi2-rs/commit/b14a3683adc2a966381982fc0914492ad32e6a6b))
- [**breaking**] Rename DeltaClockstampTPQ -> DeltaClockstampTpq - ([2b60154](https://github.com/midi2-dev/bl-midi2-rs/commit/2b60154e1f551e719b71f9f3a57cb06c742f17c9))
- Replace redundant instances of `try_new` with `new` - ([ebeb2bb](https://github.com/midi2-dev/bl-midi2-rs/commit/ebeb2bb72ebd196f6b1edf01a6891474ff359d7d))

### 📚 Documentation

- Update changelog - ([9ad26be](https://github.com/midi2-dev/bl-midi2-rs/commit/9ad26bec2077260356360e085501ad36528c86fa))
- Change working in root README - ([695e707](https://github.com/midi2-dev/bl-midi2-rs/commit/695e70703efc5eb74d5e247201ea9b42aad3cf07))
- MIDI 1.0 channel voice docs - ([01541e4](https://github.com/midi2-dev/bl-midi2-rs/commit/01541e49613dfdf6004f6a6dc3ecdd6211f3bf93))
- Fix wording in README - ([9d8de72](https://github.com/midi2-dev/bl-midi2-rs/commit/9d8de72e63fcae568abd24df32b18cae3236206d))
- Documentation for buffer traits - ([b7923e7](https://github.com/midi2-dev/bl-midi2-rs/commit/b7923e7264ecefc06af44ef17d70e9720df85933))
- Further documentation for midi2 core traits - ([4aa4d85](https://github.com/midi2-dev/bl-midi2-rs/commit/4aa4d85088ccd95d6d89a5297da14cd69755c1d1))
- Add documentation to some central traits - ([c70d859](https://github.com/midi2-dev/bl-midi2-rs/commit/c70d85946ea7c4143149ec2fd3289a2fd372f1e0))
- Indicate which functions require std feature enabled - ([f996b0f](https://github.com/midi2-dev/bl-midi2-rs/commit/f996b0fb751a653164558c8200ea9da8d62aa4f0))
- Adds docs for new packets trait - ([8631397](https://github.com/midi2-dev/bl-midi2-rs/commit/8631397fb96f800a1e738242c8ec2507389e07c8))
- Add docs for arr constructors - ([cff6e33](https://github.com/midi2-dev/bl-midi2-rs/commit/cff6e33599e5a243a33f2af602a11be349712f65))

## New Contributors ❤️

* @Hasnabruzzn made their first contribution in [#20](https://github.com/midi2-dev/bl-midi2-rs/pull/20)

## [0.4.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.3.1..0.4.0) - 2024-05-12

### ✨ Features

- Top level messages implement From for all messages - ([dd905c3](https://github.com/midi2-dev/bl-midi2-rs/commit/dd905c31789c3a434693ec6c0bad60913dde7004))

### 🐛 Fixes

- Utility messages should be excluded when feature is not enabled - ([24925c0](https://github.com/midi2-dev/bl-midi2-rs/commit/24925c003c0459cd6f7a64fd3b0b5000a8bf15ad))

### 🛠️ Refactor

- Fix unused code warnings - ([db3c1df](https://github.com/midi2-dev/bl-midi2-rs/commit/db3c1dfb6cb1305728d360bba4d5156b141b95e1))
- Fix public exports of utility module - ([7269e6b](https://github.com/midi2-dev/bl-midi2-rs/commit/7269e6be852d90e6804f835817273fe3a38d6968))

### 📚 Documentation

- Update changelog - ([feb34da](https://github.com/midi2-dev/bl-midi2-rs/commit/feb34dac7380ef37f3b1e367bab642bb384f5d62))

### ◀️🔙 Revert

- Remove redundant jitter reduction derive - ([e36f505](https://github.com/midi2-dev/bl-midi2-rs/commit/e36f5057355f10d7f71f92fceb7fc2fa4a2dbadc))


## [0.3.1](https://github.com/midi2-dev/bl-midi2-rs/compare/0.3.0..0.3.1) - 2024-05-10

### 🐛 Fixes

- Panic on empty ump text iterator - ([367e944](https://github.com/midi2-dev/bl-midi2-rs/commit/367e9443ffd76397b1601f1967a7f97c539f5181))
- Panic in empty flex-data text iterator - ([06b0140](https://github.com/midi2-dev/bl-midi2-rs/commit/06b0140533fa8065d4ec27812fd46f25b8073ce4))

### 📚 Documentation

- Update changelog - ([1012bfe](https://github.com/midi2-dev/bl-midi2-rs/commit/1012bfec4261e457906d65bcb80d130e835f7b1d))
- Fix typos in readme - ([d450982](https://github.com/midi2-dev/bl-midi2-rs/commit/d450982710e5799d6abb2ad7c7bfc803e558f746))


## [0.3.0](https://github.com/midi2-dev/bl-midi2-rs/compare/0.2.4..0.3.0) - 2024-05-10

### ✨ Features

- Utility messages are integrated into top level aggregate - ([55567b4](https://github.com/midi2-dev/bl-midi2-rs/commit/55567b411a2901f8ed6bb2557059554b25c44c5d))

### 🐛 Fixes

- Fix doc tests and utility tests - ([efa6d6e](https://github.com/midi2-dev/bl-midi2-rs/commit/efa6d6e0f2636171335f9471099ce463a8d27848))
- Fix or remove tests failing due to jr change - ([13d3d15](https://github.com/midi2-dev/bl-midi2-rs/commit/13d3d1531ef61920f13b6dada18a620f566fffc5))
- Hide private const - ([ecddbf6](https://github.com/midi2-dev/bl-midi2-rs/commit/ecddbf6f516995b1a13a7d1940e03b00bd67bf2a))
- Hide further private types - ([a688bbf](https://github.com/midi2-dev/bl-midi2-rs/commit/a688bbfe72cb041db7e6ad55539e6f3d98bada2e))
- Hide some private types - ([1604a28](https://github.com/midi2-dev/bl-midi2-rs/commit/1604a284c6ce313e8b429eeb888f398a8b0ba18e))

### 🛠️ Refactor

- Remove unused constants - ([4e59841](https://github.com/midi2-dev/bl-midi2-rs/commit/4e5984174459b4ee8e75400b8e7f6f6390dbfe90))
- Remove jr timestamp headers and trait - ([01870cb](https://github.com/midi2-dev/bl-midi2-rs/commit/01870cbf9d23fccc44cfe79b98a2e2b09237c19e))
- [wip] remove jitter reduction trait and property - ([4521d5d](https://github.com/midi2-dev/bl-midi2-rs/commit/4521d5d22c76364f28146aa22dd8a98a95a26a90))

### 📚 Documentation

- Update changelog - ([3b7d34d](https://github.com/midi2-dev/bl-midi2-rs/commit/3b7d34d7c397cf37f0bf0f44256c70cef4c5ee5a))
- Fix further readme typos - ([7881cc1](https://github.com/midi2-dev/bl-midi2-rs/commit/7881cc14e6354b0b5b443e9f0f3ffd2cb5781d41))


## [0.2.4](https://github.com/midi2-dev/bl-midi2-rs/compare/0.2.3..0.2.4) - 2024-05-08

### 🐛 Fixes

- Sysex7 iterator - ([e08fbca](https://github.com/midi2-dev/bl-midi2-rs/commit/e08fbcac9e5d8951cf9abdb51dc92a8ca2501f75))
- Bugs in the sysex8 message - ([5459337](https://github.com/midi2-dev/bl-midi2-rs/commit/54593371ac6ff016e4217ac13a709c0825c70859))

### 📚 Documentation

- Update changelog - ([6abb4f1](https://github.com/midi2-dev/bl-midi2-rs/commit/6abb4f1c85060912063a24485b760fd711769bf7))
- Fixes some docs typos - ([9d5d502](https://github.com/midi2-dev/bl-midi2-rs/commit/9d5d502637bdbea5f648f02da51f84194c2fcc44))
- Fix readme typo - ([a6c359a](https://github.com/midi2-dev/bl-midi2-rs/commit/a6c359a229c02cf83c851a7baf41741c369f7a7b))
- Buffer docs typo - ([0ddaf50](https://github.com/midi2-dev/bl-midi2-rs/commit/0ddaf50d52458d7d4a24e843a1c945d74387682a))
- Online docs generated with all features enabled - ([1f128e1](https://github.com/midi2-dev/bl-midi2-rs/commit/1f128e18273806f987982394d4fdffb207652afe))
- Fix toml example in the README.md - ([fdb908e](https://github.com/midi2-dev/bl-midi2-rs/commit/fdb908e17da1ae57126f8ac7687503b559b6f989))

### 🧪 Testing

- Add fuzzing target for sysex7 roundtrip - ([723c565](https://github.com/midi2-dev/bl-midi2-rs/commit/723c5650a664e2ee20f19896886a828f24534041))
- Fuzz testing [wip] - ([869e4bb](https://github.com/midi2-dev/bl-midi2-rs/commit/869e4bbf5acd3aa42b47f222fa627b78cff7c5be))


## [0.2.3](https://github.com/midi2-dev/bl-midi2-rs/compare/0.2.2..0.2.3) - 2024-05-08

### 🐛 Fixes

- Handling messages example code - ([88036cb](https://github.com/midi2-dev/bl-midi2-rs/commit/88036cbd0e0b27c1246b2b5343f427a6a9d7090b))

### 📚 Documentation

- Update changelog - ([63490b4](https://github.com/midi2-dev/bl-midi2-rs/commit/63490b4f51e74879c3a900245d1bd194b479b47c))


## [0.2.2](https://github.com/midi2-dev/bl-midi2-rs/compare/0.2.1..0.2.2) - 2024-05-08

### 🐛 Fixes

- Panic in fallible sysex7 payload setter - ([8ec0c9e](https://github.com/midi2-dev/bl-midi2-rs/commit/8ec0c9e3ff8eb6eb3ae09784b1b19bc7acd7ee19))
- Panic in fallible sysex8 payload setter - ([6e48325](https://github.com/midi2-dev/bl-midi2-rs/commit/6e48325a11a56fe680f6156d5583dde05643e6bd))
- Repo url must be http - ([3ff7ecb](https://github.com/midi2-dev/bl-midi2-rs/commit/3ff7ecb6959aac847b95feefc6c7be7e5438e050))
- Repository url must be http - ([6576eae](https://github.com/midi2-dev/bl-midi2-rs/commit/6576eae79537d4f7e845e517f3edb8a855ad5130))

### 🛠️ Refactor

- Repo is handled as a cargo workspace - ([2dc06c0](https://github.com/midi2-dev/bl-midi2-rs/commit/2dc06c0ed567c6b85b3c703990ed11c555451545))

### 📚 Documentation

- Update changelog - ([3765fcd](https://github.com/midi2-dev/bl-midi2-rs/commit/3765fcd4b55bdb193e6112f86f404dafa1dcb849))
- Buffer docs - ([2a8841b](https://github.com/midi2-dev/bl-midi2-rs/commit/2a8841bae8292f8aa67b22da50cc17abb96eb910))
- Address todos in readme - ([d9daf6d](https://github.com/midi2-dev/bl-midi2-rs/commit/d9daf6da73bc8599886bc0223e84463e6d9e2143))
- Adds message handling example - ([0160e2b](https://github.com/midi2-dev/bl-midi2-rs/commit/0160e2b306ac0307f9619452141bf80a8699dc03))
- Tweak project description - ([89d250c](https://github.com/midi2-dev/bl-midi2-rs/commit/89d250cdefa35b96191940f940c7d216ddabfac2))


## [0.2.1](https://github.com/midi2-dev/bl-midi2-rs/compare/0.2.0..0.2.1) - 2024-05-07

### 🐛 Fixes

- Default features build - ([2d45fac](https://github.com/midi2-dev/bl-midi2-rs/commit/2d45facbd8854192c66cb74e2fa0338e93fdb54c))

### 📚 Documentation

- Update changelog - ([1b4794e](https://github.com/midi2-dev/bl-midi2-rs/commit/1b4794ea19b01bbd790da93e5cd35a32941fad43))


## [0.2.0] - 2024-05-07

### ✨ Features

- *(hooks)* Update hooks - ([2c4476d](https://github.com/midi2-dev/bl-midi2-rs/commit/2c4476d879ae6686bb5e3aa81f566015be9cb4e8))
- Migrate top level aggregate message - ([d5bf93f](https://github.com/midi2-dev/bl-midi2-rs/commit/d5bf93f2169d93d2ba1e20c5230df66110809f52))
- Fix bug in sysex8 jr header - ([a37e723](https://github.com/midi2-dev/bl-midi2-rs/commit/a37e7238df0fe6c0b166e4ab940df179ec946b41))
- Fix another flex-data size bug - ([690375b](https://github.com/midi2-dev/bl-midi2-rs/commit/690375b4ce08aece78f087a12e56f3f5c4bf6bfd))
- Ump stream aggregate message - ([d009cfb](https://github.com/midi2-dev/bl-midi2-rs/commit/d009cfbbdecf87ce13ae8d2d56528e496dfe6393))
- Migrate all ump stream messages - ([ca23bfa](https://github.com/midi2-dev/bl-midi2-rs/commit/ca23bfa9eef179ef275132cb2c2856a5a83806f9))
- [wip]  migrate ump stream messages - ([d6c09d1](https://github.com/midi2-dev/bl-midi2-rs/commit/d6c09d1eaf7077102336ecde2057354175dbe384))
- Migrate sysex8 - ([089c308](https://github.com/midi2-dev/bl-midi2-rs/commit/089c30858bc787f61a37d74c20ee3cb496a9b0d0))
- Aggregate midi2 cv message - ([dcba227](https://github.com/midi2-dev/bl-midi2-rs/commit/dcba2275cfa8124d696bc2903fe4f8cdfc81da82))
- Migrate last midi2 cv messages - ([af2df10](https://github.com/midi2-dev/bl-midi2-rs/commit/af2df10448c0adcb8e63d0a370c3b5ec0d56d4ec))
- [wip] migrate midi2 cv messages - ([b21f7d3](https://github.com/midi2-dev/bl-midi2-rs/commit/b21f7d3990b4b6c2dba0e617b0334720f3877dd6))
- Migrate first midi2 cv message - ([57c3fd3](https://github.com/midi2-dev/bl-midi2-rs/commit/57c3fd34265adb8d96f2361339fa28e2b226e27c))
- Derive jr trait for aggregate messages - ([d2544cf](https://github.com/midi2-dev/bl-midi2-rs/commit/d2544cf13b6b710d42df7b788c929a73d8f18ab1))
- Aggregate flex-data message - ([e8c9d83](https://github.com/midi2-dev/bl-midi2-rs/commit/e8c9d838dd6e26a3f6e886dfe4118f272b0c637c))
- Implement remaining flex-data text messages - ([f3decab](https://github.com/midi2-dev/bl-midi2-rs/commit/f3decab68ca27ce88fff5faec4c84a06fa84c700))
- Doc impl for generated messages - ([58045aa](https://github.com/midi2-dev/bl-midi2-rs/commit/58045aa2eb70150dd9b4bf7de79ab16dacb6db7f))
- Generate std-only properties - ([7f77158](https://github.com/midi2-dev/bl-midi2-rs/commit/7f77158f69ab2a6af4f71aab909e25b9583ba9b3))
- No special trait necessary for borrowed properties - ([7b6df6a](https://github.com/midi2-dev/bl-midi2-rs/commit/7b6df6aa87017a4bc89094f4bd5355d97cf4fcab))
- Read borrowed data from messages - ([bcd4ad0](https://github.com/midi2-dev/bl-midi2-rs/commit/bcd4ad0df3e1c14317c9b39cd0fad6260ecf1567))
- Write str to flex data - ([ce7bc5b](https://github.com/midi2-dev/bl-midi2-rs/commit/ce7bc5b0bb4d8ad902879570a951a55e9470db87))
- Property write / read types can differ - ([19e92ec](https://github.com/midi2-dev/bl-midi2-rs/commit/19e92ecf298f824e73675795aebedddb614c3916))
- Flex-data trait - ([1230499](https://github.com/midi2-dev/bl-midi2-rs/commit/12304995146a8cd137ba3e1cb59beda576642c05))
- Add jitter reduction tests - ([dad5929](https://github.com/midi2-dev/bl-midi2-rs/commit/dad5929bcf3de1682b0ca13ce9b49eaa0ac22e70))
- Migrate all fixed size flex data messages - ([73cfa97](https://github.com/midi2-dev/bl-midi2-rs/commit/73cfa9711c063ab74d9602182c228836b36b851b))
- Aggregate utility message - ([0879196](https://github.com/midi2-dev/bl-midi2-rs/commit/0879196fb94ff7bbf9542820e767e5190ca272b7))
- All ump messages can be jitter reduced - ([9861f72](https://github.com/midi2-dev/bl-midi2-rs/commit/9861f721b89bf5f587300f1335d8ad8f9f67bff0))
- Account for jitter reduction in the buffers - ([acbdecc](https://github.com/midi2-dev/bl-midi2-rs/commit/acbdecc5ad409ccedb8f80a372875de1d3881eac))
- Migrate system common messages - ([0d85c35](https://github.com/midi2-dev/bl-midi2-rs/commit/0d85c357f6116fed139aa6735fabbd8853ece9cf))
- Finish migrate first flex-data message - ([866fe86](https://github.com/midi2-dev/bl-midi2-rs/commit/866fe867fc7f3d0cdc37a79a07a8f023f6b7c377))
- [wip] migrate first flex data message - ([7a89f66](https://github.com/midi2-dev/bl-midi2-rs/commit/7a89f66b5101a61a9dde02e6a54a89ef615914a5))
- Implement ump <-> byte conversion for sysex7 - ([67f68ac](https://github.com/midi2-dev/bl-midi2-rs/commit/67f68acbbc8e20668ffa5271ea9dfe83733b3e21))
- Optimise sysex7 payload iter size calculations - ([3247d53](https://github.com/midi2-dev/bl-midi2-rs/commit/3247d53c7ea68db135acf8100a2463b3cd20ab5a))
- Fixes grouped impl sysex7 - ([8bf8a86](https://github.com/midi2-dev/bl-midi2-rs/commit/8bf8a86e93625368b028f4477e79d7534769b821))
- Finish impl for sysex7 - ([9e40d98](https://github.com/midi2-dev/bl-midi2-rs/commit/9e40d9840df0e078787872b45866b507b3da20ac))
- [wip] sysex7 payload iterators - ([1c8c281](https://github.com/midi2-dev/bl-midi2-rs/commit/1c8c281e387bacb2e435d3047f73326fd4fe7e25))
- [wip] further sysex7 payload iter prep - ([e9caa72](https://github.com/midi2-dev/bl-midi2-rs/commit/e9caa724d96848b6c0a5af15d343ec98ebc05f68))
- [wip] simplification of sysex7 iterator impl - ([5f4d0f3](https://github.com/midi2-dev/bl-midi2-rs/commit/5f4d0f341ddb6077a755c2073b5fbb393bcd3091))
- [wip] sysex7 payload iterator - ([551f367](https://github.com/midi2-dev/bl-midi2-rs/commit/551f3679053e27c5f271b60104f95f0614aa61f5))
- Implement sysex for sysex7 bytes - ([9f6b4e7](https://github.com/midi2-dev/bl-midi2-rs/commit/9f6b4e720f794ded6444f7b8a5a811aaf1717b1a))
- Implement sysex7 static properties - ([e7a1236](https://github.com/midi2-dev/bl-midi2-rs/commit/e7a12366c360f08814b687b5796a0c945cc8f836))
- Crate is almost entirely no_std - ([94f9520](https://github.com/midi2-dev/bl-midi2-rs/commit/94f9520db9763cc10a3a47a82a5cc45d9160fdc1))
- Impl clone for aggregate - ([175b783](https://github.com/midi2-dev/bl-midi2-rs/commit/175b7834cd50e5d0322fbac1ff2d968582436602))
- Array constructors always crate fixed size buffers - ([5e277c2](https://github.com/midi2-dev/bl-midi2-rs/commit/5e277c2adb5c3b00ed0ac458b22a0b9729609ce3))
- Improve buffer constraints - ([25a221c](https://github.com/midi2-dev/bl-midi2-rs/commit/25a221c8833d84be597eb8dc0167a2d58e4f077d))
- Derive rebuffer traits - ([a2258b4](https://github.com/midi2-dev/bl-midi2-rs/commit/a2258b4a773fe67036992a2c18dfaf36d3d0d40b))
- Derive fallible byte <-> ump converters - ([2d9f959](https://github.com/midi2-dev/bl-midi2-rs/commit/2d9f959e3326414730bb2bfdcccd1874525183cf))
- Derive FromBytes and FromUmp - ([46e8d66](https://github.com/midi2-dev/bl-midi2-rs/commit/46e8d66222f5877cf723cc06d3b6573e1b8eddd0))
- Simplify conversion traits - ([831dbe7](https://github.com/midi2-dev/bl-midi2-rs/commit/831dbe7c262d103f5ddcae4883e673e6c45ed4b6))
- Derive grouped - ([3319819](https://github.com/midi2-dev/bl-midi2-rs/commit/3319819be9109a34ea5f7a7715ae7e80ed4acd41))
- Migrate aggregate message - ([30a173b](https://github.com/midi2-dev/bl-midi2-rs/commit/30a173b2c90c29a7eb1659f924028285a65ed7b1))
- Generate clone impl - ([3683777](https://github.com/midi2-dev/bl-midi2-rs/commit/368377731315c3866296cc40a3587efa70b4c85a))
- Migrate derive proc macros - ([fc060af](https://github.com/midi2-dev/bl-midi2-rs/commit/fc060af49a05ebf82f0bfcc58109623aebec0deb))
- Migrate control change - ([9d47f9e](https://github.com/midi2-dev/bl-midi2-rs/commit/9d47f9e6b439abb467449e834069713e24e2f22b))
- Use schema to represent properties - ([4f42db2](https://github.com/midi2-dev/bl-midi2-rs/commit/4f42db2cf0e935155ca738e348007d181cae2fa5))
- Check buffer length for from data constructor - ([e529228](https://github.com/midi2-dev/bl-midi2-rs/commit/e529228e361f4998f83d1961197ea68e25a0a5f3))
- Fix generated data impl - ([8d0a3f9](https://github.com/midi2-dev/bl-midi2-rs/commit/8d0a3f96cf29d5d44171bed6d4fecdb71df9c558))
- Generate fallible byte <-> ump conversions - ([1132e28](https://github.com/midi2-dev/bl-midi2-rs/commit/1132e28c3b16170d400e1afc0ec12d4f07d99731))
- Generate generic fallible constructors - ([eb1b83d](https://github.com/midi2-dev/bl-midi2-rs/commit/eb1b83d0294d452ffff7b853799f6955ff0a8f8f))
- Generate new_with_buffer - ([0729869](https://github.com/midi2-dev/bl-midi2-rs/commit/0729869e81ae6fbac2de1a98e31b0b8c0dece56f))
- Generate byte <-> ump conversion - ([8fe27e7](https://github.com/midi2-dev/bl-midi2-rs/commit/8fe27e727d7a4b5de8855078de8280fac62172af))
- Try rebuffer only on fixed sized buffers - ([2afa870](https://github.com/midi2-dev/bl-midi2-rs/commit/2afa870c63861d9cf0ef7e4137f516bc9be35cfe))
- Try rebuffer traits - ([480052c](https://github.com/midi2-dev/bl-midi2-rs/commit/480052c9016156a965e84f9e8c4905e9d1279e7f))
- Adds rebuffer traits to convert between generic buffers - ([8b9b8d0](https://github.com/midi2-dev/bl-midi2-rs/commit/8b9b8d005590b883f8bb14a294c6d96e0579cbc2))
- Generate data trait - ([2689f61](https://github.com/midi2-dev/bl-midi2-rs/commit/2689f61931f5c9ab6555e2a3358b4aa430d42ec6))
- Channel pressure works with bytes and ump - ([c1f693a](https://github.com/midi2-dev/bl-midi2-rs/commit/c1f693a5157d8c5f0656ef54f309517e2400a8e5))
- Compile channel pressure test - ([6a8d596](https://github.com/midi2-dev/bl-midi2-rs/commit/6a8d596133efed96a19ef5a2427c13451d5ed6a1))
- Some additional common properties - ([d28711b](https://github.com/midi2-dev/bl-midi2-rs/commit/d28711bb1568c99addc2c7c247532b28f5f28295))
- Generic sized arrays support Buffer and BufferMut - ([b53bc4e](https://github.com/midi2-dev/bl-midi2-rs/commit/b53bc4e627e3f8a9f4088439a2b87254ef1189e9))
- Generate TryFrom for slices - ([411aff1](https://github.com/midi2-dev/bl-midi2-rs/commit/411aff138abdba0cb5af7b236106bbfdd9970ed8))
- Util no_op test passes - ([a1d143a](https://github.com/midi2-dev/bl-midi2-rs/commit/a1d143aeb1e9e9599b505f3faba4371b71be52fd))
- Builders for initiate protocol negoptiation - ([eeab41c](https://github.com/midi2-dev/bl-midi2-rs/commit/eeab41cf251a391a884f1b9cefb3c5bb570b078c))
- First borrowed ci message confirm protocol - ([9b0c4c7](https://github.com/midi2-dev/bl-midi2-rs/commit/9b0c4c7ba4c734edab628ac9e9b31feda1668e6a))

### 🐛 Fixes

- *(hooks)* Use merge conflict check - ([efa10e6](https://github.com/midi2-dev/bl-midi2-rs/commit/efa10e6ce08809ca4606df77d688e8b15098c04a))
- Bug in flex data text message length - ([e9e4394](https://github.com/midi2-dev/bl-midi2-rs/commit/e9e43940795c1107fb1f8403b77c6904a2839185))
- Hide private error type - ([9bc36b0](https://github.com/midi2-dev/bl-midi2-rs/commit/9bc36b02068d4900ffe63d38416e2b28e69b2c59))
- A few bugs in the flex-data text messages - ([5c30a15](https://github.com/midi2-dev/bl-midi2-rs/commit/5c30a150491d08a0315f1388ab9b1699d72d57bd))
- Some unused var warnings - ([fb048f2](https://github.com/midi2-dev/bl-midi2-rs/commit/fb048f2c1ea23466c0ecb64ec0a285eb9eaed2dd))
- Prelude feature compatible - ([aaf5535](https://github.com/midi2-dev/bl-midi2-rs/commit/aaf5535703c8ce7a7855a96e4b10d87a905c165f))
- Broken use case of sysex7 payload setter - ([82770a1](https://github.com/midi2-dev/bl-midi2-rs/commit/82770a1c27bd04e8b8cf0266fb50babc692fe37a))
- Ump sysex7 from oversized - ([329a455](https://github.com/midi2-dev/bl-midi2-rs/commit/329a455f02fb6b521fc8efe02c9f2bc28d5d1e9d))
- Incorrect sysex end byte property check - ([68a3643](https://github.com/midi2-dev/bl-midi2-rs/commit/68a3643c3b6521e93791b39a3c5fb729b94b0dde))
- Allow fixed size payloads to be used with rubbish iterators - ([97fbfb5](https://github.com/midi2-dev/bl-midi2-rs/commit/97fbfb5bed5cbb2f2bb1da810e382983236f681a))
- Unhandled result warning - ([d81ac71](https://github.com/midi2-dev/bl-midi2-rs/commit/d81ac7138237fc7fc6b601a60464c5d425a24c0a))
- Docs typo - ([67202fb](https://github.com/midi2-dev/bl-midi2-rs/commit/67202fb1e4c52dd1d2284b89b296e633f210ac08))
- Channel voice unit tests - ([dced22e](https://github.com/midi2-dev/bl-midi2-rs/commit/dced22e63b1a9e21536ae9fa6efbcce6993ddb3a))
- Improve generated debug impl - ([75a2f58](https://github.com/midi2-dev/bl-midi2-rs/commit/75a2f58064483b745ff79b84a44f32f2239e722a))
- New clippy warnings - ([68cf681](https://github.com/midi2-dev/bl-midi2-rs/commit/68cf68182185bd7fc3c5dd32dffc4e5f189f0ed8))

### 🛠️ Refactor

- Simplify prelude - ([ac4d761](https://github.com/midi2-dev/bl-midi2-rs/commit/ac4d7618bb0402ec3169cbf137f6b018fdb687e8))
- Remove numeric types submod - ([26e693f](https://github.com/midi2-dev/bl-midi2-rs/commit/26e693f85e77e002626baad8ceb5313cf5f72cae))
- Remove message submod and util -> detail - ([02dbb7d](https://github.com/midi2-dev/bl-midi2-rs/commit/02dbb7d5d4dc8d6f88d96568821ea7b08e082bc9))
- Remove ci for now - ([36ef8f3](https://github.com/midi2-dev/bl-midi2-rs/commit/36ef8f3cd4911419b36b6903a7cb79bebf2b671b))
- Rename midi* channel voice -> channel voice* - ([321f11d](https://github.com/midi2-dev/bl-midi2-rs/commit/321f11dff32ca229c7abe44e92821a5286eafbc2))
- Finish splitting property trait - ([45966cd](https://github.com/midi2-dev/bl-midi2-rs/commit/45966cdc005e698d293ab7b9a6a0a8b670daed3a))
- [wip] further splitting of property traits - ([b11ec24](https://github.com/midi2-dev/bl-midi2-rs/commit/b11ec2471156978d6d8bb611afdc2f05e94cedbc))
- [wip] break up property trait - ([f6586d9](https://github.com/midi2-dev/bl-midi2-rs/commit/f6586d9446521ccfa5bd30adea257cb62209e8d9))
- Move some sysex7 code around - ([1c8b813](https://github.com/midi2-dev/bl-midi2-rs/commit/1c8b81391d861c0a09ba62847c6b403248f49e13))
- Adds buffer access private trait - ([856575a](https://github.com/midi2-dev/bl-midi2-rs/commit/856575aeca12929375120f729010fbd3ee7f0d22))
- Moving code sysex7 around - ([767764d](https://github.com/midi2-dev/bl-midi2-rs/commit/767764dc8e08ddc8d5841e128552d7eea8e4e254))
- Rename test - ([17ffebb](https://github.com/midi2-dev/bl-midi2-rs/commit/17ffebbaa0adcab01638742d951a367e9de6cbaa))
- Remove redundant trait - ([8747134](https://github.com/midi2-dev/bl-midi2-rs/commit/8747134043c57661a07bbda45417cdf9fa7effac))
- Migrate all midi1 cv messages - ([9c98434](https://github.com/midi2-dev/bl-midi2-rs/commit/9c9843465ea5e7c1fdf9fb59bb7d38d08ac6e2e9))
- Rename constructor methods - ([43b96d3](https://github.com/midi2-dev/bl-midi2-rs/commit/43b96d32226113921a2f75032f046972931d01eb))
- Improve fallible buffer trait - ([8f85009](https://github.com/midi2-dev/bl-midi2-rs/commit/8f85009f4f1eddc8883be1d507ce0a0780e3e3ae))
- Attributes for message generator - ([81f7612](https://github.com/midi2-dev/bl-midi2-rs/commit/81f76127a51a2ece3a8b1a8bc6a57e39df5051fb))
- [wip] rework message api - ([b11fe52](https://github.com/midi2-dev/bl-midi2-rs/commit/b11fe52c9c8e1c22b28f09f14a24894996c1314a))
- - - ([55839b9](https://github.com/midi2-dev/bl-midi2-rs/commit/55839b9e76b90eb6e4c32be4d8fe7199b86d491e))
- All channel voice messaged share helpers - ([e9f2fc8](https://github.com/midi2-dev/bl-midi2-rs/commit/e9f2fc8292ac22b16fc093f26d3b123be0492013))

### 📚 Documentation

- Update CONTRIBUTORS.md - ([f2041cd](https://github.com/midi2-dev/bl-midi2-rs/commit/f2041cd9bc7c17d38d5ff4539e2b8746b5d4da33))
- Fix README - ([dbe55b8](https://github.com/midi2-dev/bl-midi2-rs/commit/dbe55b867bd13ef86c9efbedd122fff26d659aee))
- Sysex tweaks - ([b46966a](https://github.com/midi2-dev/bl-midi2-rs/commit/b46966a9d2f5de91dcea840f28ddb1873d979c45))
- Messages are public within module - ([42a42f7](https://github.com/midi2-dev/bl-midi2-rs/commit/42a42f7fe792def0905d43e210ba6d871f4b88a9))
- Add type docs for m2cv note on - ([b9fd4b2](https://github.com/midi2-dev/bl-midi2-rs/commit/b9fd4b2c6466470fa8f1b3fe8fbeab475e2e4078))
- System common module docs - ([afa751d](https://github.com/midi2-dev/bl-midi2-rs/commit/afa751d85e51dbc6fd5d8600dae015c47a3d6045))
- Improve sysex7 docs - ([d44fb61](https://github.com/midi2-dev/bl-midi2-rs/commit/d44fb6182f44dd44bdd41d802ced3c8df6048c14))
- Adding comments in message proc code - ([e66fd2c](https://github.com/midi2-dev/bl-midi2-rs/commit/e66fd2cc5557fae12bb4c8226c24171f54d5e190))
- Extend docs for sysex7 - ([8bd4fa6](https://github.com/midi2-dev/bl-midi2-rs/commit/8bd4fa64b58eaf964433b898081a1886c4f8d7f8))
- Adds docs for the sysex7 module - ([21961d5](https://github.com/midi2-dev/bl-midi2-rs/commit/21961d5e6187b9f41cce6824335c34abfa6e7f24))
- Generate types with docs - ([06c33f1](https://github.com/midi2-dev/bl-midi2-rs/commit/06c33f1343e6d93eb98fe3937beb6352963ba6ae))
- Documentation for sysex7 payload iterator - ([c102029](https://github.com/midi2-dev/bl-midi2-rs/commit/c10202978379b88877b06167cdf10bf89d34bfa0))
- Adds warning comment about private trait - ([55ebe61](https://github.com/midi2-dev/bl-midi2-rs/commit/55ebe61a48729a6fc942527b2e81e7b693f11120))
- Update comments for internal sysex trait - ([da68f8c](https://github.com/midi2-dev/bl-midi2-rs/commit/da68f8c400d6a1f7336248599c868802f4d22207))
- Adds trait docs - ([5338e5c](https://github.com/midi2-dev/bl-midi2-rs/commit/5338e5c611a741fbca4b2e6f65b861e81c6b907b))
- Improve borrowed / owned example in readme - ([5f105dd](https://github.com/midi2-dev/bl-midi2-rs/commit/5f105dd3a15a2110ef68b2fa7c853644c903671a))

### 🧪 Testing

- Add tests for sysex7 data - ([fd7b566](https://github.com/midi2-dev/bl-midi2-rs/commit/fd7b5660c020adb32a06c8c44381dc4cc6543273))
- Add some tests for rebuffer - ([f4f2ef8](https://github.com/midi2-dev/bl-midi2-rs/commit/f4f2ef8a2f13954d0759751218ab853fed7aef02))

### Build

- Fix no_std builds for sysex bytes - ([504239e](https://github.com/midi2-dev/bl-midi2-rs/commit/504239e5339723bc01b1804f6ca3fab21c1f3b56))

### Ci

- Cleanup - ([f924313](https://github.com/midi2-dev/bl-midi2-rs/commit/f924313442c77a249db96437f0131eb61747b401))
- Fix ci messages - ([8a5955b](https://github.com/midi2-dev/bl-midi2-rs/commit/8a5955b0d6cd0b4f8f443d84053cd61a8598348c))

### Cleanup

- Use blanket trait instead of macros - ([1becdbb](https://github.com/midi2-dev/bl-midi2-rs/commit/1becdbba0606437cb92041c6ef817bbc573f108a))

### Debug

- Spaces between bytes - ([bcb58d1](https://github.com/midi2-dev/bl-midi2-rs/commit/bcb58d1dd6efdc449ff646b6324d86c169e1d8ff))

### Feat

- Builders return mut ref - ([7b8947b](https://github.com/midi2-dev/bl-midi2-rs/commit/7b8947b39028aa8430bed506b10a07b7704372a5))
- Builders return mut ref - ([0d9ada7](https://github.com/midi2-dev/bl-midi2-rs/commit/0d9ada782d83d990e97dcce852fa433833493c2a))
- Builders return mut ref - ([c0e8073](https://github.com/midi2-dev/bl-midi2-rs/commit/c0e8073db699a9ea2813cb2132fe4a51ff863eb3))
- Default impl for all owned builders - ([7bc9aa5](https://github.com/midi2-dev/bl-midi2-rs/commit/7bc9aa54bae249560187f52401caae3da8385cec))
- Aggregate felx data messages - ([2f71317](https://github.com/midi2-dev/bl-midi2-rs/commit/2f71317603f96125943ba28a1b0d7a3154eb3807))
- Flex data text messages implement common trait - ([27491ee](https://github.com/midi2-dev/bl-midi2-rs/commit/27491ee7f4efe14b71746c4aef12804156cbc942))
- Adds flex data text messages - ([ca96a98](https://github.com/midi2-dev/bl-midi2-rs/commit/ca96a98a619f60196684f78ce302c42917390247))
- Common trait for flex data messages - ([3954bc5](https://github.com/midi2-dev/bl-midi2-rs/commit/3954bc51153b67bd352a602a2ed0ef0e45cd60dd))
- Adds flex data group messages - ([747fde2](https://github.com/midi2-dev/bl-midi2-rs/commit/747fde2d73e51b23052cd69244de9206d8eb2bbb))
- Set chord name message - ([d3fdad3](https://github.com/midi2-dev/bl-midi2-rs/commit/d3fdad32cdf63dcc160d37278332278806403c13))
- Set key signature message - ([13d4c40](https://github.com/midi2-dev/bl-midi2-rs/commit/13d4c4004ee6e59bc1cb26865ea4cfe2be17eb5a))
- Set metronome message - ([2f3da5c](https://github.com/midi2-dev/bl-midi2-rs/commit/2f3da5cfa9abf5f81d66682ec9e1d1d52572f1df))
- Set time signature message - ([15d233a](https://github.com/midi2-dev/bl-midi2-rs/commit/15d233a9b69b236756d9f35b75ccd0dd2e3f89ae))
- Adds set tempo message - ([7637ae8](https://github.com/midi2-dev/bl-midi2-rs/commit/7637ae8adfc913e59f59e3918b3c657e72fca803))
- Debug impl for error + std feature - ([ea52f8e](https://github.com/midi2-dev/bl-midi2-rs/commit/ea52f8e60d2e926edce8f1688c60c0c10f239488))
- Adds new utility message - ([d37580b](https://github.com/midi2-dev/bl-midi2-rs/commit/d37580bb70c4f69f559aa46f0b43d18d3d623cae))
- Build top level sysex messages in std - ([b74c8b6](https://github.com/midi2-dev/bl-midi2-rs/commit/b74c8b64e8eea2f6ec621c784a47606d4e089c9f))
- Include readme in docu for no_std - ([8a2a41d](https://github.com/midi2-dev/bl-midi2-rs/commit/8a2a41d73a053d18a68b254f45f717f344f10061))
- No_std owned top level message and builder - ([d5fca38](https://github.com/midi2-dev/bl-midi2-rs/commit/d5fca38c4fe1157d3284b59eeaf70142a076da25))
- Ump stream messages in top level - ([b4a7e32](https://github.com/midi2-dev/bl-midi2-rs/commit/b4a7e327cd2cfd855a9689fa47d41da2f93c95b1))
- Aggregate ump stream messages - ([e2a3b40](https://github.com/midi2-dev/bl-midi2-rs/commit/e2a3b4033970653d7567f2a8b5a28f136c5c3326))
- Start / end of clip messages - ([6c7f31a](https://github.com/midi2-dev/bl-midi2-rs/commit/6c7f31a028e26bdd1f991805d8f048d63a9916ea))
- Function block name message - ([a3fb649](https://github.com/midi2-dev/bl-midi2-rs/commit/a3fb649e2093c677ac7a1f531cc96b08d45ea0fc))
- String properties return results - ([84ba22e](https://github.com/midi2-dev/bl-midi2-rs/commit/84ba22ee82cf10aff2c8e23008ff8215820191f1))
- Function block discovery and info messages - ([b3fbd73](https://github.com/midi2-dev/bl-midi2-rs/commit/b3fbd735a9fab7be5fe29fb6fbe719f0cb94efd5))
- Stream configuration messages - ([720ccf2](https://github.com/midi2-dev/bl-midi2-rs/commit/720ccf2204ece8c25816a63eeb5e6c35f6126b8c))
- Product instance id message - ([78785ba](https://github.com/midi2-dev/bl-midi2-rs/commit/78785bac534911aff79afe695a492fa756742956))
- Endpoint name messages - ([6f8374e](https://github.com/midi2-dev/bl-midi2-rs/commit/6f8374ee58d92f3319906d911c5ec1b44db5050f))
- (wip) ump stream group - ([2d50265](https://github.com/midi2-dev/bl-midi2-rs/commit/2d50265980bfb4f38085aeadcb9ed65f4dab2a9b))
- Device identity ump stream message - ([188a9a0](https://github.com/midi2-dev/bl-midi2-rs/commit/188a9a0b1ee3fa289d1591656502cde321dce0d0))
- Fixes endpoint info message - ([cc9479e](https://github.com/midi2-dev/bl-midi2-rs/commit/cc9479e98bc4ad2c42ebc4eec944235ca6a37305))
- (wip) begin implementing ump stream messages - ([ac0bb29](https://github.com/midi2-dev/bl-midi2-rs/commit/ac0bb29f70a1f9cef1cb5f716c6c28b2a63ff937))
- Aggregate messages are available without std - ([202a697](https://github.com/midi2-dev/bl-midi2-rs/commit/202a697d58b9aa5c5af042cfeab4c6252c95b5d1))
- Sysex messages in top level aggregate message - ([d043df1](https://github.com/midi2-dev/bl-midi2-rs/commit/d043df184478f0905592863c63c9279216658c33))
- Channeled trait - ([d2db81a](https://github.com/midi2-dev/bl-midi2-rs/commit/d2db81a9e1a90fb6c6bdd693e288c3d108ce578a))
- Owned sysex messages - ([0059dde](https://github.com/midi2-dev/bl-midi2-rs/commit/0059dded545a4f5918578ba802d9d73174c89b31))

### Feature

- Cargo default features - ([5571428](https://github.com/midi2-dev/bl-midi2-rs/commit/5571428fae70a3084ff05a1342f9eecc5bc9d763))
- Granular features - ([3ca99a1](https://github.com/midi2-dev/bl-midi2-rs/commit/3ca99a1792382a11f0646d7b03e0b49a2d8d2eb6))

### Hooks

- Adds install helper script - ([671bde8](https://github.com/midi2-dev/bl-midi2-rs/commit/671bde8577b164c95da51b9211acdde9d46815f0))
- Adds codespell - ([5cfd998](https://github.com/midi2-dev/bl-midi2-rs/commit/5cfd9986e0b3d3d7a853d1aa3487f07a89c80c52))

### Pre-commit

- Adds committed commit-msg hook - ([fccd841](https://github.com/midi2-dev/bl-midi2-rs/commit/fccd841aa222ae9b0b6cf61759671ddbc17c4227))

### Sysex

- Inserting a non existent range grows the buffer - ([f5b0f64](https://github.com/midi2-dev/bl-midi2-rs/commit/f5b0f64884237e7c750cb51b0b351b367e2206ab))
- Cleanup trait lifetimes - ([bef3c6d](https://github.com/midi2-dev/bl-midi2-rs/commit/bef3c6d2cd19a66c0682942515a6ec22b117f47d))
- Adds insert payload on sysex builders - ([e464019](https://github.com/midi2-dev/bl-midi2-rs/commit/e464019b15d5a00d6005e6c583caa3d8549a6077))
- Add builder tests for worst case payload iterators - ([8e1ba46](https://github.com/midi2-dev/bl-midi2-rs/commit/8e1ba465eef1de4c8c2e07e3a3e10a65e3066d32))
- Sysex8 implement sysex builder trait - ([4ebefe3](https://github.com/midi2-dev/bl-midi2-rs/commit/4ebefe3846d077b2245e10e40f7bdedbf07069fa))
- Sysex7 implements sysex builder trait - ([80d3cfe](https://github.com/midi2-dev/bl-midi2-rs/commit/80d3cfe273218af2a1137757d4f4e75e66ba2f28))
- Builder trait - ([6a0209e](https://github.com/midi2-dev/bl-midi2-rs/commit/6a0209e2eefa6d4185011e0d32623bc753855efb))
- Further tweaks to the builder payload methods - ([c08ae5e](https://github.com/midi2-dev/bl-midi2-rs/commit/c08ae5e822f52571fb06fea860c2bddef0aca85d))
- Improve payload insert builder method - ([6191d12](https://github.com/midi2-dev/bl-midi2-rs/commit/6191d1242b46ffd2eb89d64d695a5c1b4e21fc50))
- Adds more flexible build options for payload - ([60feab2](https://github.com/midi2-dev/bl-midi2-rs/commit/60feab266289eadbf957ac430843c5cf1bdcf819))

### Test

- Adds into_owned test for channel pressure message - ([763fd28](https://github.com/midi2-dev/bl-midi2-rs/commit/763fd28549f4d864e176eae93e8842aa0a14f509))

### Tests

- Create test support module - ([ba7ab35](https://github.com/midi2-dev/bl-midi2-rs/commit/ba7ab358b39a715de1ec88da0399391c56abbe3c))

### Bugfix

- Fixes data wrong on dirty buffer - ([d14c55b](https://github.com/midi2-dev/bl-midi2-rs/commit/d14c55baf01ed6fb447a105734b7a33c0d9cf4e7))

### Build

- Update dependencies - ([db637ae](https://github.com/midi2-dev/bl-midi2-rs/commit/db637aee58ba0796c49ed208934fe4fdd9d437da))

### Cargo

- Update exclusions file to include proc macro crate - ([e1d20ee](https://github.com/midi2-dev/bl-midi2-rs/commit/e1d20ee373568946b84751eb5edc032798a4a1f1))

### Cleanup

- Remove unused modules - ([8d038ec](https://github.com/midi2-dev/bl-midi2-rs/commit/8d038ec833b34a281a91dbbd5fffe0c6c67de32f))

### Discovery

- Borrowed messaged - ([8ddbbd4](https://github.com/midi2-dev/bl-midi2-rs/commit/8ddbbd444a4aedafda379c350fb6960ab0edcd0c))

### Sysex8

- Owned message - ([4018b3b](https://github.com/midi2-dev/bl-midi2-rs/commit/4018b3b1938dbf4f6e16e43260b6cf963d7556ac))

### Wip

- Nak messages - ([f26c119](https://github.com/midi2-dev/bl-midi2-rs/commit/f26c1197613508883ad160d7c76466eafa7d0d3b))
- Rework converters - ([bb9d977](https://github.com/midi2-dev/bl-midi2-rs/commit/bb9d977fd28dba62688e5d28232b08fa888ce930))
- Root level message type - ([ac2f2cf](https://github.com/midi2-dev/bl-midi2-rs/commit/ac2f2cf2ebbd4c12897b53c4cfa291b839b8e9c6))
- Begin implementing controller messages - ([ecd258d](https://github.com/midi2-dev/bl-midi2-rs/commit/ecd258d7aeb9b5c47a85a372d82c5d22c16fddf2))

## New Contributors ❤️

* @ made their first contribution
* @BenLeadbetter made their first contribution

