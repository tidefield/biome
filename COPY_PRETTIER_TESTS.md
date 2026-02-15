# How to Copy Tests from Prettier Repository

## YES - You Can Copy Prettier Tests!

Biome has an automated system to extract and convert Prettier's test suite. Here's how to do it.

---

## Quick Overview

**What happens:**
1. Clone the Prettier repository
2. Run a Node.js script that extracts Prettier's Jest snapshots
3. Converts them to Biome's test format
4. Copy input files and expected output snapshots

**Result:**
- All of Prettier's markdown tests
- Automatically converted to Biome format
- Ready to run with `cargo test prettier_tests`

---

## Step-by-Step Instructions

### Step 1: Clone Prettier Repository

```bash
# Clone prettier (can be anywhere, we just need the tests)
git clone https://github.com/prettier/prettier.git /tmp/prettier-repo

cd /tmp/prettier-repo
pnpm install  # Install dependencies
```

### Step 2: Copy the prepare_tests.js Script

The script is already in Biome:

```bash
cp crates/biome_formatter_test/src/prettier/prepare_tests.js \
   crates/biome_markdown_formatter/tests/specs/prettier/
```

### Step 3: Create Test Directory

```bash
cd crates/biome_markdown_formatter

# Create the specs directory if it doesn't exist
mkdir -p tests/specs/prettier
```

### Step 4: Run the Extraction Script

```bash
# From biome_markdown_formatter directory
cd crates/biome_markdown_formatter

# Run the prepare_tests.js script
# It expects the Prettier root directory as argument
node tests/specs/prettier/prepare_tests.js /tmp/prettier-repo
```

**What this does:**
- Finds all markdown test files in `/tmp/prettier-repo/tests/format/markdown/`
- Extracts their expected output from Jest snapshots
- Copies test input files to `tests/specs/prettier/`
- Creates `.prettier-snap` files with expected output

### Step 5: Verify Tests Were Extracted

```bash
# List extracted tests
ls -la tests/specs/prettier/

# Should see markdown test directories and files
# Example output:
# markdown/
#   array/
#   attributes/
#   ...
```

### Step 6: Run Tests

```bash
cargo test prettier_tests
```

**First run:**
- Tests might fail (snapshots being created)
- Or they might pass if Prettier tests match your formatter output

### Step 7: Review and Accept Snapshots

```bash
cargo insta review
```

For now with the default `format_verbatim_node`:
- Press `a` to accept all
- Snapshots are created

---

## What Gets Extracted

The script extracts:

1. **Test Input Files** - The markdown files to test
   - Stored in `tests/specs/prettier/markdown/...`

2. **Snapshots** - Expected formatted output
   - Stored as `.prettier-snap` files
   - Used by Biome's snapshot testing

3. **Prettier Snapshots** - Original unmodified output
   - Stored as `.prettier-snap-original` if different from reformatted output

---

## Example: Extracting Markdown Tests

After running the extraction script, you'll have something like:

```
tests/specs/prettier/
└── markdown/
    ├── array/
    │   ├── array.md
    │   └── array.md.prettier-snap
    ├── attributes/
    │   ├── attributes.md
    │   └── attributes.md.prettier-snap
    ├── emphasis/
    │   ├── emphasis.md
    │   └── emphasis.md.prettier-snap
    ├── headings/
    │   ├── setext.md
    │   ├── setext.md.prettier-snap
    │   ├── atx.md
    │   └── atx.md.prettier-snap
    ├── lists/
    │   ├── bullet.md
    │   ├── bullet.md.prettier-snap
    │   ├── ordered.md
    │   └── ordered.md.prettier-snap
    ...
```

Each `.md` file is a test input, each `.prettier-snap` is the expected output.

---

## Using with Biome's Test Infrastructure

Once extracted, use Biome's standard test workflow:

### 1. Make sure prettier_tests.rs exists

See `CREATE_MARKDOWN_TESTS.md` for how to set this up.

### 2. Make sure language.rs implements TestFormatLanguage

See `MARKDOWN_TESTS_QUICK_START.md` for the implementation.

### 3. Run tests

```bash
cargo test prettier_tests
```

### 4. Tests automatically discover all extracted test files

The `gen_tests!` macro in `prettier_tests.rs` will find all `.md` files in `tests/specs/prettier/`.

### 5. Snapshots are compared against .prettier-snap files

Biome's test infrastructure automatically uses the `.prettier-snap` files as expected output.

---

## How the Test Runner Uses .prettier-snap Files

**prettier_tests.rs** auto-discovers:
- All `.md` files: these are the test inputs
- All `.prettier-snap` files: these are the expected outputs

When you run `cargo test prettier_tests`:
1. Each `.md` file is formatted with your formatter
2. Output is compared to corresponding `.prettier-snap` file
3. If different, test fails and shows the diff
4. You review with `cargo insta review`

---

## Full Workflow Example

### Setup (one-time)

