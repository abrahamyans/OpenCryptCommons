# Windows Development Guide for Non-Professionals

## 1. Install Git

1. Open the Git for Windows website.
2. Download the installer.
3. Run it and keep the recommended options.
4. Restart PowerShell after installation.
5. Check the installation:

```powershell
git --version
```

## 2. Install Rust

1. Open the official Rust installation website.
2. Download and run `rustup-init.exe`.
3. Choose the standard installation option.
4. Close and reopen PowerShell.
5. Check the installation:

```powershell
rustc --version
cargo --version
```

## 3. Download the repository

```powershell
cd $HOME\Documents
git clone https://github.com/abrahamyans/OpenCryptCommons.git
cd OpenCryptCommons
```

## 4. Add the starter files

Copy the contents of this starter package into the downloaded `OpenCryptCommons` folder.

When Windows asks whether to replace `README.md`, choose **Replace** only after keeping a backup of the old file.

Do not delete the existing:

- `LICENSE.txt`;
- `CODE_OF_CONDUCT.md`;
- `CONTRIBUTING.md`;
- `SECURITY.md`.

## 5. Check the code

Run:

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

A successful test ends with text similar to:

```text
test result: ok
```

## 6. Run the example

```powershell
cargo run -p occ-cli -- sample --output-dir examples/generated
cargo run -p occ-cli -- credential-check --credential examples/sample-credential.json
cargo run -p occ-cli -- policy-check --policy examples/sample-policy.json --approvals examples/sample-approvals.json
```

## 7. Upload the changes to GitHub

```powershell
git status
git add .
git commit -m "Add Phase 1 Rust workspace and CLI foundation"
git push origin main
```

After the push, open the repository's **Actions** page. The Rust CI workflow should run automatically.

## 8. When something fails

Copy all terminal text beginning from the command you entered through the final error line. Do not send passwords, private keys, access tokens, or confidential credential data.
