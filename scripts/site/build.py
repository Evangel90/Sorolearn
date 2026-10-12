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
    cols: 100, rows: 30, idleTimeLimit: 2, poster: "npt:0:18.3", fit: "width", theme: "asciinema", preload: true
  });
</script>
"""

home = f"""
<section class="hero">
  <div class="hero-text">
    <span class="pill">Stellar · Soroban · Rust</span>
    <h1>Sorolearn</h1>
    <p class="tagline">Small exercises to get you used to building on Stellar and writing Soroban smart contracts.</p>
    <p class="lede">Every exercise is a tiny piece of broken code with tests. Fix it, save, and Sorolearn tells you
    instantly whether you got it. Start with the network itself, then write, test and <em>review</em> contracts.</p>
    <div class="buttons">
      <a class="btn primary" href="setup/">Get started</a>
      <a class="btn" href="{CODESPACES}">Open in Codespaces</a>
    </div>
    <p class="muted"><strong>AI can write your Soroban contract. Sorolearn teaches you to tell when it's wrong.</strong></p>
  </div>
  <div>
    <div class="player" id="demo" aria-label="Terminal recording: Sorolearn watch mode catches an exploit, the fix is saved, the exercise passes"></div>
  </div>
</section>

<section class="grid" aria-label="Features">
  <div class="card">
    <h3>Stellar first</h3>
    <p>Accounts, keys and StrKeys, reserves, multisig, transactions, network-specific signatures and
    classic assets, built with the real Stellar types and checked against the Stellar CLI.</p>
  </div>
  <div class="card">
    <h3>Soroban by fixing it</h3>
    <p>Contract functions, types, storage and TTL, errors, auth, events, tokens, cross-contract calls and
    ledger time. Each exercise compiles against <code>soroban-sdk</code> 29.</p>
  </div>
  <div class="card">
    <h3>Review the AI</h3>
    <p>A security track of plausible, AI-style contracts with one planted vulnerability each. The happy path
    passes; an exploit test proves the attack. Fix the contract.</p>
  </div>
  <div class="card">
    <h3>Graduate on-chain</h3>
    <p><code>sorolearn graduate</code> deploys your contract to testnet, where a registry contract examines it
    with cross-contract calls and records you. <a href="graduates/">See the graduates.</a></p>
  </div>
  <div class="card">
    <h3>Offline or in the browser</h3>
    <p>Run it locally (fully offline after a one-time setup) or open it in GitHub Codespaces with Rust, the
    Stellar CLI and a pre-built SDK ready to go.</p>
  </div>
  <div class="card">
    <h3>Watch mode &amp; hints</h3>
    <p>Sorolearn re-runs the tests every time you save. Stuck? Type <code>h</code> for a hint, read the topic
    README, then compare with the reference solution once you pass.</p>
  </div>
</section>

<section class="prose">
  <h2>Quick start</h2>
<pre><code>git clone {REPO}.git sorolearn
cd sorolearn
cargo install --path .
sorolearn setup   <span class="comment"># once: downloads and pre-builds the SDK</span>
sorolearn         <span class="comment"># start watch mode</span></code></pre>
  <p>Need Rust? See <a href="setup/">Setup</a>. Want to skip installing anything?
  <a href="{CODESPACES}">Open it in GitHub Codespaces</a>.</p>
</section>
"""
write("index.html", page("Sorolearn", "Small exercises to get you used to building on Stellar and writing Soroban smart contracts.",
                         None, "", home, PLAYER_HEAD, PLAYER_JS))

# ----------------------------------------------------------------- setup
setup = f"""
<div class="prose">
<h1>Setup</h1>
<p class="lede">Two ways to run Sorolearn: in your browser with nothing to install, or locally, where it
works fully offline after a one-time setup.</p>

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
<div class="prose">
<h1>Usage</h1>
<p class="lede">Read, fix, save, repeat. Sorolearn takes care of compiling and testing.</p>

<h2>Watch mode</h2>
<pre><code>sorolearn</code></pre>
<p>Sorolearn shows the current exercise, for example <code>exercises/07_storage/storage1.rs</code>. Open it in
your editor, read the comments, and make the code compile and the tests pass. Every time you save, Sorolearn
re-runs the tests. When the exercise passes, type <code>n</code> + Enter for the next one.</p>
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

