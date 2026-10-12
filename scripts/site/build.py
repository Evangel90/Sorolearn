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
  <a class="badge" href="curriculum/"><span class="dot"></span>Built for Stellar <span class="sep">·</span> soroban-sdk 29 <span class="sep">·</span> Protocol 29 <span class="arrow">→</span></a>
  <h1 class="display"><span class="gradient-text">AI writes it.</span><br><span class="second-line">Can you fix it?</span></h1>
  <p class="sub">Most Soroban code will be written with AI. Sorolearn gives beginners the foundation to understand it,
  review it and catch its mistakes: 35 hands-on exercises, from your first contract to fixing AI-written exploits
  and passing an exam graded on Stellar testnet.</p>
  <div class="buttons">
    <a class="btn primary" href="setup/">Start learning <span class="arrow">→</span></a>
    <a class="btn" href="{CODESPACES}">Try it in your browser</a>
  </div>
  <p class="fineprint">Free and open source · No install needed in Codespaces · Works offline after setup</p>

  <div class="window">
    <div class="window-bar"><i></i><i></i><i></i><span>sorolearn: watch mode</span></div>
    <div class="player" id="demo" aria-label="Terminal recording: Sorolearn watch mode catches an exploit in an AI-written contract, the fix is saved, and the exercise passes"></div>
  </div>
  <p class="caption">An AI-written token contract lets anyone drain anyone. Sorolearn's exploit test catches it, a one-line
  fix lands, and the exercise turns green.</p>
</section>

<section class="stats-strip" aria-label="Sorolearn in numbers">
  <div><strong>35</strong><span>hands-on exercises</span></div>
  <div><strong>5</strong><span>real-world exploits to fix</span></div>
  <div><strong>4</strong><span>testnet deployment labs</span></div>
  <div><strong>1</strong><span>exam graded on-chain</span></div>
  <div><strong>0</strong><span>backend servers</span></div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">Why Sorolearn</p>
    <h2>You won't write every line by hand.<br><span class="muted">You still need to know when it's wrong.</span></h2>
  </div>
  <div class="pillars">
    <div class="pillar">
      <h3>AI writes the first draft now</h3>
      <p>Ask for a token, a vault or a vote and you get a contract that compiles, looks right and
      passes the obvious tests, in seconds.</p>
    </div>
    <div class="pillar">
      <h3>It fails where it costs money</h3>
      <p>A <code>require_auth</code> on the wrong address. A negative amount. Storage that quietly expires.
      Bugs like these only get caught by someone who understands the platform.</p>
    </div>
    <div class="pillar">
      <h3>Fixing code builds that understanding</h3>
      <p>Every exercise is broken on purpose. Repairing it teaches the same skill as reviewing AI output:
      reading a contract and knowing exactly why it fails.</p>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">The course</p>
    <h2>From first contract to testnet in four movements.</h2>
    <p class="lede">Every exercise is a small, broken program with tests. Save the file and Sorolearn re-runs them
    instantly, so you always know what's wrong and exactly when it's fixed.</p>
  </div>
  <div class="movements">
    <div class="movement">
      <span class="num">1.0</span>
      <h3>Write contracts</h3>
      <p>Functions, types, storage and TTL, errors, auth, events, tokens, cross-contract calls and time.</p>
      <ul><li>topics 01 → 10 · 2 quizzes</li><li>23 exercises</li></ul>
    </div>
    <div class="movement">
      <span class="num">2.0</span>
      <h3>Understand Stellar</h3>
      <p>The network under your contracts: accounts and keys, multisig, transactions and signatures, classic assets.</p>
      <ul><li>topics 11 → 13</li><li>checked against the Stellar CLI</li></ul>
    </div>
    <div class="movement">
      <span class="num">3.0</span>
      <h3>Review the AI</h3>
      <p>Plausible, AI-style contracts with one planted vulnerability each. An exploit test proves the attack works until you fix it.</p>
      <ul><li>topic 14</li><li>5 security audits</li></ul>
    </div>
    <div class="movement">
      <span class="num">4.0</span>
      <h3>Ship to testnet</h3>
      <p>Deploy what you wrote, call it from real accounts, watch its events, then pass the on-chain exam.</p>
      <ul><li>topic 15</li><li>4 labs + graduation</li></ul>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">Built on Stellar, end to end</p>
    <h2>Not a tutorial about Stellar.<br><span class="muted">A course that runs on it.</span></h2>
  </div>
  <div class="bento">
    <div class="tile wide">
      <span class="label">registry · <a href="https://stellar.expert/explorer/testnet/contract/CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY">CD23Z2…FWYY on stellar.expert</a></span>
      <h3>An exam graded by a smart contract</h3>
      <p><code>sorolearn graduate</code> uploads your contract to Stellar testnet. A registry contract deploys a
      fresh copy, examines it with cross-contract calls, and only then records you and emits a
      <code>Graduated</code> event. No one self-reports; anyone can verify. <a href="graduates/">See the graduates →</a></p>
      <div class="mini-term"><span class="p">$</span> sorolearn graduate
