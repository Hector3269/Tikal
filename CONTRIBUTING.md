# 🤝 Contributing Guide

> **Welcome, contributor!** 🎉  
> We're excited to have you here. This guide will help you navigate our contribution process and keep our codebase clean, consistent, and maintainable.


## 📝 Commit Rules

### 📌 The Anatomy of a Perfect Commit

```
<type>(<scope>): <short message>
```

**Breaking it down:**

- **`type`** → What kind of change is this? (see types below)  
- **`scope`** → Which part of the codebase? (`model`, `macros`, `core`, `validate`, `migrate`, `docs`)  
- **`short message`** → Clear, concise, present tense. Max 72 chars. No period at the end.

### ✨ Examples That Shine

```text
✅ feat(model): implement ColumnAttr and RelationAttr parsing
✅ fix(validate): prevent multiple primary keys
✅ docs(readme): add usage example for Entity macro
✅ perf(core): optimize query builder allocation

❌ added new feature
❌ Fixed bug.
❌ update
```

### 🎨 Commit Types Reference

| Type        |  Purpose                                              | Example                                    |
|-------------|-------------------------------------------------------|--------------------------------------------|
| `feat`      |  New functionality, features, or APIs                 | `feat(model): add support for JSON fields` |
| `fix`       |  Bug fixes and error corrections                      | `fix(migrate): handle null constraints`    |
| `refactor`  |  Code restructuring without behavior changes          | `refactor(core): simplify query builder`   |
| `docs`      |  Documentation updates                                | `docs(api): document new relation macros`  |
| `test`      |  Adding or modifying tests                            | `test(validate): add primary key tests`    |
| `chore`     |  Maintenance, dependencies, tooling                   | `chore(deps): update syn to 2.0`           |
| `perf`      |  Performance improvements                             | `perf(query): cache compiled statements`   |
| `style`     | Code style, formatting (no logic changes)             | `style(core): format with rustfmt`         |
| `ci`        | CI/CD configuration changes                           | `ci(github): add clippy workflow`          |

---

## 🐛 Reporting Issues

Found a bug? We want to know! Help us help you by including:

### 📋 Issue Template

```markdown
**Description:**
A clear description of what's wrong

**Steps to Reproduce:**
1. Do this
2. Then do that
3. See error

**Expected Behavior:**
What should happen

**Actual Behavior:**
What actually happens

**Environment:**
- Rust version: 1.75.0
- OS: Ubuntu 22.04
- Project version: 0.1.0
```

---

## 💡 Feature Requests

Have an idea? We'd love to hear it!

Open a discussion or issue with:
- Clear description of the feature
- Use cases and examples
- Potential implementation ideas (optional)

---

## 🌟 Recognition

Every contribution matters! Contributors will be:
- Listed in our `CONTRIBUTORS.md` file
- Mentioned in release notes for significant contributions
- Forever appreciated by the community ❤️

---

## 💬 Need Help?

- 💭 **Questions?** Open a discussion
- 🐛 **Found a bug?** Open an issue
- 💡 **Have an idea?** Start a conversation

---

## 📜 Code of Conduct

Be respectful, be kind, be constructive. We're all here to build something great together.

---

<div align="center">

**Thank you for contributing!** 🚀

*Together, we build better software.*

</div>