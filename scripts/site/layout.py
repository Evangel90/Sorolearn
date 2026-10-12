"""Shared header and footer for the Sorolearn website pages."""
REPO = "https://github.com/Evangel90/Sorolearn"
CODESPACES = "https://codespaces.new/Evangel90/Sorolearn"

def page(title, description, active, root, body, head_extra="", scripts=""):
    nav = [("Setup", "setup/"), ("Usage", "usage/"), ("Curriculum", "curriculum/"), ("Graduates", "graduates/")]
    links = "".join(
        f'<a href="{root}{href}"{" aria-current=\"page\"" if label == active else ""}>{label}</a>'
        for label, href in nav
    ) + f'<a href="{REPO}">GitHub</a>'
    full_title = "Sorolearn" if title == "Sorolearn" else f"{title} · Sorolearn"
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{full_title}</title>
<meta name="description" content="{description}">
<link rel="icon" href="{root}assets/favicon.svg" type="image/svg+xml">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;600&display=swap" rel="stylesheet">
<link rel="stylesheet" href="{root}assets/site.css">
{head_extra}</head>
<body>
<header class="site-header">
  <div class="inner">
    <a class="brand" href="{root}"><img class="mark" src="{root}assets/favicon.svg" alt="" width="28" height="28">Sorolearn</a>
    <nav class="site-nav" aria-label="Main">{links}</nav>
  </div>
</header>
<main>
{body}
</main>
<footer class="site-footer">
  <div class="inner">
    <span>Sorolearn: small exercises for Stellar &amp; Soroban. MIT licensed.</span>
    <span><a href="{REPO}">Source on GitHub</a> · Inspired by <a href="https://rustlings.rust-lang.org">Rustlings</a></span>
  </div>
</footer>
{scripts}</body>
</html>
"""