```bash
# 1. Clone prettier
git clone https://github.com/prettier/prettier.git /tmp/prettier

# 2. Install prettier deps
cd /tmp/prettier
pnpm install

# 3. Copy script to markdown formatter tests
cp path/to/biome/crates/biome_formatter_test/src/prettier/prepare_tests.js \
   path/to/biome/crates/biome_markdown_formatter/tests/specs/prettier/

# 4. Extract tests
cd path/to/biome/crates/biome_markdown_formatter
node tests/specs/prettier/prepare_tests.js /tmp/prettier

# 5. Run tests (will auto-generate snapshots)
cargo test prettier_tests

# 6. Review snapshots
cargo insta review
# Press 'a' to accept all
```

### After Setup

```bash
# Implement formatter features
vim src/markdown/auxiliary/heading.rs

# Validate with prettier-compare
bun packages/prettier-compare/bin/prettier-compare.js -f tests/specs/prettier/markdown/headings/atx.md

# Run tests
cargo test prettier_tests

# Review changes
cargo insta review
# Press 'y' for changes you're happy with
# Press 'n' to reject and fix code
```

---

## Important Notes

### 1. Script Location

The `prepare_tests.js` script is available in:
- `crates/biome_formatter_test/src/prettier/prepare_tests.js` (main version)
- `crates/biome_html_formatter/tests/specs/prettier/prepare_tests.js` (example copy)
- `crates/biome_json_formatter/tests/specs/prettier/prepare_tests.js` (example copy)

Copy from the main version in `biome_formatter_test`.

### 2. Script Expects Prettier Root

The script takes the Prettier repository root as an argument:

```bash
node prepare_tests.js /path/to/prettier/repo
```

It looks for tests in: `/path/to/prettier/repo/tests/format/markdown/`

### 3. Prettier Configuration

The script reformats Prettier's snapshots with Biome's default config:

```javascript
const defaultConfig = {
  trailingComma: 'all',
  tabWidth: 2,
  printWidth: 80,
  singleQuote: false,
  jsxSingleQuote: false,
  useTabs: false,
  embeddedLanguageFormatting: 'off'
};
```

This ensures tests are compatible with Biome's defaults.

### 4. File Extensions

- Input files: `.md` (unchanged from Prettier)
- Expected output: `.prettier-snap` (created by script)
- Original snapshots: `.prettier-snap-original` (if different from reformatted)

---

## Troubleshooting

### Issue: "Cannot find module 'prettier'"
**Solution**: Run `pnpm install` in the prettier repository root first

### Issue: "tests/format/markdown directory not found"
**Solution**: Make sure you cloned the full prettier repo with tests directory

### Issue: Script doesn't extract any files
**Solution**: Check that the path points to the prettier root (contains `tests/` directory)

### Issue: Tests don't auto-discover extracted files
**Solution**: Make sure `prettier_tests.rs` has correct `gen_tests!` macro with right glob pattern

### Issue: "snapshot not found" error
**Solution**: This is normal on first run. Run `cargo insta review` to create snapshots

---

## Advantages of Copying Prettier Tests

✅ **Complete coverage** - Hundreds of test cases from Prettier
✅ **Instant compatibility** - Tests define what Prettier does
✅ **Automated extraction** - No manual test creation needed
✅ **Easy updates** - Re-run script when Prettier updates tests
✅ **Validation tool** - REPORT_PRETTIER=1 shows exact differences
✅ **Regression prevention** - Catches any deviations from Prettier

---

## Generate Reports

After running tests with extracted Prettier tests, you can generate reports:

```bash
# Show all differences between Biome and Prettier
REPORT_PRETTIER=1 cargo test prettier_tests

# This generates: report.md with detailed comparison

# Show only incompatible cases
INCOMPATIBLE_ONLY=1 REPORT_PRETTIER=1 cargo test prettier_tests

# Output format: json or markdown
REPORT_TYPE=json REPORT_PRETTIER=1 cargo test prettier_tests
```

---

## Next Steps

1. Clone Prettier repo: `git clone https://github.com/prettier/prettier.git /tmp/prettier`
2. Run extraction: `node tests/specs/prettier/prepare_tests.js /tmp/prettier`
3. Set up `prettier_tests.rs` and `language.rs` (see CREATE_MARKDOWN_TESTS.md)
4. Run tests: `cargo test prettier_tests`
5. Review snapshots: `cargo insta review`
6. Start implementing formatters
7. Watch tests validate your implementation

---

## Reference

**Example implementations:**
- HTML formatter: `crates/biome_html_formatter/tests/specs/prettier/`
- JSON formatter: `crates/biome_json_formatter/tests/specs/prettier/`
- JS formatter: `crates/biome_js_formatter/tests/specs/prettier/`

**Documentation:**
- `crates/biome_json_formatter/tests/specs/prettier/README.md`
- `crates/biome_css_formatter/tests/specs/prettier/README.md`

**Script source:**
- `crates/biome_formatter_test/src/prettier/prepare_tests.js`

---

**That's it!** You now have an automated way to copy all of Prettier's markdown tests into Biome.
