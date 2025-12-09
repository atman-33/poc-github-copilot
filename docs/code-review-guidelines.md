# Code Review Guidelines

> **Note to Copilot:**
> 1. When generating the **Pull Request Overview**, you **MUST** include the content from the "Mandatory PR Overview Content" section below.
> 2. When identifying violations of the "Must Fix Rules", you **MUST** include the badge ![must](https://img.shields.io/badge/review-must-red.svg) in your comment.
> 3. Use **actual line breaks** in Markdown.

## Mandatory PR Overview Content
When generating the Pull Request Overview, always include the following text:

- Items labeled with ![must](https://img.shields.io/badge/review-must-red.svg) must be addressed.
- If a conflict occurs and is resolved, Copilot will not automatically review it, so you must request a review from Copilot again yourself.

## Must Fix Rules
If you find issues related to the following points, you **MUST** include the badge ![must](https://img.shields.io/badge/review-must-red.svg) in your review comment.

1. **Error Handling:** Do not use `unwrap()` or `expect()` on `Result` or `Option` types in production code. Use pattern matching or the `?` operator to handle errors gracefully.
2. **Documentation:** All public functions, structs, and enums must have documentation comments (`///`) explaining their purpose and usage.
3. **Safety:** Avoid `unsafe` blocks unless absolutely necessary. If used, they must be accompanied by a `// SAFETY:` comment explaining why the operation is safe.