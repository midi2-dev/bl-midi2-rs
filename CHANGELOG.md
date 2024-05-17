# 0.5.3
docs: flex data module docs
docs: adds system common module docs
docs: channel voice 2 module docs
fix: incorrect flex_data deserialisation

# 0.5.0
docs: generally improve documentation of public modules and traits
feat: infallible constructors and converters for array backed messages
feat: new `Packets` trait implemented by all ump messages
fix: flex data text bytes iterator is public
refactor!: ⚠️  remove dedicated array constructors in favour of unified generic constructors
refactor!: ⚠️  remove redundant aggregate error type and result
refactor!: ⚠️  rename DeltaClockstampTPQ -> DeltaClockstampTpq
refactor: switching implementation from mod.rs to file names based on module name

# 0.4.0
feat: top level messages implement From for all messages
fix: ⚠️  utility messages should be excluded when feature is not enabled
refactor: remove some unused code
refactor: ⚠️  hide private utility submodules

# 0.3.1
docs: fix typos in readme
fix: panic on empty flex-data text iterator
fix: panic on empty ump-stream text iterator

# 0.3.0
docs: fix further readme typos
feat: utility messages are integrated into top level aggregate
fix: ⚠️  hide some leaked private types and constants
revert: ⚠️  remove jr timestamp headers and trait

# 0.2.4
ci: add standard cargo github actions
docs: fix various typos
docs: online docs generated with all features enabled
fix: sysex7 / sysex8 payload iterator integration with jr headers
fix: sysex7 / sysex8 payload iterator panics when empty
test: add fuzzing target for sysex7 and sysex8 roundtrip

# 0.2.3
fix: handling messages example code
fix: default features include cv2 not cv1

# 0.2.2
chore: licencing
docs: address todos in readme
docs: adds message handling example
docs: buffer module docs
fix: panic in fallible sysex7 payload setter
fix: panic in fallible sysex8 payload setter
fix: repo url must be http
fix: repository url must be http
refactor: repo is handled as a cargo workspace

# 0.2.1
fix: default features build
