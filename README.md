# TREL Software Onboarding

A sandbox for practicing how we work, before you touch anything that matters.

This repo is a small mirror of
[trel3](https://trel-github.ae.utexas.edu/Texas-Rocket-Engineering-Laboratory/trel3):
same build system, same test conventions, same shape of CI. Six short exercises,
each one a pull request. The code is throwaway; the habits are the point.

- **Read [Where This Code Lives](#where-this-code-lives) before you clone.** It
  is short and it is the one section that is not optional.
- **A fresh clone does not pass.** Exercises ship broken on purpose. Red is the
  starting line, not a broken setup.

A few hours total, spread out however you like. Ask questions early - that is
expected, not a sign you are behind.

**Contents:** [Where This Code Lives](#where-this-code-lives) ·
[Setup](#setup) · [What Is In Here](#what-is-in-here) · [The Ladder](#the-ladder) ·
[Bazel](#bazel) · [Git And Pull Requests](#git-and-pull-requests) · [CI](#ci) ·
[Formatting And Linting](#formatting-and-linting) ·
[Writing A Good Test](#writing-a-good-test) · [Next Steps](#next-steps)

## Where This Code Lives

TREL software is export-controlled. These rules apply to trel3, and to this repo
too, because the habits you form in a sandbox are the ones you keep. TREL's
Technology Control Plan technically wants development to happen on TREL machines
over SSH, and those machines are unreliable with several people on them. None of
this is encouragement to work around the TCP - it is what makes working on a
personal machine defensible rather than reckless.

**1. Keep it on local disk.** Clone into a plain local directory:
`mkdir -p ~/dev && cd ~/dev`. Not a network share, not a shared lab machine, not
a cloud VM, not Codespaces. On macOS avoid `~/Documents` and `~/Desktop` even if
iCloud looks off today - the moment anyone enables "Desktop & Documents Folders"
sync, everything in them uploads. `~/dev` is never synced by default.

**2. Encrypt that disk.** Verified, not assumed:

| Platform | Mechanism | Check it |
| --- | --- | --- |
| macOS | FileVault | `fdesetup status` |
| Linux | LUKS / dm-crypt | `findmnt --target .` then `lsblk`, look for `crypt` |
| WSL | BitLocker on the **Windows** host | `manage-bde -status C:` (admin PowerShell) |

Use a real password and lock the machine after a few idle minutes. Encryption at
rest does nothing against an unlocked laptop.

**3. Do not put it in the cloud.** This is the one people get wrong by accident,
because most of these are on by default. No syncing or backing up the repo to
iCloud, Dropbox, Google Drive, OneDrive, Box, Backblaze, or similar - if you run
one, exclude your source directory and confirm the exclusion applied. An
*ITAR-compliant* cloud is fine; iCloud and your personal Google account are not.

No mirroring the code anywhere else either: not github.com, not a personal
GitLab, not a gist or pastebin. Same logic for AI tools - pasting this repo's
throwaway exercise code into a chatbot is fine, pasting trel3 source into one
means uploading export-controlled source to a third party. And do not travel
internationally with the machine while this code is on it.

**4. The Bazel cache counts too.** Bazel keeps an *output base* full of compiled
copies of repo sources (`bazel info output_base`). It is as sensitive as the
checkout, and big enough that people move it "somewhere with more space," which
is occasionally a synced folder. Keep it local.

**Check yourself:** `./dev_scripts/check_storage.sh` verifies the checkable
parts - synced paths, disk encryption, output base location. `setup.sh` runs it
for you. Passing does not make you compliant; failing means you are not.

Not legal advice. The authoritative document is
[NIST SP 800-171r3](https://csrc.nist.gov/pubs/sp/800/171/r3/final).

## Setup

macOS or Linux. On Windows, use WSL.

### Windows: WSL first

Building this natively on Windows is not worth anyone's time. WSL2 gives you real
Linux in about fifteen minutes.

1. In an **admin** PowerShell, then reboot when prompted:

   ```powershell
   wsl --install -d Ubuntu-24.04
   ```

2. Confirm the Windows drive is encrypted. This is the host's job - encrypting
   inside Linux does nothing for you. You want `Protection On`; if not, turn on
   BitLocker in Windows Settings.

   ```powershell
   manage-bde -status C:
   ```

3. Open Ubuntu and install the basics:

   ```bash
   sudo apt update && sudo apt install -y \
     build-essential curl git make tar libsecret-1-0 libsecret-1-dev
   git config --global credential.helper \
     /usr/share/doc/git/contrib/credential/libsecret/git-credential-libsecret
   ```

4. **Clone into your Linux home (`~/dev`), never `/mnt/c/...`.** Bazel across the
   Windows/Linux filesystem boundary is brutally slow, and anything under
   `/mnt/c` sits on the Windows drive where OneDrive can reach it.

5. For VS Code, install the **WSL** extension on Windows, then run `code .` from
   inside Ubuntu so the language servers run Linux-side.

If Bazel eats all your RAM, cap it in `C:\Users\<you>\.wslconfig`, then
`wsl --shutdown` and reopen:

```ini
[wsl2]
memory=8GB
```

### Everyone

1. **Do trel3's `Hardware Setup` section first** if you have not - command line
   tools, a Python version manager, a git credential manager.

2. **Install bazelisk, not bazel.** It reads `.bazelversion` and runs the exact
   Bazel this repo pins. `brew install bazelisk` on macOS; on Linux or WSL grab a
   binary from [bazelbuild/bazelisk](https://github.com/bazelbuild/bazelisk/releases)
   and put it on your `PATH` as `bazel`.

3. **Clone**, somewhere legal per the section above, then run
   **`./dev_scripts/setup.sh`**. The first run downloads a Rust toolchain and a
   full LLVM toolchain - a few minutes and a few GB, which buys you a build that
   behaves identically on your laptop and in CI. Later runs take seconds.

4. **VS Code** with the Rust Analyzer, Bazel, and C/C++ extensions is optional,
   but our tooling is built around it.

## What Is In Here

```
MODULE.bazel        <- every external dependency, declared once, repo-wide
.bazelrc            <- build flags, including rustfmt/clippy enforcement
tools/              <- clang-format and clang-tidy, pinned via Bazel
dev_scripts/        <- setup, storage check, formatting, C++ linting
rust_exercises/     <- exercises 1-4, in order
cpp_exercises/      <- two short exercises on C++ mechanics
.github/workflows/  <- CI
```

Read this file, then `rust_exercises/README.md`, then start on
`rust_exercises/ex1_engineering_units/EXERCISE.md`.

Every exercise has an `EXERCISE.md` with steps, a "Done when" list, and the
things that usually trip people up. The real teaching is in the source comments,
so read the code. Three sub-READMEs hold the detail this file only summarizes:
`rust_exercises/README.md` (Rust and testing conventions),
`cpp_exercises/README.md` (why C++ hygiene differs), and `dev_scripts/README.md`.

## The Ladder

In order. Each one is one pull request.

| # | Exercise | You learn |
| --- | --- | --- |
| 1 | `rust_exercises/ex1_engineering_units` | Bazel build/test, what a test looks like here, your first PR |
| 2 | `rust_exercises/ex2_sensor_limits` | Real Rust: enums, `match`, `Result`, borrowed slices |
| 3 | `rust_exercises/ex3_abort_debounce` | Write a failing test that exposes a real bug, then fix it. **The important one.** |
| 4 | `rust_exercises/ex4_lint_cleanup` | rustfmt, clippy, and what the `manual` tag costs you |
| 5 | `cpp_exercises/ex1_pressure_units` | `cc_library`, `cc_test`, GoogleTest |
| 6 | `cpp_exercises/ex2_format_and_tidy` | A target that builds green and is still unmergeable |

If one of these takes more than a couple of hours of real work, stop and ask.
That is a tooling problem, not a you problem, and it is ours to fix.

Python and TypeScript are not covered here even though trel3 uses both. The
Bazel, PR, and testing habits transfer directly.

## Bazel

Bazel is the least familiar thing here for most people. The payoff: hermetic
toolchains, so builds do not depend on what you have installed; caching, so
rebuilds are fast; and an exact dependency graph, so CI knows what your change
could have broken. trel3's `README.bazel.md` has the long version.

A **target** is something Bazel can build, named by a **label** like
`//rust_exercises/ex1_engineering_units:tests` - a directory containing a
`BUILD.bazel`, then a target name defined in it. `//foo/bar/...` means everything
at or below that directory; `//...` is the whole repo.

Bazel only sees what a `BUILD.bazel` declares, so a source file not listed in
some `srcs` does not exist as far as Bazel is concerned. That is the trade: you
write your dependencies down, and Bazel can answer questions about them.

| Command | What it does |
| --- | --- |
| `bazel build //path:target` | Build it. For Rust this also runs rustfmt and clippy. |
| `bazel build //...` | Build everything (skips targets tagged `manual`) |
| `bazel test //path:target` | Build and run a test |
| `bazel test //...` | Run every test |
| `bazel test //path:target --test_output=all` | Show output from passing tests too |
| `bazel run //path:target` | Build and execute a binary |
| `bazel query 'deps(//path:target)'` | What it depends on |
| `bazel query 'rdeps(//..., path/to/file.rs)'` | What depends on that - *what did I just break?* |

`rdeps` is what CI uses to decide what to run on your PR. Worth trying on your
own changes before you push.

**When Bazel is being annoying:**

| Symptom | Cause |
| --- | --- |
| "no such target" / "no such package" | Wrong label, or no `BUILD.bazel` there. Check `bazel query //...`. |
| Your test will not re-run | Cached. `--nocache_test_results`. |
| A source change is ignored | The file is not in any `srcs`. |
| A target is missing from `//...` | It is tagged `manual`. See `rust_exercises/ex4_lint_cleanup`. |
| Genuinely wedged | `bazel clean`. `--expunge` re-downloads every toolchain, so ask first. |

Asking an AI for Bazel help? Say you use **bzlmod** (`MODULE.bazel`), not
`WORKSPACE`, or you get confident, obsolete answers.

## Git And Pull Requests

Our GitHub has SSH disabled, so git uses HTTPS with a Personal Access Token as
your password. Generate a fine-grained token at
`https://trel-github.ae.utexas.edu/settings/personal-access-tokens`, scoped to
the Texas-Rocket-Engineering-Laboratory org, with Metadata: read and Contents:
read and write, and no organization permissions. Username is your EID, password
is the token; a credential manager means you type it once. Full steps are in
trel3's README. Treat the token like a password - never in a file, a commit, or a
chat message.

**Never commit to `main`.** Always a branch, always a PR.

| Command | What it does |
| --- | --- |
| `git status` | What changed and which branch you are on. Run it constantly. |
| `git switch -c my-branch` | Create and switch to a branch |
| `git diff` / `git diff --staged` | Unstaged changes / what you are about to commit |
| `git add -p` | Stage hunk by hunk. Best habit on this list. |
| `git push -u origin my-branch` | Push a new branch and track it |
| `git fetch origin` / `git rebase origin/main` | Update refs / replay your commits on current main |
| `git push --force-with-lease` | Update a branch you rebased or amended, safely |
| `git restore <file>` / `git reset --soft HEAD~1` | Discard changes to a file / undo the last commit, keeping changes |

Use `--force-with-lease`, never bare `--force`. It refuses to push if someone
else pushed in the meantime, which is the exact accident force-push is famous
for.

### The loop

```bash
git switch main && git fetch origin && git rebase origin/main
git switch -c ex1-to-counts-tests

# work, then run what you are about to ask someone to review
./dev_scripts/format.sh
bazel test //rust_exercises/ex1_engineering_units:tests

git add -p
git commit

# read your own diff before anyone else does
git diff origin/main...HEAD

git push -u origin ex1-to-counts-tests
```

Then open the PR in the web UI and fill in the template. Open it as a **draft**
if it is not ready - drafts do not ask for review or run the full CI suite.

Reading your own diff is the step people skip and the one that pays. It catches
the debug `println!`, the commented-out block, the file you did not mean to
touch.

### Commit messages

Written for whoever is bisecting a regression in six months. Usually you.

```
Fix abort debounce not resetting on a false cycle

run() incremented the consecutive-cycle counter when the condition held but
never cleared it when the condition went false, so a flickering condition
still fired once the total count reached min_cycles.
```

First line imperative and under ~70 characters ("Fix X", not "Fixed X"). Blank
line, then what and **why** - the diff already shows how. One logical change per
commit; `git add -p` makes that easy.

### Keep PRs small

One self-contained change: the codebase makes sense before it and after it. Big
enough to mean something, small enough to follow in 15-30 minutes.

Small PRs actually get reviewed, because nobody needs to find a free hour. They
are easy to revert. And a reviewer who dislikes one piece of a 600-line PR holds
up all 600 lines. This is the hardest professional habit to pick up - student
projects never have reviewers - and the one your future coworkers will most
appreciate you already having.

The `PR Hygiene` workflow warns above 300 changed lines and fails above 600 -
smell detectors, not laws.

Review comments are about the code, not about you. Answer every one, even if the answer
is "done." If you disagree, say so and say why - reviewers are wrong sometimes.
Push fixes as new commits during review so the reviewer can see what changed.

What you will most likely hear:

- "This PR does three things. Split it."
- "What happens when this is empty, zero, or missing?"
- "There is no test for the branch you added."
- "Would this test fail if the code were wrong? Show me."

And the big one: **if the diff needs a verbal explanation, the PR is not ready.**

## CI

Every non-draft PR triggers workflows, and they have to pass to merge.

| Workflow | Does |
| --- | --- |
| `Rust Build & Lint, Test` | Builds affected Rust targets (which runs rustfmt and clippy), then tests them |
| `C++ Build & Test` | Same, for `cc_*` targets |
| `C++ Format & Tidy` | clang-format and clang-tidy on the C++ files your PR changed |
| `PR Hygiene` | Checks the PR has a real description and is reviewable in one sitting |

There is no separate Rust lint job on purpose: `.bazelrc` attaches the rustfmt
and clippy aspects to every build, so a Rust build failure is as likely to be a
formatting problem as a compile error.

**When it fails**, read the actual error in the failed step - usually the last 20
lines, past the Bazel progress spam - then reproduce it locally with the same
command. The build is hermetic, so if CI fails and your machine passes, suspect
something you did not commit and check `git status`.

**Affected targets.** The workflows do not build everything. They ask Bazel which
targets your changed files could affect - `rdeps(all targets of that kind, files
you changed)` - which is an answer rather than a guess. At trel3's scale that
means a change to `message_formats` automatically tests the flight and ground
software consuming it, with nobody maintaining a list.

## Formatting And Linting

The rule from trel3: **code with failing lint, tests, or build does not get
merged.** A lint ignore for a stated, legitimate reason is fine; one used to make
a message go away is not. How it gets enforced depends on the language:

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` | `clang-format`, separate tool |
| Linter | `clippy`, run by `bazel build` | `clang-tidy`, separate tool |
| Can unformatted code build? | **No.** Build fails with a diff. | Yes. |
| Are lint findings errors? | **Yes.** Hard build failure. | Only where a script or CI runs them |

In Rust you cannot forget. In C++ you can, so it needs discipline Rust gets for
free - `cpp_exercises/README.md` covers that.

```bash
./dev_scripts/format.sh           # format all Rust and C++ in place
./dev_scripts/format.sh --check   # report only, fail if dirty
./dev_scripts/lint_cpp.sh         # clang-tidy over C++ sources
bazel build //...                 # this IS the Rust lint
```

Both C++ tools come from the LLVM toolchain Bazel downloaded, so your output
matches CI's exactly. Do not `brew install llvm` and use that instead -
clang-format 18 and 20 disagree about real code, and the repo churns.

Tools only catch mechanical problems. Reviewers check the rest: **YAGNI**,
[**SOLID**](https://en.wikipedia.org/wiki/SOLID), **DRY**, **KISS**, and tests
reaching about 90% coverage where reasonable.

## Writing A Good Test

Why we care: so we know your code works, so we find out when someone else's
change breaks it, and because tests document what a function should do. That last
one matters more here than at a company - every year the most experienced people
on this team graduate and take whatever they did not write down with them.

The whole shape, lightly abbreviated from `rust_exercises/ex1_engineering_units`:

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
  combinations. Exercise 3 adds `#[fixture]` for shared setup, which is how the
  real ground software tests work.

The rest, briefly:

- Walk each path through the function and test each one. Boundaries especially:
  if a threshold is inclusive, some test should fail if it were made exclusive.
- One behavior per test, and no logic in tests - no `if` deciding what to assert,
  no computing the expected value with the formula the code uses. Loops that just
  repeat a call are fine.
- `.expect("message")` rather than `.unwrap()`, so a failure is a sentence.
- Nothing `pub` in a `tests` module. Rust tests live in the same file as the code.
- C++: GoogleTest, `EXPECT_*` unless continuing would crash, and `EXPECT_NEAR` or
  `EXPECT_DOUBLE_EQ` for floats - `0.1 + 0.2 == 0.3` is false.
- **Check that your test can fail.** Break the code on purpose and confirm it
  catches it. That is what exercise 3 is about.

Full conventions: `rust_exercises/README.md`, and trel3's `README.testing.md`.

## Next Steps

**When you are stuck, ask.** Everyone here had to learn all of this, and
answering questions is how we find out which docs are wrong. Give it a bounded
try first - say 30 minutes - then bring what you were trying to do, the exact
command, the actual error pasted rather than paraphrased, and what you tried.
That makes you answerable in one message instead of five. Known rough edges are
in trel3's `README.known_issues.md`. And if you learn something that should have
been written down, write it down - a PR against this README is a real
contribution.

Once your six PRs are merged, read these in trel3, in order: `README.md`,
`README.new_member_tips.md` (which links notable past PRs), `README.bazel.md`,
`README.testing.md`, `README.known_issues.md`, `dev_scripts/README.md`.

Then take a small real task. A missing test on an existing function is an
excellent first real PR: no design decisions, real code, real reviewers.

**How this repo differs from trel3:**

| | Here | trel3 |
| --- | --- | --- |
| Languages | Rust, a little C++ | Rust, Python, TypeScript; C++ in its own repo |
| Rust targets | host platforms only | plus `thumbv7em` for scopium boards and `wasm32` |
| CI runners | GitHub-hosted | self-hosted AWS runners, ephemeral per run |
| Bazel cache | local only | shared remote cache via `--config=ci` |
| Breaking `main` | harmless | you broke it for everyone |

If this repo moves onto TREL's GitHub Enterprise Server, the `runs-on:` lines in
`.github/workflows/` need trel3's self-hosted labels, since GHES has no
GitHub-hosted runners. Each workflow file says exactly what to change.
