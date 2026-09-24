<!--
Keep this short. A reviewer should be able to read the description and then read
the diff without having to ask you anything.

Delete any section that genuinely does not apply.
-->

## What this changes

<!-- One or two sentences on what is different after this PR. -->

## Why

<!-- The motivation. "Rust exercise 2" is a perfectly good answer in this repo.
     On a real repo, link the ticket or describe the problem you hit. -->

## How I verified it

<!-- The commands you ran and what happened. For example:

     bazel test //rust_exercises/ex2_abort_debounce:tests
     -> 10 tests pass (was 7 pass / 3 fail before the fix in commit 2)

     "CI is green" on its own is not verification - CI only runs what you told
     it to run. -->

## Notes for the reviewer

<!-- Anything that would otherwise come up as a review comment: a decision you
     were unsure about, a tradeoff you made, something you left out on purpose
     and plan to do in a follow-up. Flagging these yourself is a sign of a
     strong engineer, not a weak one. -->

## Checklist

- [ ] `bazel test //...` passes, or I can explain which targets fail and why
- [ ] New or changed behavior has tests, and I checked they fail without my change
- [ ] Tests follow the setup / code under test / assertions layout
- [ ] Rust tests use `#[rstest]` and are named `test_<function>_<behavior>[_when_<condition>]`
- [ ] `./dev_scripts/format.sh --check` passes for the code I touched
- [ ] This is one self-contained change, not several bundled together
