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

### Commit validation example:

```bash
git commit -m "feat: add new API endpoint"
git commit -m "feat(core): add new API endpoint"
```

Validation Failure Example:

```bash
git commit -m "add feature"
```

### Uninstall

remove `commit-msg` hook from `.git/hooks` directory: 

```shell
gitru commit-msg uninstall
```

remove `.commit-msg-rule.yaml` file manually .
