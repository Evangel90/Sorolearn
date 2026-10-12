"""Generate the Sorolearn website pages into docs/.

Run from anywhere: `python3 scripts/site/build.py`. The graduates page
(docs/graduates/index.html) is hand-written and not generated here.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(__file__))
from layout import CODESPACES, REPO, page

DOCS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "docs")
TREE = f"{REPO}/tree/main/exercises"


def write(rel, html):
    path = os.path.join(DOCS, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(html)
    print("wrote", rel)


# ------------------------------------------------------------------ home
PLAYER_HEAD = '<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/asciinema-player@3.17.0/dist/bundle/asciinema-player.css">\n'
PLAYER_JS = """<script src="https://cdn.jsdelivr.net/npm/asciinema-player@3.17.0/dist/bundle/asciinema-player.min.js"></script>
<script>
  AsciinemaPlayer.create("assets/demo.cast", document.getElementById("demo"), {
    cols: 100, rows: 30, idleTimeLimit: 2, autoPlay: true, loop: true, poster: "npt:0:16.9", fit: "width", theme: "asciinema", preload: true, terminalFontFamily: "JetBrains Mono, Menlo, monospace"
  });
</script>
"""

home = f"""
<section class="hero">
  <a class="badge" href="curriculum/"><span class="dot"></span>soroban-sdk 29 <span class="sep">·</span> Protocol 29 <span class="sep">·</span> 35 exercises <span class="arrow">→</span></a>
  <h1 class="display gradient-text">Learn Soroban by fixing it.</h1>
  <p class="sub">Small, broken smart contracts with tests. Fix them, save, and watch them turn green.
  Then learn the Stellar network underneath, review AI-written code, and ship to testnet.</p>
  <div class="buttons">
    <a class="btn primary" href="setup/">Get started <span class="arrow">→</span></a>
    <a class="btn" href="{CODESPACES}">Open in Codespaces</a>
  </div>
  <p class="fineprint">Free and open source · Works offline after a one-time setup</p>

  <div class="window">
    <div class="window-bar"><i></i><i></i><i></i><span>sorolearn — watch mode</span></div>
    <div class="player" id="demo" aria-label="Terminal recording: Sorolearn watch mode catches an exploit, the fix is saved, the exercise passes"></div>
  </div>
</section>

