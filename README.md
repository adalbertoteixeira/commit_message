# Commit message

Tool to automate the creation of PRs for JavaScript repositories.

## Highlights
- When a pre-commit hook fails, the next run automatically suggests the previously entered commit
message;
- Helps build a Github PR template and automatically check tasks.
- Extracts id and name from the branch name;
- Extracts type (and, in the future, scope) from the changed files.


## Install
Currently only manual build or release binary donwload is supported. There are plans to use a package manager to install the
tool using npm or similar.

### Download binary
Navigate to the releases tab and donwload the latest `commit_message` file. Currently only built for
macOs.

### Local build
Clone the repository and run

```{sh}
cargo build --release
```

Optionally, copy `./target/release/commit_message` to your repository. In the package.json add a script
entry like `"commit": "./scripts/commit_message"`.

Add `.commit_message` to your `.gitignore`. This step will be automated in the future.


## Usage

![demo](static_files/first.gif)

After the binary and the install command is set up, just run just run `[yarn | npm run | ...] commit`;

You can also trigger the tool manually using `./scripts/commit_message`.

You can use flags in the command. The most useful one is the custom commit message (`-m "..."`). 

If you are committing several PRs to the same branch then type, prefix and id shouldn't change or will be
extracted automatically, but the commit message will probably be different every time.


The first time the tool is run an editor setup prompt will appear. For now only terminal based
`$EDITOR`s are mentioned, plus how to install VSCode usage.


### Get help

Help is available at any time by running
```{sh}
yarn commit -h
```
![help](static_files/help.gif)


## @TODO
- [ ] add tests;
- [ ] add support for scopes;
- [ ] cleanup old commit message files;
- [ ] add support for other languages;
- [ ] installl the library using yarn;
- [ ] support for different systems (currently only macOS is tested);
- [ ] allow usage in Github Actions to automatically update the fields like labels based on scopes,
etc).
- [ ] allow using a config file at the repository level;
- [ ] automate adding`.commit_message` to the `.gitignore` file;
- [ ] allow reuse of existing PR template (changing fields if necessary).
