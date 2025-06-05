# Troubleshooting Notes

## Buck2 Setup Issues

### Buck2 Download Failures
**Problem:** CI failing with "unsupported format" when downloading Buck2 releases  
**Root Cause:** GitHub's "latest" release URL redirecting to 404 page  
**Solution:** Use `dtolnay/install-buck2@latest` action instead of manual download

### Missing Toolchains
**Problem:** Buck2 expecting python_bootstrap and cxx toolchains  
**Root Cause:** Rust compilation requires multiple toolchain components  
**Solution:** Add all required toolchains to `toolchains/BUCK`:
```bzl
system_rust_toolchain(name = "rust", visibility = ["PUBLIC"])
system_python_bootstrap_toolchain(name = "python_bootstrap", visibility = ["PUBLIC"])
system_cxx_toolchain(name = "cxx", visibility = ["PUBLIC"])
```

### Platform Specification Errors
**Problem:** "Attempted to access configuration data for <unspecified> platform"  
**Root Cause:** Buck2 audit commands need explicit platform specification  
**Solution:** Add `--target-platforms=prelude//platforms:default` to audit provider commands

### Buck2 Caching Issues
**Problem:** Buck2 not recognizing new targets after file changes  
**Root Cause:** Aggressive caching of target discovery  
**Solution:** Run `buck2 clean` to clear cache when targets aren't recognized

## CI/CD Issues

### Clippy Nightly Variations
**Problem:** Clippy passing locally but failing in CI  
**Root Cause:** Nightly Rust has different clippy rules than stable  
**Solution:** Only run clippy on stable Rust: `if: matrix.rust == 'stable'`

### Rust 2024 Edition Compatibility
**Problem:** Potential edition compatibility issues across Rust versions  
**Root Cause:** Rust 2024 is relatively new (released Feb 2025)  
**Solution:** Verified working locally, kept 2024 edition, using latest toolchains

## Development Workflow

### Jujutsu Push Issues
**Problem:** `jj git push` not working for PR updates  
**Root Cause:** Not on a named bookmark, jj creates temporary branch names  
**Solution:** Use `jj squash` to combine changes, then `jj git push` to update PR

### Reindeer Configuration
**Problem:** Complex Buck2 + Rust dependency management  
**Root Cause:** reindeer generates complex Buck configuration  
**Solution:** Use `reindeer buckify` to generate, don't hand-edit generated files

## Build System Integration

### Dual Build System Sync
**Problem:** Keeping Cargo and Buck2 dependencies in sync  
**Root Cause:** Two different build systems with different formats  
**Solution:** Use reindeer to auto-generate Buck configs from Cargo.toml

### Cross-Platform CI
**Problem:** Different behavior across Linux/macOS/Windows  
**Root Cause:** Platform-specific toolchain differences  
**Solution:** Use GitHub's matrix strategy with consistent actions across platforms

## Useful Commands

### Buck2 Debugging
```bash
buck2 audit cell prelude                    # Check prelude configuration
buck2 targets toolchains//:                 # List available toolchains
buck2 audit providers //target:name --target-platforms=prelude//platforms:default
buck2 clean                                 # Clear all caches
```

### Jujutsu Workflow
```bash
jj squash                                    # Combine current changes with parent
jj git push -c @-                           # Push parent commit, auto-create branch
jj git push                                  # Update existing branch
jj bookmark create name -r @-                # Create named bookmark
```

### CI Debugging
```bash
gh pr checks <pr-number>                    # Check CI status
gh run view <run-id> --log-failed           # View failed run logs
gh pr create --head <branch> --title "..."   # Create PR with specific branch
```