<h2>Three kinds of exercises</h2>
<h3>Stellar lessons</h3>
<p>Plain Rust against the real Stellar types: derive keys and addresses, apply reserve and multisig rules,
build, hash and sign transactions, parse assets. Expected values (like a transaction hash) were checked
against the Stellar CLI.</p>
<h3>Soroban contracts</h3>
<p>Small <code>#![no_std]</code> contracts tested against Soroban's simulated environment: no network needed.</p>
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
        ("00", "intro", "00_intro", "intro1-2", "Your first contract: <code>#[contract]</code>, <code>#[contractimpl]</code>, <code>Env</code> and test clients."),
    ]),
    ("The Stellar network", [
        ("01", "Accounts", "01_stellar_accounts", "accounts1-3", "Lumens and stroops, reserves, ed25519 keys, StrKeys (<code>G…</code> <code>S…</code> <code>C…</code> <code>M…</code>), multisig thresholds."),
        ("02", "Transactions", "02_stellar_transactions", "transactions1-2", "Operations, sequence numbers, fees, memos, XDR, network-specific hashing and signing."),
        ("03", "Assets", "03_stellar_assets", "assets1", "Classic assets: codes and issuers, trustlines, the <code>CODE:ISSUER</code> form."),
    ]),
    ("Soroban smart contracts", [
        ("04", "Functions", "04_functions", "functions1-2", "Exported functions and the types allowed across the contract boundary."),
        ("05", "Types", "05_types", "symbols1, vecs1, maps1, bytes1", "<code>Symbol</code>, <code>Vec</code>, <code>Map</code>, <code>Bytes</code>, <code>String</code>."),
        ("06", "Custom types", "06_custom_types", "types1-2", "<code>#[contracttype]</code> structs and enums."),
        ("07", "Storage", "07_storage", "storage1-3", "Instance, persistent and temporary storage, <code>DataKey</code>, TTL."),
        ("08", "Errors", "08_errors", "errors1-2", "<code>#[contracterror]</code>, <code>Result</code>, <code>panic_with_error!</code>, checked math."),
        ("09", "Auth", "09_auth", "auth1-2", "<code>require_auth</code>, the admin pattern, constructors."),
        ("10", "Events", "10_events", "events1", "<code>#[contractevent]</code>, topics and data."),
        ("11", "Tokens", "11_tokens", "tokens1-2", "<code>TokenClient</code>, contracts holding funds, regulated classic assets via the Stellar Asset Contract."),
        ("12", "Cross-contract", "12_cross_contract", "cross1", "Calling other contracts."),
        ("13", "Time", "13_time", "time1", "Ledger timestamp vs sequence."),
        ("★", "Quizzes", "quizzes", "quiz1-2", "A poll and a time-locked token vault: everything together."),
    ]),
    ("Security: review the AI", [
        ("14", "Security", "14_security", "audit_auth, audit_init, audit_amount, audit_storage, audit_token",
         "Auth on the wrong address, re-callable initialize, negative amounts, expiring storage, trusting a caller's token."),
    ]),
    ("On the network", [
        ("15", "Testnet &amp; graduation", "15_testnet", "lab + <code>sorolearn graduate</code>",
         "Issue an asset, open a trustline, pay, use the SAC from Soroban, deploy your contract, take the on-chain exam."),
    ]),
]
sections = []
for group, rows in topics:
    cards = "".join(
        f'<div class="card"><div class="num">{num}</div><h3><a href="{TREE}/{folder}">{name}</a></h3>'
        f'<p>{desc}</p><p class="muted"><code>{exs}</code></p></div>' if "<code>" not in exs else
        f'<div class="card"><div class="num">{num}</div><h3><a href="{TREE}/{folder}">{name}</a></h3>'
        f'<p>{desc}</p><p class="muted">{exs}</p></div>'
        for num, name, folder, exs, desc in rows
    )
    sections.append(f'<h2>{group}</h2><div class="grid">{cards}</div>')
curriculum = f"""
<h1>Curriculum</h1>
<p class="lede">35 exercises across 16 topics and two quizzes, plus a testnet lab. Each topic folder has a README with background reading
and links to the Stellar docs.</p>
{''.join(sections)}
"""
write("curriculum/index.html", page("Curriculum", "Every Sorolearn topic, from Stellar accounts to an on-chain exam.", "Curriculum", "../", curriculum))

