# gitru

## Git Commit Message Validation Tool

> Enforces commit message conventions through configurable rules .  
(currently supports type、scope and subject validation, Additional features will be implemented gradually.)

### Installation

**Method 1:** (Recommended)

With Rust toolchain installed:

```bash
cargo install gitru
```

**Method 2:**  
Download pre-built binary:

1. Get latest release for your OS
2. Add to system PATH
3. Set executable permissions:

### Usage

Install hook and initialize configuration:

```bash
gitru commit-msg  ii
```

Command breakdown:

* `ii` = `install` (sets up git hook) + `init` (creates config template)
* Execute separately: `gitru commit-msg install` then `gitru commit-msg init`

### Workflow

After installation:

* Git hook installed to `.git/hooks/commit-msg `
* Configuration template created at `.commit-msg-rule.yaml`

Customize validation rules by editing `.commit-msg-rule.yaml`.  
You can optionally modify, delete, or comment out the options that do not require validation.

#### configuration file example:

```yaml
# Commit Message may like : .
# ╔══════════════════════════════════════════════╗
# ║    type(optional scope): subject             ║
# ╚══════════════════════════════════════════════╝
#
# Structure explanation:
# 1. type        → Required, commit type (feat/fix/docs etc)
# 2. (scope)     → Optional, scope (wrapped in parentheses)
# 3. : subject   → Required, brief description (space after colon)


# Type validation module
type:
  allowed_types:
    - feat
    - fix
    - docs
    - style
    - refactor
    - test
    - chore

# Scope validation module
scope:
#    allowed_scopes:
#      - core
#      - cli
#      - ui
#      - docs
#      - test

# Subject validation module
subject:
  require_space_after_colon: true #true is default value
  min_length: 2 # default min_length is 1
  max_length: 72 # default max_length is 72
```

### Commit validation example:

Validation success Example:

```bash
git commit -m "feat: add new API endpoint"
git commit -m "feat(core): add new API endpoint"
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

### Uninstall

remove `commit-msg` hook from `.git/hooks` directory:

```shell
gitru commit-msg uninstall
```

remove `.commit-msg-rule.yaml` file manually .
