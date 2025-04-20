# 🧠 Faro Shuffle (soon to be renamed) – Your AI-Powered Project Scoping Tool

> **Land better clients. Protect your time. Scope smarter—before the proposal is even written.**

`faro-shuffle` is a local-first CLI that uses AI to analyze your task descriptions or client intake notes, **considering the context of your project directory**, then returns a **complexity score and plain-language rationale**. It's built for developers, freelancers, and agencies who are tired of guessing how much effort a project will *really* take.

---

## 🚀 Why Use Faro?

You've been here before:

- A client sends you a vague brief.
- You nod, guess, and price it.
- Then it explodes into something 10x more complex.

`faro-shuffle` is your **early warning system**. It reads your intake notes and tells you how difficult the task is—*before you commit*.  

Whether you're building a proposal, deciding your rate, or just protecting your calendar, this tool gives you clarity fast.

---

## ⚙️ How It Works

```sh
# Analyze a task, providing the project directory for context
faro-shuffle analyze ./path/to/task.md --project-dir ./path/to/project

# Get output in JSON format
faro-shuffle analyze ./task.md --project-dir . --format json
```

### Example Output (JSON):
```json
{
  "score": 8,
  "rationale": "Considering the project context (Rust, ~100 .rs files), adding this feature requires significant changes to the core engine and API layers, making it complex."
}
```

> ✅ Instant insight (now context-aware!)
> ✅ Honest complexity score
> ✅ Flexible output (Text, JSON, Markdown)
> ✅ No cloud. No signup. No BS.

---

## 📦 Installation

> **Requirements:**  
> - Rust (v1.74+ recommended)  
> - Ollama running locally (with a supported model like `mistral` or `llama3`)  

**Install dependencies:**
```sh
cargo build --release
```

**Run CLI:**
```sh
# Basic analysis
./target/release/faro_shuffle analyze ./your-task.md

# Analysis with project context and JSON output
./target/release/faro_shuffle analyze ./your-task.md --project-dir /path/to/your/project --format json
```

> 📝 Your input file should be a simple Markdown file with a task or project description.

---

## 💼 Use Cases

### 🔍 For Freelancers & Agencies
- Analyze client briefs **before** you quote
- Spot vague or high-risk requests early
- Set better expectations with clients
- Use it alongside your client intake forms

### 🧑‍💻 For Developers
- Evaluate your own backlog tasks
- Prioritize based on effort vs impact
- Use AI to avoid time sinks and rabbit holes

---

## 🔒 Local-First, Privacy-Respecting

No APIs. No uploads. Your project data never leaves your machine.  
Runs on top of **Ollama**, using local quantized LLMs like Mistral or LLaMA 3.

---

## 📈 Roadmap

- [x] JSON + Markdown output options
- [ ] Subtask decomposition engine (Pro tier)
- [ ] GitHub issue integration
- [ ] `client-intake-system` integration
- [ ] SaaS-ready web UI (optional)
- [ ] Developer analytics (complexity trends, historical effort estimates)

---

## 🧪 Integration Example (Coming Soon)

Seamlessly plug into your `client-intake-system`:

```bash
# After a client submits a form:
faro-shuffle analyze ./intake/generated/brief-XYZ.md >> ./scoping/logs/report-XYZ.txt
```

Use this to:
- Auto-score project complexity
- Flag high-risk proposals
- Prep smarter follow-up questions

---

## 👥 Who It's For

- 🧑‍💻 Freelancers tired of scope creep
- 🏢 Agencies who quote custom projects weekly
- 🧠 Devs who want AI to sanity-check tasks
- 🧩 PMs who want to clarify team workload before assigning tickets

---

## 💬 Join the Waitlist

> **Coming Soon:** Faro Pro  
> - Auto-decompose tasks  
> - Subtask scoring  
> - Project-wide scoping dashboards  
> - Markdown export for clients  
> 👉 [Join the waitlist](#)

---

## 📣 Shoutouts & Credits

Built with ❤️ and frustration by [@0xjcf](https://github.com/0xjcf).  
Using [Rust](https://www.rust-lang.org/), [Ollama](https://ollama.com), and [Mistral](https://mistral.ai).

---

## 🧠 Bonus: What's in a Name?

This project may be rebranded soon to better reflect its focus on scoping, complexity scoring, and AI-powered clarity. Suggestions welcome. 😉 