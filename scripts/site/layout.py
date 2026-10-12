"""Shared header and footer for the Sorolearn website pages."""
import hashlib
import os

ASSETS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "docs", "assets")


def asset_version(name):
    """Content hash for cache-busting: browsers refetch an asset whenever it changes."""
    with open(os.path.join(ASSETS, name), "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()[:10]

REPO = "https://github.com/Evangel90/Sorolearn"
CODESPACES = "https://codespaces.new/Evangel90/Sorolearn"


def page(title, description, active, root, body, head_extra="", scripts=""):
    nav = [("Curriculum", "curriculum/"), ("Setup", "setup/"), ("Usage", "usage/"), ("Graduates", "graduates/")]
    links = "".join(
        f'<a href="{root}{href}"{" aria-current=\"page\"" if label == active else ""}>{label}</a>'
        for label, href in nav
    )
    full_title = "Sorolearn" if title == "Sorolearn" else f"{title} · Sorolearn"
    return f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{full_title}</title>
<meta name="description" content="{description}">
<meta name="theme-color" content="#08090a">
<link rel="icon" href="{root}assets/favicon.svg" type="image/svg+xml">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
<link rel="stylesheet" href="{root}assets/site.css?v={asset_version("site.css")}">
{head_extra}</head>
<body>
<header class="site-header">
  <div class="inner">
    <a class="brand" href="{root}"><img class="mark" src="{root}assets/favicon.svg" alt="" width="22" height="22">Sorolearn</a>
    <nav class="site-nav" aria-label="Main">{links}</nav>
    <div class="header-cta">
      <a class="link" href="{REPO}">GitHub</a>
      <a class="btn small primary" href="{root}setup/">Get started</a>
    </div>
  </div>
</header>
<main>
{body}
</main>
<footer class="site-footer">
  <div class="inner">
    <div>
      <a class="brand" href="{root}"><img class="mark" src="{root}assets/favicon.svg" alt="" width="22" height="22">Sorolearn</a>
      <p>Small exercises for Stellar and Soroban.<br><a href="{REPO}/blob/main/LICENSE" style="display:inline;margin:0">MIT licensed</a>. Inspired by <a href="https://rustlings.rust-lang.org" style="display:inline;margin:0">Rustlings</a>.</p>
    </div>
    <div>
      <h4>Learn</h4>
      <a href="{root}curriculum/">Curriculum</a>
      <a href="{root}setup/">Setup</a>
      <a href="{root}usage/">Usage</a>
    </div>
    <div>
      <h4>On-chain</h4>
      <a href="{root}graduates/">Graduates</a>
      <a href="https://stellar.expert/explorer/testnet/contract/CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY">Registry contract</a>
    </div>
    <div>
      <h4>Project</h4>
      <a href="{REPO}">GitHub</a>
      <a href="{REPO}/blob/main/CONTRIBUTING.md">Contributing</a>
      <a href="{CODESPACES}">Open in Codespaces</a>
      <a href="https://developers.stellar.org/docs/build/smart-contracts/overview">Soroban docs</a>
    </div>
  </div>
</footer>
<script src="{root}assets/copy.js?v={asset_version("copy.js")}" defer></script>
{scripts}</body>
</html>
"""
