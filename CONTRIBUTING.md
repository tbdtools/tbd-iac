# Contributing to TBD IAC

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages. This allows us to automatically generate changelogs and determine semantic version bumps.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: A new feature (correlates with MINOR in SemVer)
- `fix`: A bug fix (correlates with PATCH in SemVer)
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools
- `ci`: Changes to CI configuration files and scripts

### Breaking Changes

Add `BREAKING CHANGE:` in the commit footer to trigger a major version bump:

```
feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other config files
```

### Examples

```
feat(cli): add provider install command
fix(state): handle concurrent state updates
docs(readme): update installation instructions
test(init): add test for project initialization
```

## Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write/update tests
5. Run tests locally: `cargo test`
6. Push changes
7. Create Pull Request

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_init_creates_project_structure

# Run with logging
RUST_LOG=debug cargo test
```
