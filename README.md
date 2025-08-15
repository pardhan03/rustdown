# md2html-rs

A lightweight Rust CLI tool to convert Markdown files into HTML.  
It reads a `.md` file, parses it using a Markdown parser, wraps it in a full HTML document, and saves or prints the result.

---

## 🚀 Features
- Read Markdown files (like `README.md`)
- Parse and convert Markdown into **HTML**
- Wrap the result in a full HTML document (`<html>`, `<head>`, `<body>`)
- Print to console or save to a file

---

## 📦 Installation
Clone the repository and build the project:

```bash
git clone https://github.com/your-username/md2html-rs.git
cd md2html-rs
cargo build --release