<section class="stats-strip" aria-label="Sorolearn in numbers">
  <div><strong>35</strong><span>exercises</span></div>
  <div><strong>16</strong><span>topics</span></div>
  <div><strong>5</strong><span>security audits</span></div>
  <div><strong>4</strong><span>testnet labs</span></div>
  <div><strong>1</strong><span>on-chain exam</span></div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">The course</p>
    <h2>Four movements, one broken program at a time.</h2>
    <p class="lede">Every exercise compiles against the real SDK and is checked by real tests. You always know
    exactly what's wrong, and exactly when it's fixed.</p>
  </div>
  <div class="movements">
    <div class="movement">
      <span class="num">1.0</span>
      <h3>Write contracts</h3>
      <p>Functions, types, storage and TTL, errors, auth, events, tokens, cross-contract calls and time.</p>
      <ul><li>01 → 10 · quizzes</li><li>23 exercises</li></ul>
    </div>
    <div class="movement">
      <span class="num">2.0</span>
      <h3>Understand Stellar</h3>
      <p>The network underneath: accounts and keys, multisig, transactions and signatures, classic assets.</p>
      <ul><li>11 → 13</li><li>checked against the Stellar CLI</li></ul>
    </div>
    <div class="movement">
      <span class="num">3.0</span>
      <h3>Review the AI</h3>
      <p>Plausible, AI-style contracts with one planted vulnerability each. An exploit test proves the attack.</p>
      <ul><li>14 · security</li><li>5 audits</li></ul>
    </div>
    <div class="movement">
      <span class="num">4.0</span>
      <h3>Ship to testnet</h3>
      <p>Deploy what you wrote, call it from real accounts, watch its events, then pass the on-chain exam.</p>
      <ul><li>15 · deploy</li><li>4 labs + graduation</li></ul>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">Why Sorolearn</p>
    <h2>AI can write your contract.<br><span class="muted">Can you tell when it's wrong?</span></h2>
  </div>
  <div class="bento">
    <div class="tile wide">
      <span class="label">14_security/audit_auth.rs</span>
      <h3>Review the AI</h3>
      <p>It compiles. It calls <code>require_auth</code>. The happy path passes. And anyone can drain anyone,
      because it asks the <em>recipient</em> to sign. Five audits like this, each with an exploit test.</p>
      <div class="diff"><div class="dim">pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {{</div><div class="del">-    to.require_auth();</div><div class="add">+    from.require_auth();</div><div class="dim">    let from_balance = Self::balance(env.clone(), from.clone());</div></div>
    </div>
    <div class="tile narrow">
      <span class="label">watch mode</span>
      <h3>One key away</h3>
      <p>Save to re-run the tests. Keys work instantly.</p>
      <div class="keys"><span class="key">n<small>next</small></span><span class="key">h<small>hint</small></span><span class="key">l<small>list</small></span><span class="key">q<small>quit</small></span></div>
    </div>
    <div class="tile narrow">
      <span class="label">offline first</span>
      <h3>No network needed</h3>
      <p>One setup, then every build runs offline against a pinned lockfile. Or skip installing: open it in Codespaces.</p>
      <div class="mini-term"><span class="p">$</span> sorolearn setup
<span class="ok">done:</span> works fully offline</div>
    </div>
    <div class="tile wide">
      <span class="label">registry · CD23Z2…FWYY</span>
      <h3>Graduate on-chain</h3>
      <p>A registry contract on Stellar testnet deploys a fresh copy of your contract, examines it with
      cross-contract calls, and records you. Nobody self-reports. <a href="graduates/">See the graduates →</a></p>
      <div class="mini-term"><span class="p">$</span> sorolearn graduate
[6/6] Ask the registry to examine your contract and record you
<span class="ok">✓ You are Sorolearn graduate #3!</span></div>
    </div>
  </div>
</section>

<section class="cta">
  <h2 class="gradient-text" style="font-size:clamp(36px,6vw,56px)">Start fixing.</h2>
  <p class="lede" style="margin:0 auto 32px">Clone it and run <code>sorolearn</code>, or open it in your browser.</p>
  <div class="buttons">
    <a class="btn primary" href="setup/">Get started <span class="arrow">→</span></a>
    <a class="btn" href="{REPO}">View on GitHub</a>
  </div>
</section>
"""
write("index.html", page("Sorolearn", "Small exercises to get you used to building on Stellar and writing Soroban smart contracts.",
                         None, "", home, PLAYER_HEAD, PLAYER_JS))

# ----------------------------------------------------------------- setup
setup = f"""
<div class="page-head">
<p class="eyebrow">Setup</p>
<h1 class="gradient-text">Run it anywhere.</h1>
<p class="lede">Two ways to run Sorolearn: in your browser with nothing to install, or locally, where it
works fully offline after a one-time setup.</p>
</div>
<div class="prose">

<h2>In the browser (GitHub Codespaces)</h2>
<p>Click the button, wait for the codespace to open, then type <code>sorolearn</code> in the terminal.
Rust, the <code>sorolearn</code> command, the Stellar CLI and a pre-compiled Soroban SDK are already installed.</p>
<div class="buttons"><a class="btn primary" href="{CODESPACES}">Open in Codespaces</a></div>
<p class="muted">A free GitHub account includes a monthly Codespaces allowance; Sorolearn's repo is public, so it
runs on yours.</p>

<h2>Locally</h2>
<h3>1. Install Rust</h3>
<p>You need Rust 1.91 or newer. Install it with <a href="https://rustup.rs">rustup</a>, or update an existing
install:</p>
<pre><code>rustup update stable</code></pre>

<h3>2. Get Sorolearn</h3>
<pre><code>git clone {REPO}.git sorolearn
cd sorolearn
cargo install --path .</code></pre>
<p>This installs the <code>sorolearn</code> command. Run it from inside the <code>sorolearn</code> folder.
(<code>cargo run --release</code> works too, without installing.)</p>

<h3>3. One-time setup</h3>
<pre><code>sorolearn setup</code></pre>
<p>Downloads and pre-compiles the Soroban SDK (a minute or two). It runs automatically the first time if you
skip it. Afterwards every build uses <code>cargo --offline</code> against a pinned lockfile, so Sorolearn works on a
plane or behind a firewall.</p>

<h3>4. Start</h3>
<pre><code>sorolearn</code></pre>
<p>Then open the exercise it shows in your editor. See <a href="../usage/">Usage</a>.</p>

<h2>Editor</h2>
<p>Any editor works. In VS Code, install <strong>rust-analyzer</strong>: the repo's settings link the generated
exercise workspace, so you get completions and inline errors inside exercise files. The repo also ships a
<em>Dev Container</em>, so VS Code with Docker gives you the same environment as Codespaces.</p>

<h2>Optional: the Stellar CLI</h2>
<p>Only needed for the final topic: the testnet lab and <code>sorolearn graduate</code>. It's pre-installed in Codespaces.</p>
<pre><code>brew install stellar-cli              <span class="comment"># macOS</span>
cargo install --locked stellar-cli    <span class="comment"># anywhere else</span>
rustup target add wasm32v1-none</code></pre>

<h2>Troubleshooting</h2>
<ul>
  <li><strong>"soroban-sdk needs Rust 1.91 or newer"</strong>: run <code>rustup update stable</code>.</li>
  <li><strong>"Could not find info.toml"</strong>: run <code>sorolearn</code> from the repository folder.</li>
  <li><strong>Build cache deleted?</strong> Sorolearn notices and re-runs setup (this one needs internet).</li>
</ul>
</div>
"""
write("setup/index.html", page("Setup", "Install Sorolearn locally or open it in GitHub Codespaces.", "Setup", "../", setup))

# ----------------------------------------------------------------- usage
usage = f"""
<div class="page-head">
<p class="eyebrow">Usage</p>
<h1 class="gradient-text">Read, fix, save, repeat.</h1>
<p class="lede">Sorolearn takes care of compiling and testing. You fix the code.</p>
</div>
<div class="prose">

<h2>Watch mode</h2>
<pre><code>sorolearn</code></pre>
<p>Sorolearn shows the current exercise, for example <code>exercises/04_storage/storage1.rs</code>. Open it in
your editor, read the comments, and make the code compile and the tests pass. Every time you save, Sorolearn
re-runs the tests. When the exercise passes, press <code>n</code> for the next one. Keys work instantly; no Enter needed.</p>
<div class="table-wrap"><table>
<thead><tr><th>Key</th><th>Action</th></tr></thead>
<tbody>
<tr><td><code>n</code></td><td>next exercise (once the current one passes)</td></tr>
<tr><td><code>h</code></td><td>show a hint</td></tr>
<tr><td><code>l</code></td><td>list all exercises and your progress</td></tr>
<tr><td><code>r</code></td><td>re-run the current exercise</td></tr>
<tr><td><code>q</code></td><td>quit (progress is saved)</td></tr>
</tbody></table></div>

<h2>Rules of the game</h2>
<ul>
  <li><strong>Don't edit the tests</strong> at the bottom of each file. They are the spec.</li>
  <li>Each topic folder has a <code>README.md</code> with the background you need.</li>
  <li>Reference solutions live in <code>solutions/</code>. Compare once you've passed, not before.</li>
</ul>

<h2>Kinds of exercises</h2>
<h3>Soroban contracts</h3>
<p>Small <code>#![no_std]</code> contracts tested against Soroban's simulated environment: no network needed.</p>
<h3>Stellar lessons</h3>
<p>Plain Rust against the real Stellar types: derive keys and addresses, apply reserve and multisig rules,
build, hash and sign transactions, parse assets. Expected values (like a transaction hash) were checked
against the Stellar CLI.</p>
<h3>Security: review the AI</h3>
<p>Each contract looks fine and its <code>happy_path</code> tests pass. An <code>exploit_</code> test reproduces
a real attack. Fix the contract so the attack fails.</p>

<h2>All commands</h2>
<div class="table-wrap"><table>
<thead><tr><th>Command</th><th>What it does</th></tr></thead>
<tbody>
<tr><td><code>sorolearn</code></td><td>watch mode</td></tr>
<tr><td><code>sorolearn list</code></td><td>all exercises and your progress</td></tr>
<tr><td><code>sorolearn run [name]</code></td><td>compile and test one exercise</td></tr>
<tr><td><code>sorolearn hint [name]</code></td><td>show a hint</td></tr>
<tr><td><code>sorolearn reset &lt;name&gt;</code></td><td>restore an exercise to its original state</td></tr>
<tr><td><code>sorolearn verify</code></td><td>check every exercise, in order</td></tr>
<tr><td><code>sorolearn setup</code></td><td>one-time download and pre-build (enables offline use)</td></tr>
<tr><td><code>sorolearn build-wasm &lt;name&gt;</code></td><td>compile an exercise to a deployable <code>.wasm</code></td></tr>
<tr><td><code>sorolearn graduate [identity]</code></td><td>deploy your quiz1 poll to testnet and take the on-chain exam</td></tr>
</tbody></table></div>

<h2>Graduating</h2>
<p>When you finish, run <code>sorolearn graduate</code>. It creates a funded testnet account, builds your
<code>quiz1</code> poll to WebAssembly, uploads it, and calls the Sorolearn registry contract. The registry deploys
a fresh copy, examines it with cross-contract calls, and records you as a graduate. Every Stellar CLI command
is printed as it runs. Your record appears on the <a href="../graduates/">graduates page</a>.</p>

<h2>Contributing exercises</h2>
<ol>
  <li>Write the solution in <code>solutions/&lt;topic&gt;/&lt;name&gt;.rs</code>.</li>
  <li>Copy it to <code>exercises/&lt;topic&gt;/&lt;name&gt;.rs</code> and break it in an instructive way.</li>
  <li>Add an entry with a hint to <code>info.toml</code>.</li>
  <li>Run <code>sorolearn dev check</code>: every solution must pass and every exercise must fail.</li>
</ol>
</div>
"""
write("usage/index.html", page("Usage", "How to use Sorolearn: watch mode, hints, commands and graduation.", "Usage", "../", usage))

# ------------------------------------------------------------ curriculum
topics = [
    ("Getting started", [
        ("00", "Intro", "00_intro", "intro1-2", "Your first contract: <code>#[contract]</code>, <code>#[contractimpl]</code>, <code>Env</code> and test clients."),
    ]),
    ("Soroban smart contracts", [
        ("01", "Functions", "01_functions", "functions1-2", "Exported functions and the types allowed across the contract boundary."),
        ("02", "Types", "02_types", "symbols1, vecs1, maps1, bytes1", "<code>Symbol</code>, <code>Vec</code>, <code>Map</code>, <code>Bytes</code>, <code>String</code>."),
        ("03", "Custom types", "03_custom_types", "types1-2", "<code>#[contracttype]</code> structs and enums."),
        ("04", "Storage", "04_storage", "storage1-3", "Instance, persistent and temporary storage, <code>DataKey</code>, TTL."),
        ("05", "Errors", "05_errors", "errors1-2", "<code>#[contracterror]</code>, <code>Result</code>, <code>panic_with_error!</code>, checked math."),
        ("06", "Auth", "06_auth", "auth1-2", "<code>require_auth</code>, the admin pattern, constructors."),
        ("07", "Events", "07_events", "events1", "<code>#[contractevent]</code>, topics and data."),
        ("08", "Tokens", "08_tokens", "tokens1", "<code>TokenClient</code>, the Stellar Asset Contract, contracts holding funds."),
        ("09", "Cross-contract", "09_cross_contract", "cross1", "Calling other contracts."),
        ("10", "Time", "10_time", "time1", "Ledger timestamp vs sequence."),
        ("★", "Quizzes", "quizzes", "quiz1-2", "A poll and a time-locked token vault: everything together."),
    ]),
    ("The Stellar network underneath", [
        ("11", "Accounts", "11_stellar_accounts", "accounts1-3", "Lumens and stroops, reserves, ed25519 keys, StrKeys (<code>G…</code> <code>S…</code> <code>C…</code> <code>M…</code>), multisig thresholds."),
        ("12", "Transactions", "12_stellar_transactions", "transactions1-2", "Operations, sequence numbers, fees, memos, XDR, network-specific hashing and signing."),
        ("13", "Assets", "13_stellar_assets", "assets1-2", "Classic assets: codes and issuers, trustlines, the <code>CODE:ISSUER</code> form, and a regulated asset paid out by a contract."),
    ]),
    ("Security: review the AI", [
        ("14", "Security", "14_security", "audit_auth, audit_init, audit_amount, audit_storage, audit_token",
         "Auth on the wrong address, re-callable initialize, negative amounts, expiring storage, trusting a caller's token."),
    ]),
    ("Ship it on Stellar", [
        ("15", "Deploy to testnet", "15_testnet", "4 labs + <code>sorolearn graduate</code>",
         "Deploy your own contracts, call them from real accounts, watch events, issue an asset and hold it in your contract, then take the on-chain exam."),
    ]),
]
sections = []
for group, rows in topics:
    count = len(rows)
    items = "".join(
        f'<a class="row" href="{TREE}/{folder}"><span class="n">{num}</span><span class="name">{name}</span>'
        f'<span class="desc">{desc}</span><span class="ex">{exs}</span></a>'
        for num, name, folder, exs, desc in rows
    )
    sections.append(
        f'<section class="group"><div class="group-title"><h2>{group}</h2>'
        f'<span>{count} topic{"s" if count != 1 else ""}</span></div><div class="rows">{items}</div></section>'
    )
curriculum = f"""
<div class="page-head">
<p class="eyebrow">Curriculum</p>
<h1 class="gradient-text">From your first contract to testnet.</h1>
<p class="lede">35 exercises across 16 topics and two quizzes, plus four testnet deployment labs. Each topic folder
has a README with background reading and links to the Stellar docs.</p>
</div>
{''.join(sections)}
"""
write("curriculum/index.html", page("Curriculum", "Every Sorolearn topic, from Stellar accounts to an on-chain exam.", "Curriculum", "../", curriculum))

