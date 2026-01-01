## MIGRATION TODO

This document summarizes the migration of the TSU (TypeScript Utilities) package from TypeScript to Rust, creating the `mobdev` package.

## Remaining Works

### Stub Implementations (Future Work) 🚧

The following commands have stub implementations that need to be completed:

0. **Setup**
  - Local Lint checking and passing
  - Test coverage high and passing
  - CI workflows for linting and test coverage

1. **Claude CLI Integration**
   - `git commit-msg` - Generate commit messages
   - `git pr-description` - Generate PR descriptions

2. **Dart Tooling**
   - `dart fix` - Run dart fix
   - `dart dcm analyze` - Run DCM analysis
   - `dart changed downstream` - Find dependent files

3. **Hook Checks**
   - `hook format check` - Check Dart formatting
   - `hook analysis check` - Run dart analyze
   - `hook fix check` - Check dart fix
   - `hook dcm fix check` - DCM fix checks
   - `hook dcm analyze check` - DCM analyze checks
   - `hook graphql check` - GraphQL fakes check

4. **Other**
   - `git codeowners check` - CODEOWNERS validation
   - `check version` - Version checking
   - `upgrade` - Self-update functionality


