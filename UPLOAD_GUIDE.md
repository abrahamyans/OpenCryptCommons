# Upload This Starter Package to GitHub

## Recommended method: Git on Windows

Follow `DEVELOPMENT_WINDOWS.md`. This method preserves folders correctly and lets you test before publishing.

## Browser-only method

GitHub's browser interface can upload files, but creating many nested folders is inconvenient. Use it only when Git is unavailable.

1. Open the OpenCryptCommons repository.
2. Choose **Add file → Upload files**.
3. Drag the extracted starter package's files and folders into the upload area.
4. Confirm that folders such as `.github/workflows`, `crates/occ-core/src`, and `cli/src` remain correctly nested.
5. Use the commit message:

```text
Add Phase 1 Rust workspace and CLI foundation
```

6. Commit directly to `main` only when the repository owner accepts direct changes. Otherwise, create a new branch and pull request.
7. Open **Actions** and confirm that `Rust CI` completes successfully.

## Important

Keep the repository's existing licence, code of conduct, contribution, and security files. Never upload secret keys, passwords, tokens, real identity credentials, or confidential documents.
