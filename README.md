# TREL Software Onboarding

A sandbox for learning how we work before you touch anything that matters.

Everything here is a small, deliberately simplified mirror of
[trel3](https://trel-github.ae.utexas.edu/Texas-Rocket-Engineering-Laboratory/trel3):
same build system, same test conventions, same hygiene rules, same shape of CI.
The code is throwaway. The habits are not.

**Two things to know before you start.**

First, read the [Where This Code Lives](#where-this-code-lives) section below
before you clone anything. It is not boilerplate and it is not optional.

Second, **a fresh clone of this repo does not pass.** Exercises ship broken on
purpose - that is the assignment. When `bazel test //...` comes back red, your
setup is fine.

Budget four to six hours across a few sittings. Nobody is timing you. Skimming
this README and guessing at the exercises will take longer than reading it.

## Table of Contents

- [Where This Code Lives](#where-this-code-lives)
  - [Keep it on your own disk](#keep-it-on-your-own-disk)
  - [Encrypt that disk](#encrypt-that-disk)
  - [Do not put it in the cloud](#do-not-put-it-in-the-cloud)
  - [The Bazel cache counts too](#the-bazel-cache-counts-too)
  - [Check yourself](#check-yourself)
- [Setup](#setup)
- [How To Navigate This Repo](#how-to-navigate-this-repo)
- [The Ladder](#the-ladder)
- [Bazel](#bazel)
  - [The mental model](#the-mental-model)
  - [Commands you will actually use](#commands-you-will-actually-use)
  - [When Bazel is being annoying](#when-bazel-is-being-annoying)
- [Git](#git)
  - [Authenticating with our GitHub](#authenticating-with-our-github)
  - [Commands you will actually use](#commands-you-will-actually-use-1)
  - [Commits](#commits)
- [Making A Pull Request](#making-a-pull-request)
  - [Why small PRs](#why-small-prs)
  - [The loop](#the-loop)
  - [Responding to review](#responding-to-review)
- [CI](#ci)
  - [What runs](#what-runs)
  - [Reading a failure](#reading-a-failure)
  - [Affected targets](#affected-targets)
- [Linting, Formatting, And Hygiene](#linting-formatting-and-hygiene)
  - [The standards behind the tooling](#the-standards-behind-the-tooling)
- [Writing A Test We Will Approve](#writing-a-test-we-will-approve)
  - [A worked example](#a-worked-example)
  - [The rules](#the-rules)
- [What Your Reviewer Will Say](#what-your-reviewer-will-say)
- [When You Are Stuck](#when-you-are-stuck)
- [Graduating To trel3](#graduating-to-trel3)
- [How This Repo Differs From trel3](#how-this-repo-differs-from-trel3)

## Where This Code Lives

TREL software is export-controlled. The rules below apply to trel3, and they
apply to this repo too - partly because habits formed in a sandbox are the
habits you keep, and partly because these are just good practice for any source
you are trusted with.

TREL's Technology Control Plan technically requires that development happen on
TREL machines over SSH rather than on personal laptops. Those machines are
unreliable and do not handle several developers at once. **This is not
encouragement to work around the TCP.** It is a statement that if you are going
to develop on a personal machine, these practices are what make that defensible
rather than reckless.

I am not a lawyer and this is not legal advice. The authoritative document is
[NIST SP 800-171r3](https://csrc.nist.gov/pubs/sp/800/171/r3/final).

### Keep it on your own disk

The checkout goes on physical storage attached to your machine. A concrete
recommendation:

```bash
mkdir -p ~/dev
cd ~/dev
# clone here
```

Not in a network share, not on a shared lab machine, not on a cloud VM, not in a
browser IDE, not in GitHub Codespaces, not on a USB stick that lives in your
backpack.

On macOS, avoid `~/Documents` and `~/Desktop` even if iCloud Drive looks off
today. The moment anyone enables "Desktop & Documents Folders" sync - including
a well-meaning future you setting up a new Mac from a backup - everything in
them uploads. `~/dev` is never synced by default.

### Encrypt that disk

Full disk encryption, on, verified, not "probably on."

| Platform | Mechanism | Verify with |
| --- | --- | --- |
| macOS | FileVault | `fdesetup status` |
| Linux | LUKS / dm-crypt | `findmnt --target .` then `lsblk` and look for `crypt` |
| WSL | BitLocker **on the Windows host** | `manage-bde -status` in an admin PowerShell |

WSL users: encrypting inside the Linux distro is not the job. The Windows drive
backing that distro is what needs BitLocker.

Also, because encryption at rest does nothing against someone walking up to an
unlocked laptop:

- A strong password on the account. Not a four-digit PIN.
- Automatic lock after a short idle period. Minutes, not hours.
- Do not leave it logged in and unattended in the lab or a coffee shop.

### Do not put it in the cloud

This is the one people get wrong by accident, because most of these are on by
default.

**No backup or sync of the repository to:** iCloud Drive, Dropbox, Google Drive,
OneDrive, Box, Time Machine to a cloud target, Backblaze, pCloud, or anything
similar. If you run any of these, exclude your source directory explicitly and
confirm the exclusion actually applied.

If your organization provides an **ITAR-compliant** cloud, that one is fine.
iCloud is not one. Neither is your personal Google account.

**No mirroring the code anywhere else.** Not to github.com, not to a personal
GitLab, not to a gist, not to a pastebin. Our GitHub Enterprise Server instance
is where this code lives and the only place it lives.

**Be careful what you paste into AI tools.** Pasting a snippet of this
onboarding repo's throwaway exercise code into a chatbot is not a problem.
Pasting trel3 source into one is a different thing entirely, because you have
just uploaded export-controlled source to a third party. Know which repo you are
in, and ask leadership what tooling is approved before you rely on it.

**Do not travel internationally with the machine** while this code is on it.

### The Bazel cache counts too

This is the part nobody thinks about. Bazel keeps an *output base* full of
compiled copies of repository sources, and it is large:

| Platform | Typical location |
| --- | --- |
| macOS | `/private/var/tmp/_bazel_$USER/` |
| Linux | `~/.cache/bazel/` |

Find yours with `bazel info output_base`.

It is as sensitive as the checkout, it is big enough that people relocate it to
"somewhere with more space," and that somewhere is occasionally a synced folder.
It must be local, on the encrypted volume, and excluded from cloud backup.

### Check yourself

```bash
./dev_scripts/check_storage.sh
```

That checks all of the above it can check mechanically: cloud-synced paths, disk
encryption, and the Bazel output base location. `setup.sh` runs it for you.

Passing it does not make you compliant. Failing it means you definitely are not.

## Setup

You need a Mac or a Linux machine. Windows works only through WSL or a Linux
partition; making this toolchain run natively on Windows is not worth anyone's
time.

1. **Do the trel3 hardware setup first** if you have not already - the
   `Hardware Setup` section of trel3's README. It installs command line tools, a
   Python version manager, and a git credential manager. Those all apply here.

2. **Install bazelisk, not bazel.** Bazelisk reads `.bazelversion` and runs the
   exact Bazel this repo pins, which is how a team stays on one version.

   ```bash
   brew install bazelisk          # macOS
   ```

   On Linux, grab a release binary from
   [bazelbuild/bazelisk](https://github.com/bazelbuild/bazelisk/releases) and
   put it on your `PATH` as `bazel`.

3. **Clone it** somewhere legal per the section above.

4. **Run setup:**

   ```bash
   ./dev_scripts/setup.sh
   ```

   The first run downloads a Rust toolchain and a full LLVM toolchain. That is
   several minutes and a few GB. It is the price of a build that behaves
   identically on your laptop, your teammate's laptop, and in CI. Later runs are
   seconds.

5. **VS Code**, with the Rust Analyzer, Bazel, and C/C++ extensions. Not
   required, but our tooling is set up around it.

## How To Navigate This Repo

```
.
├── README.md                  <- you are here
├── MODULE.bazel               <- every external dependency, declared once, repo-wide
├── .bazelrc                   <- build flags, including the rustfmt/clippy enforcement
├── .bazelversion              <- the one Bazel version everyone uses
├── .clang-format              <- C++ formatting rules
├── .clang-tidy                <- C++ lint rules
├── tools/                     <- clang-format and clang-tidy, pinned via Bazel
├── dev_scripts/               <- setup, storage checks, formatting, C++ linting
├── rust_exercises/            <- exercises 1-4, do these in order
├── cpp_exercises/             <- two short exercises on C++ mechanics
└── .github/
    ├── workflows/             <- CI
    ├── workflow_scripts/      <- the affected-target query CI uses
    ├── actions/setup-bazel/   <- shared CI setup
    └── pull_request_template.md
```

Read in this order:

1. This file, all of it.
2. `rust_exercises/README.md` - the conventions you are held to.
3. `rust_exercises/ex1_engineering_units/EXERCISE.md` - start working.

Every exercise directory has an `EXERCISE.md` with steps, a "Done when" list,
and the specific things that trip people up. The source files have the actual
teaching in them, in comments. Read the code.

## The Ladder

Do these in order. Each one is one pull request.

| # | Exercise | You learn |
| --- | --- | --- |
| 1 | `rust_exercises/ex1_engineering_units` | Bazel build/test, what a test looks like here, your first PR end to end |
| 2 | `rust_exercises/ex2_sensor_limits` | Actual Rust: enums, `match`, `Result`, borrowed slices |
| 3 | `rust_exercises/ex3_abort_debounce` | Write a failing test that exposes a real bug, then fix it. The most important one. |
| 4 | `rust_exercises/ex4_lint_cleanup` | rustfmt, clippy, and what the `manual` tag costs you |
| 5 | `cpp_exercises/ex1_pressure_units` | `cc_library`, `cc_test`, GoogleTest |
| 6 | `cpp_exercises/ex2_format_and_tidy` | A target that builds green and is still unmergeable |

Six PRs. If any of them takes you more than a couple of hours of actual work,
stop and ask someone - you have hit a tooling problem, not a skill problem, and
those are on us to fix and document.

**Not covered here:** Python and TypeScript, which trel3 uses heavily. The Bazel
and PR and testing habits transfer directly; the per-language details are in
trel3's `README.bazel.md` and `README.testing.md`.

## Bazel

Bazel is the most unfamiliar thing here for most people, and the most likely to
frustrate you early. Read trel3's `README.bazel.md` for the long version -
it explains why we accept the cost. The short version: hermetic toolchains, so
builds do not depend on what you happen to have installed; aggressive caching,
so rebuilds are fast; and an exact dependency graph, so CI can figure out
precisely what your change could have broken.

Notable users include Google, SpaceX, Tesla, Waymo, Stripe, and Nvidia. Getting
comfortable with it is a transferable skill, not TREL trivia.

### The mental model

A **target** is a thing Bazel can build, named by a **label**:

```
//rust_exercises/ex1_engineering_units:tests
│ └── package path (a directory with a BUILD.bazel)
└──── repository root
                                        └── target name, defined in that BUILD.bazel
```

- `//foo/bar:baz` - one specific target
- `//foo/bar:all` - every target in that one package
- `//foo/bar/...` - every target in that package and everything under it
- `//...` - everything in the repo

Targets and their dependencies are declared in `BUILD.bazel` files. Bazel only
sees what is declared - if you add a source file and do not add it to a `srcs`,
Bazel does not know it exists. That is the trade: you write down your
dependencies, and in exchange Bazel can answer questions about them.

### Commands you will actually use

| Command | What it does |
| --- | --- |
| `bazel build //path:target` | Build one target. For Rust this also runs rustfmt and clippy. |
| `bazel build //...` | Build everything (except targets tagged `manual`) |
| `bazel test //path:target` | Build and run a test target |
| `bazel test //...` | Run every test |
| `bazel test //path:target --test_output=all` | Show output from passing tests too, including `println!` |
| `bazel test //path:target --test_filter=test_name` | Run a subset of tests inside a target |
| `bazel test //path:target --nocache_test_results` | Re-run a test Bazel thinks it already answered |
| `bazel run //path:target` | Build and execute a binary target |
| `bazel query //...` | List every target |
| `bazel query 'kind(rust_test, //...)'` | List targets of one rule type |
| `bazel query 'deps(//path:target)'` | Everything that target depends on |
| `bazel query 'rdeps(//..., //path:target)'` | Everything that depends on that target - *what did I just break?* |
| `bazel query 'somepath(//a:b, //c:d)'` | Show a dependency chain between two targets, when you are wondering why A pulls in D |
| `bazel info output_base` | Where Bazel keeps its cache |

Four you should actually try right now, because they turn Bazel from an
obstacle into a tool:

```bash
# What does this crate depend on?
bazel query 'deps(//rust_exercises/ex3_abort_debounce:abort_debounce)'

# If I change this file, what could break?
bazel query 'rdeps(//..., rust_exercises/ex1_engineering_units/src/lib.rs)'

# What test targets exist at all?
bazel query 'kind(rust_test, //...)'

# Show me the test's output even though it passed
bazel test //rust_exercises/ex1_engineering_units:tests --test_output=all
```

`rdeps` is the one CI uses to decide what to run on your PR. Run it on your own
changes before you push and you will know what you are risking.

### When Bazel is being annoying

**"No such target" / "no such package".** Your label is wrong, or the directory
has no `BUILD.bazel`. Confirm with `bazel query //...`.

**Your test does not re-run.** Bazel cached the result because nothing it knows
about changed. `--nocache_test_results` forces it. If a *source* change is not
being noticed, the file is probably not in any `srcs`.

**A target you expected in `//...` is missing.** It is tagged `manual`. See
`rust_exercises/ex4_lint_cleanup/BUILD.bazel`.

**Something is deeply, inexplicably wrong.** `bazel clean` drops the build
outputs; `bazel clean --expunge` drops the whole output base and re-downloads
toolchains. Expunge is a last resort, not a first move - it costs you the full
setup download again. Ask someone before reaching for it.

**On macOS, Bazel occasionally wedges itself** in a known upstream bug that has
been open for years. trel3 ships a `bazel_health_verifier` script for exactly
this. If Bazel starts refusing to do anything sensible here, `bazel clean` and
then a small build is the local equivalent.

**AI help:** tell it you are using **bzlmod** (`MODULE.bazel`), not `WORKSPACE`.
Otherwise you get confident, obsolete instructions.

## Git

### Authenticating with our GitHub

Our GitHub Enterprise Server has SSH disabled, so git goes over HTTPS with a
Personal Access Token as your password. The full instructions are in trel3's
README under `Cloning The Repository`. Summary:

1. Generate a fine-grained token at
   `https://trel-github.ae.utexas.edu/settings/personal-access-tokens`, scoped
   to the Texas-Rocket-Engineering-Laboratory organization, with Metadata: read
   and Contents: read and write. No organization permissions.
2. Use your EID as the username and the token as the password.
3. Use a credential manager (`git-credential-manager` on macOS, `libsecret` on
   Linux) so you only enter it once.

Treat the token like a password: it goes in your credential manager, never in a
file in the repo, never in a commit, never pasted into chat.

### Commands you will actually use

| Command | What it does |
| --- | --- |
| `git status` | What is changed, staged, and which branch you are on. Run it constantly. |
| `git switch -c ex1-to-counts-tests` | Create and move to a new branch |
| `git switch main` | Move back to main |
| `git diff` | Unstaged changes |
| `git diff --staged` | What you are about to commit |
| `git add -p` | Stage changes interactively, hunk by hunk. The best habit on this list. |
| `git commit` | Commit staged changes; opens your editor for the message |
| `git push -u origin <branch>` | Push a new branch and set it to track |
| `git fetch origin` | Get the latest refs without changing your files |
| `git rebase origin/main` | Replay your commits on top of current main |
| `git log --oneline -10` | The last ten commits |
| `git log --oneline origin/main..HEAD` | Just the commits your PR adds. Check this before opening a PR. |
| `git push --force-with-lease` | Update a branch you rebased or amended, safely |
| `git restore <file>` | Throw away uncommitted changes to a file |
| `git reset --soft HEAD~1` | Undo the last commit, keep the changes staged |

Use `--force-with-lease`, never bare `--force`. It refuses to push if someone
else has pushed to that branch in the meantime, which is the exact accident that
force-push is infamous for.

**Never commit to `main` directly.** Always a branch, always a PR.

### Commits

A commit message is written for the person bisecting a regression six months
from now. That person is usually you.

```
Fix abort debounce not resetting on a false cycle

AbortOperator::run incremented the consecutive-cycle counter when the
condition held but never cleared it when the condition went false, so a
condition that flickered would still fire once the total count of true
cycles reached min_cycles. Reset the counter on a false cycle.
```

- First line: imperative mood, under ~70 characters, no trailing period.
  "Fix X", not "Fixed X" or "Fixes X".
- Blank line, then the body: what and **why**. The diff already shows how.
- One logical change per commit. `git add -p` makes this easy.
- `wip`, `fixes`, `asdf`, and `address comments` are not commit messages.

## Making A Pull Request

### Why small PRs

From trel3's README, and worth repeating: a PR should be a single self-contained
change, such that the codebase makes sense before it and makes sense after it.
Big enough to mean something, small enough that a reviewer can follow it in 15
to 30 minutes.

- Small changes are easy to revert when they break something.
- Small changes get reviewed. Large ones sit in the queue because nobody has a
  free hour.
- Small changes fail review in isolation. If a reviewer dislikes one piece of a
  600-line PR, all 600 lines wait.
- Small changes get validated against the whole codebase sooner, so you find out
  sooner if you need to redo something.

This is the single hardest professional habit to pick up, and it is not taught
in school, because student projects never have a reviewer. It is also the thing
your future coworkers will most appreciate you already knowing.

The `PR Hygiene` workflow warns above 300 changed lines and fails above 600.
Those numbers are a smell detector, not a law of nature - but if you are hitting
them in *this* repo, you have bundled several changes together.

### The loop

```bash
# 1. Start from current main
git switch main
git fetch origin
git rebase origin/main

# 2. Branch
git switch -c ex1-to-counts-tests

# 3. Work. Then check what you did, honestly.
git status
git diff

# 4. Run what you are about to ask someone to review
./dev_scripts/format.sh
bazel test //rust_exercises/ex1_engineering_units:tests

# 5. Stage deliberately, not with `git add .`
git add -p

# 6. Commit
git commit

# 7. Look at your own PR before anyone else does
git log --oneline origin/main..HEAD
git diff origin/main...HEAD

# 8. Push
git push -u origin ex1-to-counts-tests
```

Then open the PR in the web UI. Fill in the template - it asks what changed,
why, and how you verified it. "CI is green" is not verification; CI runs what
you told it to run.

**Open it as a draft if it is not ready.** Draft PRs do not demand review and do
not trigger the full CI suite. Mark it ready when you want eyes on it.

Step 7 is the one people skip and the one that pays. Reading your own diff
catches the debug `println!`, the commented-out block, the file you did not mean
to touch. Every reviewer can tell who does this.

### Responding to review

- Review comments are about the code. They are not about you. Nobody who has
  worked on a real codebase thinks otherwise.
- Answer every comment, even if the answer is "done."
- If you disagree, say so and say why. Reviewers are wrong sometimes, and a
  reviewer who has to guess your reasoning is a reviewer who blocks you.
- Push fixes as new commits while review is in progress, so the reviewer can see
  what changed since they last looked. Squash at merge if that is what the
  repo does.
- After a rebase or amend: `git push --force-with-lease`.

## CI

Every non-draft PR triggers workflows, and they have to pass to merge.

### What runs

| Workflow | Triggers on | Does |
| --- | --- | --- |
| `Rust Build & Lint, Test` | `*.rs`, `BUILD.bazel`, `MODULE.bazel`, `.bazelrc` | Works out the affected Rust targets, builds them (which runs rustfmt and clippy), then tests them |
| `C++ Build & Test` | `*.cc`, `*.h`, `BUILD.bazel` | Same, for `cc_*` targets |
| `C++ Format & Tidy` | `*.cc`, `*.h`, `.clang-format`, `.clang-tidy` | clang-format and clang-tidy on the files your PR changed |
| `PR Hygiene` | any PR | Checks the PR has a real description and is not too big to review |

There is no separate Rust lint job, and that is deliberate. `.bazelrc` attaches
the rustfmt and clippy aspects to every build, so a Rust build failure is as
likely to be a formatting problem as a compile error.

### Reading a failure

1. Open the failed job, expand the failed step, and **read the actual error.**
   It is usually the last 20 lines. Scroll up past the Bazel progress spam.
2. Reproduce it locally. Almost always the same command:
   `bazel test //the/target/it/named`. The build is hermetic - that is the whole
   point - so if CI fails and your machine passes, suspect something you did not
   commit.
3. Check `git status` for exactly that: the file you forgot to `git add`.
4. `bazel build //...` locally before pushing catches most of it. Note it skips
   `manual` targets.

The failing-test-logs artifact on the test job has the full Bazel test logs when
you need more than the console output.

### Affected targets

The Rust and C++ workflows do not build everything. They ask Bazel which targets
your changed files can possibly affect:

```
rdeps(set(<all targets of this kind>), set(<files your PR changed>))
```

Bazel knows the exact dependency graph, so that is an answer rather than a
guess. Change one file in `ex1` and CI builds and tests what depends on `ex1`,
and nothing else.

This is a genuinely big deal at trel3's scale: a change to `message_formats`
means the flight software and ground software targets that consume it get built
and tested too, automatically, without anyone maintaining a list. And a change
to one Python script does not trigger a 40-minute full-repo build. Read
`.github/workflow_scripts/compute_affected_targets.sh`; it is 40 lines and it is
the whole trick.

## Linting, Formatting, And Hygiene

The rule from trel3: **code with failing lint, tests, or build does not get
merged.** Using a lint ignore for a legitimate, stated reason is fine. Using one
to make a message go away is not.

How that gets enforced depends on the language, and the difference is worth
understanding rather than memorizing:

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` | `clang-format`, separate tool |
| Linter | `clippy`, run by `bazel build` | `clang-tidy`, separate tool |
| Can unformatted code build? | **No.** The build fails with a diff. | Yes. |
| Are lint findings errors? | **Yes.** Hard build failure. | Only where a script or CI runs them |
| Caught by | the build, so always | you, your reviewer, and one CI job |

In Rust you cannot forget. In C++ you can, which means C++ needs discipline that
Rust gets for free. `cpp_exercises/README.md` goes into this properly.

```bash
./dev_scripts/format.sh           # format all Rust and C++ in place
./dev_scripts/format.sh --check   # report, change nothing, fail if dirty
./dev_scripts/lint_cpp.sh         # clang-tidy over C++ sources
bazel build //...                 # this IS the Rust lint
```

Both C++ tools come from the LLVM toolchain Bazel downloaded, so your output is
byte-identical to CI's. Do not `brew install llvm` and use that instead;
clang-format 18 and clang-format 20 disagree about real code and the repo will
churn.

### The standards behind the tooling

Tools catch mechanical problems. These are the things a reviewer catches, from
trel3's Code Quality Standards:

- **YAGNI** - You Ain't Gonna Need It. Do not build for the requirement you
  imagine arriving next semester.
- **[SOLID](https://en.wikipedia.org/wiki/SOLID)** - single responsibility,
  open/closed, Liskov substitution, interface segregation, dependency inversion.
- **DRY** - Don't Repeat Yourself. Though two things that merely look alike are
  not duplication.
- **KISS** - Keep It Simple, Stupid.

And the expectation that changes come with tests reaching **90% coverage where
that is reasonable.**

## Writing A Test We Will Approve

Testing is where new members most often need to recalibrate, so this section is
long. The full version is trel3's `README.testing.md`.

Why we are strict about it:

- So we know your code works.
- So we know when someone else's change breaks your code.
- So we know when your change breaks someone else's code.
- Because tests are living documentation. When someone wonders what your
  function is supposed to do, they read your tests.

That last one matters more here than at a company. Every year the most
experienced people on this team graduate and take everything they did not write
down with them. Your tests are how your code survives you.

### A worked example

This is the real test from `rust_exercises/ex3_abort_debounce`. Read it before
you write your own.

```rust
#[rstest]                                 // Always rstest. Never #[test].
#[case(2)]                                // Three cases, one test body.
#[case(5)]                                // Each is reported separately, so a
#[case(12)]                               // failure tells you which input broke.
fn test_run_issues_no_outputs_after_discontinuous_min_cycles(
    #[case] min_cycles: u16,              // The case values arrive here...
    mut context: LoopContext,             // ...and this is a fixture: rstest
) {                                       // builds a fresh one per case.
    // ---- setup ----
    context
        .abort_configs_mut()
        .insert(ABORT_ID, config_with(Comparison::GreaterThan, min_cycles));
    let mut operator = AbortOperator::default();

    // ---- the code under test ----
    // Drive the condition true, then false, then true again. A loop that just
    // repeats the call is fine; a loop that decides what to assert is not.
    operator.run(&mut context).expect("run should succeed");
    if let Some(config) = context.abort_configs_mut().get_mut(&ABORT_ID) {
        *config = config_with(Comparison::LessThan, min_cycles);
    }
    operator.run(&mut context).expect("run should succeed");
    if let Some(config) = context.abort_configs_mut().get_mut(&ABORT_ID) {
        *config = config_with(Comparison::GreaterThan, min_cycles);
    }
    for _ in 1..min_cycles {
        operator.run(&mut context).expect("run should succeed");
    }

    // ---- assertions ----
    let command = context.device_write_queue().flush_to_composite_command();
    assert!(command.analog_outputs.is_empty());
    assert!(command.digital_outputs.is_empty());
}
```

Things to notice:

- **The name is a sentence.** `test_run_issues_no_outputs_after_discontinuous_min_cycles`.
  A reviewer reading a CI failure knows what broke without opening the file.
- **Three blocks, one blank line between them.** Setup, the code under test,
  assertions. This is required in review, not suggested.
- **The fixture** removes setup duplication across the whole test module without
  hiding what each test does differently.
- **`.expect("run should succeed")`** with a message, not `.unwrap()`. When it
  panics you get a sentence instead of a line number.
- **It tests the behavior the feature exists for**, not just the happy path. The
  three tests next to it all pass against buggy code. This one does not.

The production version of this test, against the real expression engine, is in
trel3 at `ground_software/hardware_control/src/aborts/abort_operator.rs` - search
for `discontinuous_min_cycles`.

### The rules

**Naming.** `test_<function>`, or `test_<function>_<behavior>`, or
`test_<function>_<behavior>_when_<condition>`. Pick the shortest one that is
still specific.

**Structure.** Setup, code under test, assertions, separated by single blank
lines. If you cannot tell which block a line belongs to, the test is doing too
much.

**What to test.** Walk each path through the function and test each one, plus
the cases that reach it. Boundaries especially: if a threshold is inclusive,
there is a test that would fail if someone made it exclusive.

**One behavior per test.** A test named `test_is_analog_input` that asserts two
different things gives you half an answer when it fails.

**No logic in tests.** No `if` deciding what to assert, no computing the
expected value with the same formula the code uses. A test with a branch has an
untested branch. Loops that merely repeat a call are fine.

**Rust specifics:** `#[rstest]` always. `#[case]` for specific input/expected
pairs, `#[values]` for combinations - but do not stack three `#[values]` and
call it thorough; 125 cases down one code path is a slow suite pretending to be
a good one. Nothing `pub` in a `tests` module. Tests live in the same file as
the code, in `#[cfg(test)] mod tests`.

**C++ specifics:** GoogleTest. `TEST(ThingUnderTest, WhatShouldHappen)`.
`EXPECT_*` unless continuing would crash, then `ASSERT_*`. `EXPECT_DOUBLE_EQ` or
`EXPECT_NEAR` for floats, never `EXPECT_EQ` - `0.1 + 0.2 == 0.3` is false.

**Verify your test can fail.** Break the code on purpose and confirm your test
catches it. A test that passes against broken code is decoration. This is what
exercise 3 is entirely about, and it is the habit that separates people who
write tests from people who write test-shaped code.

> "I'm so good I know my code will work without testing it" - you are not that
> guy, pal.

## What Your Reviewer Will Say

The comments you are most likely to get, so you can pre-empt them:

- "This PR does three things. Split it."
- "What happens when this is empty / zero / negative / missing?"
- "There is no test for the branch you just added."
- "Would this test fail if the code were wrong? Show me."
- "Put a blank line between setup and assertions."
- "This test name does not say what it tests."
- "Name this after what it is, not what type it is."
- "Why? Not what - the diff shows what. Put the why in the description or a
  comment."
- "This comment restates the code. Delete it or explain the reason instead."
- "`unwrap()` here will panic in production. What is the actual failure mode?"

And the one that matters most: **if the diff needs a verbal explanation to make
sense, the PR is not ready.** Write that explanation into the description, the
comments, or the tests.

## When You Are Stuck

**Ask.** Genuinely. Nobody on this team thinks less of you for asking, everyone
here had to learn all of this too, and answering questions is how veteran
members find out which docs are wrong.

Before you ask, spend a bounded amount of time - say 30 minutes - and then bring
what you have:

1. What you were trying to do.
2. The exact command you ran.
3. The actual error, pasted, not paraphrased.
4. What you already tried.

That makes you answerable in one message instead of five.

**If you learn something that should have been written down, write it down.**
Open a PR against this README or the exercise docs. That is a completely
legitimate contribution and it is the only reason this documentation is any
good.

Known rough edges live in trel3's `README.known_issues.md`. Check there before
concluding you broke something.

## Graduating To trel3

Once your six PRs are merged, read these in trel3, in this order:

| Document | Why |
| --- | --- |
| `README.md` | Getting started, tech stack, PR and code quality standards |
| `README.new_member_tips.md` | What the team expects, plus notable past PRs worth reading |
| `README.bazel.md` | Bazel per language, dependency management, known quirks |
| `README.testing.md` | The full testing guide, including Python and TypeScript |
| `README.known_issues.md` | The DevX problems you would otherwise rediscover |
| `dev_scripts/README.md` | The scripts and VS Code tasks that run automatically |

Then pick a small real task. A missing test on an existing function is an
excellent first real PR: no design decisions, real code, real reviewers, and it
leaves the codebase better.

## How This Repo Differs From trel3

Stated plainly so nothing surprises you later.

| | Here | trel3 |
| --- | --- | --- |
| Languages | Rust, a little C++ | Rust, Python, TypeScript; C++ in its own repo |
| Dependencies | a handful in `MODULE.bazel` | dozens, plus pnpm and pip lockfiles |
| Rust targets | host platforms only | plus `thumbv7em` for scopium boards and `wasm32` |
| Submodules | none | several, cloned with `--recurse-submodules` |
| CI runners | GitHub-hosted (`ubuntu-latest`) | self-hosted AWS runners, ephemeral per run |
| Bazel cache | local only | shared remote cache on TREL's network via `--config=ci` |
| Coverage gate | none; your reviewer is the gate | planned, in the 70-80% range |
| Test frameworks | rstest, GoogleTest | rstest, pytest, jest, React Testing Library |
| Consequences of breaking `main` | none | you have broken it for everyone |

If this repo is moved onto TREL's GitHub Enterprise Server, the `runs-on:` lines
in `.github/workflows/` need to change to the self-hosted labels trel3 uses -
GHES has no GitHub-hosted runners. There is a note at the top of each workflow
file saying exactly what to change.

The exercise code is throwaway. The habits are the deliverable.
