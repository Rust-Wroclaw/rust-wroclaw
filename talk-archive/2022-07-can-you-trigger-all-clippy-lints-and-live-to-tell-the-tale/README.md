There are 348 lints in total.

Meaning that in 40-ish minutes time I have I would have to allot for each lint roughly 6 seconds.
That's hardly a presentation and more of a fire-and-forget slide reading session.

So instead I'm going to focus on a subset of available lints - specifically on so called "correctness" lints, of which there are 72.
Which gives me a comfortable 30 seconds per lint.

If time permits I also prepared a few slides regarding the "suspicious" group of lints.

But before we get to that - a small foreword about clippy itself and how the lints are organized.

# What is Clippy & stuff

Clippy is the standard linter for the Rust programming language.
For those who don't know a linter is a tool to catch a certain kind of errors, issues and code smells in one's code.

Rust's compiler can already catch and notify you about quite a range of issues, such as an unused variable, an unused function/method, etc.

But there are many more issues that it simply cannot or should not catch.

That's instead the job of the linter (or the formatter, but that's another matter).

The linter will analyze your code and try to find certain predefined patterns.

In Clippy the lints are organized in groups, which are listed as follows:
1. Correctness - the lints which catch code that is quote "outright wrong or useless"
2. Suspicious - "code that is most likely wrong or useless"
3. Style - "code that should be written in a more idiomatic way"
4. Complexity - "code that does something simple but in a complex way"
5. Perf - "code that can be written to run faster"
6. Pedantic - "lints which are rather strict or have occasional false positives"

and finally there are
7. Nursery - lints which are under development
8. Cargo - lints for the cargo manifest

Each group has an assigned "level" - allow, warn or deny. Which will result in respectively - passing, giving a lint-time warning or a lint-time error.

Only the "Correctness" lints are "deny" by default.

All other groups apart from "Pedantic", "Nursery" or "Cargo" are "warn" by default.

The last set of groups are set to "Allow".

