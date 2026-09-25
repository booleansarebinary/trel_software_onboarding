# TREL Software Onboarding

Welcome!

We hope this repo gives you practice with the tools TREL Software uses so that when you get access to the repo, you hit the ground running.

This should take around 2-4 hours to go through; if it takes much longer, please reach out! The goal is not to master these tools, but to become familiar with them.

As for AI Use: I would recommend trying to do these exercises without AI, so you develop the habit. Ask us for pointers instead; your team is here for you!

- **Read [Where This Code Lives](#where-this-code-lives) before you clone.**
- **A fresh clone does not pass.** Some of these exercises are broken on purpose for you to fix!

**Contents:** [Where This Code Lives](#where-this-code-lives) · [Setup](#setup) ·
[What Is In Here](#what-is-in-here) · [The Ladder](#the-ladder) · [Bazel](#bazel) ·
[Git And Pull Requests](#git-and-pull-requests) · [CI](#ci) ·
[Formatting And Linting](#formatting-and-linting) ·
[Writing A Good Test](#writing-a-good-test) · [Next Steps](#next-steps)

## Where This Code Lives

TREL software is export-controlled. So keep the following in mind when you get access to our main repo. Ideally, we should be sshing into the TREL PCs and developing on them, but we're not able to host our software on those computers, so this is our workaround.

**1. Keep the repo on your local disk.** Clone into a plain local directory:
`mkdir -p ~/dev && cd ~/dev`. Don't use Documents/Desktop on your Mac or Windows (even if it's not currently in the Cloud), don't store it in iCloud or any cloud.

**2. Encrypt your disk.**

| Platform | Mechanism | Check it |
| --- | --- | --- |
| macOS | FileVault | `fdesetup status` |
| Linux | LUKS / dm-crypt | `findmnt --target .` then `lsblk`, look for `crypt` |
| WSL | BitLocker on the **Windows** host | `manage-bde -status C:` (admin PowerShell) |

**3. Do not put it in the cloud.** Again, repeating this point because it's very important to meet ITAR regulations.

**4. The Bazel cache counts too.** Bazel keeps an *output base* full of compiled
copies of repo sources (`bazel info output_base`). So this needs to be local too.

**A note on AI use:** For now, leadership has okayed using AI to help code. In the future, when software becomes more maintenance work, we might be able to look into creating local models for TREL to use.

## Setup

macOS or Linux. On Windows, use WSL.

### Windows: WSL

Our repo doesn't support Windows, so the workaround is WSL.

1. In an **admin** PowerShell, then reboot when prompted:
   `wsl --install -d Ubuntu-24.04`

2. Check the Windows drive is encrypted with `manage-bde -status C:` (still in
   admin PowerShell). You want `Protection On`. If not, turn on BitLocker in
   Windows Settings. It has to be the Windows drive; encrypting inside Linux
   doesn't count.

3. Open Ubuntu and follow the **Linux** steps below, inside it.

4. **Clone into your Linux home (`~/dev`), never `/mnt/c/...`.** `/mnt/c` is the
   Windows drive: Bazel is very slow there, and OneDrive can reach it.

5. For VS Code, install the **WSL** extension on Windows, then run `code .` from
   inside Ubuntu.

If Bazel eats all your RAM, cap it in `C:\Users\<you>\.wslconfig` with
`[wsl2]` and `memory=8GB`, then `wsl --shutdown` and reopen.

Note: I don't have a Windows computer, so if you find that this does not accurately reflect your experience, make a PR to change the steps.

### macOS

```bash
xcode-select --install                # git and the system build tools
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install bazelisk
brew install --cask git-credential-manager
git config --global credential.helper manager
```

### Linux

```bash
sudo apt update && sudo apt install -y \
  build-essential curl git make tar libsecret-1-0 libsecret-1-dev
git config --global credential.helper \
  /usr/share/doc/git/contrib/credential/libsecret/git-credential-libsecret
```

There's no apt package for bazelisk, so copy this whole block. It picks the
right build for your CPU, checks the download isn't corrupted, and installs it as
`bazel`:

```bash
ARCH=$(dpkg --print-architecture)          # prints amd64 on most machines, arm64 on ARM
BAZELISK=v1.29.0
BASE=https://github.com/bazelbuild/bazelisk/releases/download/$BAZELISK

curl -fsSLo /tmp/bazelisk "$BASE/bazelisk-linux-$ARCH"
curl -fsSLo /tmp/bazelisk.sha256 "$BASE/bazelisk-linux-$ARCH.sha256"
echo "$(cat /tmp/bazelisk.sha256)  /tmp/bazelisk" | sha256sum -c -

sudo install -m 755 /tmp/bazelisk /usr/local/bin/bazel
```

The `sha256sum -c -` line should print `/tmp/bazelisk: OK`. If it says `FAILED`,
stop and tell us instead of installing it.

Check it worked, from inside this repo:

```bash
bazel --version     # should print: bazel 8.5.1
```

(Not on Ubuntu or WSL? The `dpkg` line won't work. Use `uname -m` instead:
`x86_64` means `amd64`, and `aarch64` means `arm64`.)

**Install bazelisk, not bazel,** on either platform. It reads `.bazelversion` and
runs the exact Bazel version this repo pins (8.5.1), so everyone is on the same
one.

The `git-credential-manager` / `libsecret` lines are optional. They save you
retyping your password on every push over HTTPS. Skip them if you use SSH keys.

### Then, everyone

1. **Clone it** somewhere local (see
   [Where This Code Lives](#where-this-code-lives)):

   ```bash
   mkdir -p ~/dev && cd ~/dev
   git clone <paste the URL from the repo's "Code" button>
   cd trel_software_onboarding
   ```

   SSH or HTTPS both work. If git asks for credentials and you're not sure what
   it wants, just ask us. It's a two-minute fix.

2. **Run `./dev_scripts/setup.sh`.** The first run downloads the Rust and C++
   toolchains (the compiler and everything that goes with it). It takes a few
   minutes and a few GB the first time, then seconds after that. Bazel uses these
   instead of whatever is on your laptop, so the build is the same for everyone.

3. **VS Code** with the Rust Analyzer, Bazel, and C/C++ extensions is optional,
   but it'll make your life easier. It also formats your code on save.

## What Is In Here

```
MODULE.bazel        <- every external dependency, declared once, repo-wide
.bazelrc            <- build flags, including rustfmt/clippy enforcement
tools/              <- clang-format and clang-tidy, pinned via Bazel
dev_scripts/        <- setup, storage check, formatting, C++ linting
rust_exercises/     <- the two Rust exercises, in order
cpp_exercises/      <- the two C++ exercises, after the Rust pair
.github/workflows/  <- CI
```

Finish reading this file, then read `rust_exercises/README.md`, then start on
`rust_exercises/ex1_engineering_units/EXERCISE.md`.

Every exercise has an `EXERCISE.md` with the steps and a "Done when" list, and a
`HINTS.md` that explains the syntax it uses. The source comments explain a lot
too, so read the code.

## The Ladder

Create an individual pull request (PR) on a separate branch for every one of these.

| Exercise | Time | You learn |
| --- | --- | --- |
| `rust_exercises/ex1_engineering_units` | 45 min | Bazel, rustfmt, clippy, a bit of Rust, the house test style, your first PR |
| `rust_exercises/ex2_abort_debounce` | 45 min | Write a failing test that exposes a real bug, then fix it. **The important one.** |
| `cpp_exercises/ex1_pressure_units` | 25 min | `cc_library`, `cc_test`, GoogleTest |
| `cpp_exercises/ex2_format_and_tidy` | 20 min | A target that builds green and is still unmergeable |

If you find these exercises are taking you too long, stop and ask someone. It could be an issue with the tools and not your code.

## Bazel
Bazel is...complicated. But it's worth working with because it ensures compile-time consistency for every computer. So you never have the problem of "the rocket code works on my computer but not theirs." (Pretty dangerous when launching a rocket.)

A **build** is compiling and linking source code into something usable like a
library, an executable, or a test binary.

A **target** is one named unit of that work, declared in a `BUILD.bazel` file.
The declaration says what kind of thing to produce, which source files go into
it, and what it depends on. Here is the real one from exercise 1:

```python
rust_library(                       # produce a library
    name = "engineering_units",     # call it this
    srcs = glob(["src/**/*.rs"]),   # out of these files
    edition = "2024",
)

rust_test(                          # produce a test binary
    name = "tests",
    crate = ":engineering_units",   # testing the library above
    edition = "2024",
    deps = ["@crates//:rstest"],    # which also needs rstest
)
```

You refer to a target by its **label**: the path to the directory holding the
`BUILD.bazel`, a colon, then the target name. So those two are
`//rust_exercises/ex1_engineering_units:engineering_units` and
`//rust_exercises/ex1_engineering_units:tests`. `//foo/bar/...` means every
target at or below that directory, and `//...` is the whole repo.

Why declare all this? Because then Bazel knows exactly what depends on what. It
can skip rebuilding anything that didn't change, and CI can work out which
targets your PR could have broken. The catch: if a file isn't listed in some
`srcs`, Bazel doesn't know it exists.

The commands you'll use the most often are bazel build and bazel test.

| Command | What it does |
| --- | --- |
| `bazel build //path:target` | Build it. For Rust this also runs rustfmt and clippy. |
| `bazel build //...` | Build everything (skips targets tagged `manual`) |
| `bazel test //path:target` | Build and run a test |
| `bazel test //...` | Run every test |
| `bazel test //path:target --test_output=all` | Show output from passing tests too |
| `bazel query 'deps(//path:target)'` | What it depends on |
| `bazel query 'rdeps(//..., path/to/file.rs)'` | What depends on that - *what did I just break?* |

`rdeps` is what CI uses to decide what to run on your PR.

**When Bazel is being annoying:**

| Symptom | Cause |
| --- | --- |
| "no such target" / "no such package" | Wrong label, or no `BUILD.bazel` there. Check `bazel query //...`. |
| Your test will not re-run | Cached. `--nocache_test_results`. |
| A source change is ignored | The file is not in any `srcs`. |
| A target is missing from `//...` | It is tagged `manual` in its `BUILD.bazel`, which keeps it out of wildcards. |
| Genuinely wedged | `bazel clean`. `--expunge` re-downloads every toolchain, so ask first. |

One gotcha when you search online: Bazel has an old setup style (a `WORKSPACE`
file) and a new one (**bzlmod**, using `MODULE.bazel`). We use bzlmod. Most
answers online, and from AI, are for the old style and won't work here, so add
"bzlmod" to your search.

## Git And Pull Requests

GitHub helps us track our code and changes. After pulling the code to your machine, create a branch of your own. You can make changes on this branch without touching the production code. When you're done with your branch, you can push it and create a PR for it. Again, if you have questions or concerns, please ask, we'd rather answer the same question twice than debug tooling :).

Our main repo has a ticket system that should make this more intuitive.

**Never commit to `main`.**

| Command | What it does |
| --- | --- |
| `git status` | What changed and which branch you are on. Run it constantly. |
| `git switch -c my-branch` | Create and switch to a branch |
| `git diff` / `git diff --staged` | Unstaged changes / what you are about to commit |
| `git add .` | Gets changes ready to be committed.
| `git push -u origin my-branch` | Push a new branch and track it |
| `git fetch origin` / `git rebase origin/main` | Update refs / replay your commits on current main |
| `git push --force-with-lease` | Update a branch you rebased or amended, safely |
| `git restore <file>` / `git reset --soft HEAD~1` | Discard changes to a file / undo the last commit, keeping changes |

Use `--force-with-lease`, never bare `--force`. It refuses to push if someone
else pushed in the meantime, which is the exact accident force-push is famous
for.

### The loop

```bash
git switch main && git pull
git switch -c ex1-to-counts-tests

# The -c stands for create. Omit it when switching to an existing branch.

# Work, then run what you are about to ask someone to review
./dev_scripts/format.sh rust_exercises/ex1_engineering_units
bazel test //rust_exercises/ex1_engineering_units:tests

# . is for all files you touched, you can name specific files as well.
git add .
git commit -m "YOUR_COMMIT_MESSAGE_HERE"
git diff origin/main...HEAD   # don't forget to check this step!
git push -u origin ex1-to-counts-tests
```

Then open the PR in the web UI and fill in the template: what it changes, and
why. Open it as a **draft** if it's not ready but you want to ask someone
questions.

### Commit messages

Keep them short and descriptive enough that if you have to go back to the version of the code at that commit, you understand what changed and what didn't. For example:

```
Fixed graph on dashboard page to update immediately.
```

If you stick around software engineers for long enough, you know that they have very strong opinions on code hygiene, commit messages, and seemingly insignificant details. We're trying to find a middle ground: detailed enough that we eliminate many small and careless bugs, but not so strict as to waste time.

[With this in mind, do you think the author of this repo made good commits when creating this repo 😜?]

### Keep PRs small

This really helps your reviewer see your changes. Ideally, keep it to a few hundred lines of changes. (The ticketing system will make this easier because it will define your task for you.)

You don't need to do it for this repo, but for our other repo, once you make a PR, send a message in the chat and link the PR. Someone should respond and start to review your PR.

In a month or so, you will be reviewing our PRs!

## CI

Don't worry about this too much now. It's a suite of automated tests that run whenever we want to merge something. It checks that the code won't break anything else.

## Formatting And Linting

The rule: **code with failing lint, tests, or build doesn't get merged.**
Turning off a lint warning is fine if you write down a real reason, but not just
to make the message go away. How it's enforced depends on the language:

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` | `clang-format`, separate tool |
| Linter | `clippy`, run by `bazel build` | `clang-tidy`, separate tool |
| Can unformatted code build? | **No.** Build fails with a diff. | Yes. |
| Are lint findings errors? | **Yes.** Hard build failure. | Only where a script or CI runs them |

In Rust you can't forget. In C++ you can, so it's up to you (and CI).

```bash
./dev_scripts/format.sh             # format all Rust and C++ in place
./dev_scripts/format.sh <folder>    # format just one folder, like an exercise
./dev_scripts/format.sh --check     # report only, fail if anything needs formatting
./dev_scripts/lint_cpp.sh           # clang-tidy over C++ sources
bazel build //...                   # this IS the Rust lint
```

**Do you have to run `format.sh` every time?** Not for Rust. `bazel build` already
checks formatting and fails if it's off. `format.sh` is how you *fix* it, so you
don't fix whitespace by hand. For C++ nothing in the build checks, so run the
scripts yourself (or let CI catch it).

Easiest of all: open the repo in VS Code after running setup, and it formats on
save.

Use these scripts rather than a clang-format you installed yourself (like
`brew install llvm`). Different versions format the same file differently, and
the scripts use the exact version CI does.

## Writing A Good Test

Very very important! At one point (maybe even currently?) our codebase had more lines of code that was test code as opposed to feature code. It may seem excessive: do we really need tests in an MVP?

The answer is it depends on the kind of test. If we were implementing heavy tests that you might find in a CD (continuous deployment) pipeline, we would not be making a wise decision. But if we don't have unit tests, when something breaks, it's extremely difficult to figure out where the problem is. This takes less time in the long run.

Here's an example from `rust_exercises/ex1_engineering_units`:

```rust
#[rstest]                                  // Always rstest, never #[test].
#[case(0, -10.0)]                          // Three cases, one body. Each is
#[case(100, 0.0)]                          // reported separately, so a failure
#[case(65535, 6543.5)]                     // tells you which input broke.
fn test_from_counts_applies_scale_and_offset(#[case] counts: u16, #[case] expected: f64) {
    let channel = EngineeringUnit::new(0.1, -10.0);   // setup

    let value = channel.from_counts(counts);          // the code under test

    assert!((value - expected).abs() < EPSILON);      // assertions
}
```

- **The name reads as a sentence**, so a CI failure tells a reviewer what broke
  without opening the file: `test_<function>_<behavior>[_when_<condition>]`.
- **Three blocks, one blank line between them:** setup, the code under test,
  assertions. This one is checked in review.
- **`#[case]`** adds inputs without duplicating the body; `#[values]` gives you
  combinations. Exercise 2 adds `#[fixture]` for shared setup, which is how the
  real ground software tests work.

The rest, briefly:

- Walk each path through the function and test each one, boundaries especially: if
  a threshold is inclusive, some test should fail if it were made exclusive.
- One behavior per test, and no logic in tests - no `if` deciding what to assert,
  no computing the expected value with the formula the code uses. Loops that just
  repeat a call are fine.
- `.expect("message")` rather than `.unwrap()`, so a failure explains itself.
  Nothing `pub` in a `tests` module. Rust tests live in the same file as the code.
- C++: GoogleTest, `EXPECT_*` unless continuing would crash, and `EXPECT_NEAR` or
  `EXPECT_DOUBLE_EQ` for floats - `0.1 + 0.2 == 0.3` is false.
- **Check that your test can fail.** Break the code on purpose and confirm it
  catches it. That is what exercise 2 is entirely about.

Full conventions: `rust_exercises/README.md`.

## Next Steps

Try giving the exercises a shot. And reach out with any questions!

If you want more depth: [the Rust Book](https://doc.rust-lang.org/book/),
[rstest's docs](https://docs.rs/rstest/), and
[Bazel's guides](https://bazel.build/start).

