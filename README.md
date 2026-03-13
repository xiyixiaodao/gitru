# gitru

## Git Commit Message Validation Tool

**gitru** is a lightweight, configurable **Git commit message validation tool** designed to
enforce [Conventional Commits](https://www.conventionalcommits.org/) standards across development teams.

---

### Installation

**Method 1:** (Recommended)

With Rust toolchain installed:

```bash
cargo install gitru
```

**Method 2:**  
Download pre-built [binary](https://github.com/xiyixiaodao/gitru/releases):

1. Get latest release for your OS
2. Add to system PATH
3. Set executable permissions:

---

### Usage

Install hook and initialize configuration:

```bash
gitru ii commit-msg
```

Command breakdown:

* `ii` = `install` (sets up git hook) + `init` (creates config template)
* Execute separately: `gitru install commit-msg` and `gitru init commit-msg`

For more options:

```bash
gitru --help
```

---

### Workflow

After installation:

* Git hook installed to `.git/hooks/commit-msg `
* Configuration template created at `.commit-msg-rule.toml`

Customize validation rules by editing `.commit-msg-rule.toml`.  
You can optionally modify, delete, or comment out the options that do not require validation.

#### configuration file example:

```toml
# The Conventional Commits 
#
# ╔═════════════════════════════════════════════════╗
# ║          COMMIT FORMAT TEMPLATE                 ║
# ╠═════════════════════════════════════════════════╣
# ║    type(optional scope): subject                ║
# ║                                                 ║
# ║    [optional body]                              ║
# ║                                                 ║
# ║    [optional footer]                            ║
# ║     - BREAKING CHANGE: xxxxx                    ║
# ║     - Closes #issue                             ║
# ╚═════════════════════════════════════════════════╝
#


[global]
version = "1.0.0"
enable_validation = true
skip_validation_words = [
    "--no-verify",
    "SKIP",
]


[header]
[header.type]
allowed_types = [
    "feat", # New feature
    "fix", # Bug fix
    "docs", # Documentation
    "style", # Code style
    "refactor", # Code refactor
    "test", # Test related
    "chore", # Maintenance
]


[header.scope]
required = false
allowed_scopes = [
    "core",
    "cli",
    "ui",
    "docs",
    "test",
]

[header.subject]
spaces_after_colon = 1
forbid_trailing_period = true  # Forbid ending with a period
min_length = 2                 # Default min_length is 2
max_length = 72                # Default max_length is 72


[body]
required = false
min_blank_lines_before_body = 1
forbid_trailing_whitespace = true
min_line_length = 2
max_line_length = 72


[footer]
start_key_words = ["BREAKING CHANGE", "Closes", "Fixes", "Signed-off-by"]
min_blank_lines_before_footer = 1
min_line_length = 2
max_line_length = 72
forbid_trailing_whitespace = true

# Whether to enable spell checking for footer keywords
[footer.start_key_words_spellcheck]
# When enabled, and the commit contains only a header + body (no footer),
# the body will be checked to determine whether it is a misspelled footer keyword.
enable = true
# Similarity threshold. Default is 0.7.
# When the similarity score exceeds this threshold, the text is considered a misspelling.
threshold = 0.7

```

### Commit validation example:

Validation success Example:

```bash
git commit -m "feat: add new API endpoint"
git commit -m "feat(core): add new API endpoint"
git commit -m "refactor(core)!: change public API"
git commit -m "feat: new feature" -m "BREAKING CHANGE: removes old API"

```

Validation Failure Example:   
(default config)

```bash
git commit -m "add feature" # type is missing
git commit -m "feat: a" # subject 'a' is too short
git commit -m "feat:add feature" # need space default
git commit -m "feat(): add feature"
git commit -m "feat(: add feature"
git commit -m "feat): add feature"
```

---

### Skip Validation

There are three ways to skip validation:

1. Add the `--no-verify` option when committing from the command line.
2. Manually choose “skip validation” in your IDE’s commit interface.
3. Write the keyword specified in the `skip_validation_words` option (from the configuration file) as the **first line
   **  
   of the commit message. This will automatically skip validation.  
   *Note: the keyword must appear alone on a single line.*

This means the following commit message will pass directly **without any validation**:

```
--no-verify

feat: add new feature
```

---

### Uninstall

remove `commit-msg` hook from `.git/hooks` directory:

```shell
gitru uninstall commit-msg 
```

remove `.commit-msg-rule.toml` file manually .