[5/6] Upload the contract code to testnet
[6/6] Ask the registry to examine your contract and record you
<span class="ok">✓ You are Sorolearn graduate #2!</span></div>
    </div>
    <div class="tile narrow">
      <span class="label">topic 15</span>
      <h3>Real deployments</h3>
      <p>Deploy your own counter, enforce admin auth from real accounts, stream events, and hold a classic asset you
      issued inside your contract through the Stellar Asset Contract.</p>
    </div>
    <div class="tile narrow">
      <span class="label">topics 11 → 13</span>
      <h3>The network, verified</h3>
      <p>Keys, StrKeys, reserves, multisig, transaction hashing and signing, built with the real Stellar types. The
      expected hash matches <code>stellar tx hash</code> byte for byte.</p>
    </div>
    <div class="tile wide">
      <span class="label">topic 14 · audit_auth.rs</span>
      <h3>Security, the way it actually fails</h3>
      <p>It compiles. It calls <code>require_auth</code>. The happy path passes. And anyone can drain anyone,
      because it asks the <em>recipient</em> to sign. Five audits like this: wrong signer, re-initialization,
      negative amounts, expiring storage, a fake token.</p>
      <div class="diff"><div class="dim">pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {{</div><div class="del">-    to.require_auth();</div><div class="add">+    from.require_auth();</div><div class="dim">    let from_balance = Self::balance(env.clone(), from.clone());</div></div>
    </div>
  </div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">Engineered like a product</p>
    <h2>Every exercise is proven, on every push.</h2>
  </div>
  <div class="checks">
    <div class="check"><span>✓</span><div><h3>Solutions pass, exercises fail</h3><p>CI builds all 35 reference solutions and confirms each exercise fails until it's fixed.</p></div></div>
    <div class="check"><span>✓</span><div><h3>Audits fail for the right reason</h3><p>Each security exercise must compile and pass its happy path. Only the exploit test may fail.</p></div></div>
    <div class="check"><span>✓</span><div><h3>Current to the network</h3><p>Pinned to soroban-sdk 29 for protocol 29, the version testnet and mainnet run today.</p></div></div>
    <div class="check"><span>✓</span><div><h3>Offline by design</h3><p>One setup, then every build runs offline against a committed lockfile. Workshop Wi-Fi can't stop it.</p></div></div>
    <div class="check"><span>✓</span><div><h3>Zero-install option</h3><p>One click opens a prebuilt Codespace with Rust, the Stellar CLI and a compiled SDK.</p></div></div>
    <div class="check"><span>✓</span><div><h3>Instant feedback</h3><p>Watch mode re-runs on save and responds to single keys: <code>n</code> next, <code>h</code> hint.</p></div></div>
  </div>
</section>

<section class="section">
  <div class="section-head">
    <p class="eyebrow">Who it's for</p>
    <h2>Built to grow the Stellar developer ecosystem.</h2>
  </div>
  <div class="pillars">
    <div class="pillar">
      <h3>Developers new to Stellar</h3>
      <p>Rust or Solidity background? Go from zero to a deployed, auth-checked contract on testnet in one guided path.</p>
    </div>
    <div class="pillar">
      <h3>Hackathon builders</h3>
      <p>Learn the patterns and the pitfalls before the clock starts, then ship with the exact CLI commands you practiced.</p>
    </div>
    <div class="pillar">
      <h3>Educators and communities</h3>
      <p>Run a workshop in a browser tab or fully offline. Open source and MIT licensed: add your own exercises in minutes.</p>
    </div>
  </div>
</section>

<section class="cta">
  <h2 class="gradient-text" style="font-size:clamp(36px,6vw,56px)">Learn Soroban by fixing it.</h2>
  <p class="lede" style="margin:0 auto 24px">Clone it and run <code>sorolearn</code>, or open it in your browser. Your first contract is
  one save away.</p>
  <div class="install">
<pre><code>git clone {REPO}.git && cd Sorolearn && cargo install --path . && sorolearn</code></pre>
  </div>
  <div class="buttons">
    <a class="btn primary" href="setup/">Start learning <span class="arrow">→</span></a>
    <a class="btn" href="{REPO}">View on GitHub</a>
  </div>
</section>
"""
write("index.html", page("Sorolearn", "AI writes it. Can you fix it? 35 hands-on Soroban exercises, from your first smart contract to fixing AI-written exploits and an exam graded on Stellar testnet.",
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
<p>On macOS:</p>
<pre><code>brew install stellar-cli</code></pre>
<p>Anywhere else:</p>
<pre><code>cargo install --locked stellar-cli</code></pre>
<p>Then add the WebAssembly target:</p>
<pre><code>rustup target add wasm32v1-none</code></pre>

<h2>Troubleshooting</h2>
<ul>
  <li><strong>"soroban-sdk needs Rust 1.91 or newer"</strong>: run <code class="cmd">rustup update stable</code>.</li>
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
<thead><tr><th>Command <span class="muted" style="text-transform:none;letter-spacing:0">(click to copy)</span></th><th>What it does</th></tr></thead>
<tbody>
<tr><td><code class="cmd">sorolearn</code></td><td>watch mode</td></tr>
<tr><td><code class="cmd">sorolearn list</code></td><td>all exercises and your progress</td></tr>
<tr><td><code class="cmd">sorolearn run [name]</code></td><td>compile and test one exercise</td></tr>
<tr><td><code class="cmd">sorolearn hint [name]</code></td><td>show a hint</td></tr>
<tr><td><code class="cmd">sorolearn reset &lt;name&gt;</code></td><td>restore an exercise to its original state</td></tr>
<tr><td><code class="cmd">sorolearn verify</code></td><td>check every exercise, in order</td></tr>
<tr><td><code class="cmd">sorolearn setup</code></td><td>one-time download and pre-build (enables offline use)</td></tr>
<tr><td><code class="cmd">sorolearn build-wasm &lt;name&gt;</code></td><td>compile an exercise to a deployable <code>.wasm</code></td></tr>
<tr><td><code class="cmd">sorolearn graduate [identity]</code></td><td>deploy your quiz1 poll to testnet and take the on-chain exam</td></tr>
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

