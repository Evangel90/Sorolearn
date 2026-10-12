// Copy buttons for command blocks, and click-to-copy for inline `code.cmd`.
// Copied text drops shell comments (`.comment` spans) and leading `$ ` prompts,
// so it can be pasted straight into a terminal.
(() => {
  const COPY_ICON =
    '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>';
  const CHECK_ICON =
    '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 6 9 17l-5-5"/></svg>';

  function commandText(el) {
    const clone = el.cloneNode(true);
    clone.querySelectorAll(".comment, .code-buttons").forEach((c) => c.remove());
    return clone.textContent
      .split("\n")
      .map((line) => line.replace(/^\s*\$\s+/, "").replace(/\s+$/, ""))
      .filter((line) => line.length > 0)
      .join("\n");
  }

  async function writeClipboard(text) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      // Fallback for browsers without the async clipboard API.
      const area = document.createElement("textarea");
      area.value = text;
      area.setAttribute("readonly", "");
      area.style.position = "fixed";
      area.style.opacity = "0";
      document.body.appendChild(area);
      area.select();
      let ok = false;
      try {
        ok = document.execCommand("copy");
      } catch {
        ok = false;
      }
      area.remove();
      return ok;
    }
  }

  function flash(el, label, done) {
    el.classList.add("copied");
    if (label) label.textContent = "Copied";
    clearTimeout(el._copyTimer);
    el._copyTimer = setTimeout(() => {
      el.classList.remove("copied");
      done();
    }, 1600);
  }

  // Like mdBook (the Rust Book): the button lives inside the <pre> itself,
  // top-right, and appears when the block is hovered.
  document.querySelectorAll("main pre").forEach((pre) => {
    // The asciinema player renders its terminal as a <pre>; it isn't a command.
    if (pre.closest(".player, .window")) return;

    const buttons = document.createElement("div");
    buttons.className = "code-buttons";
    const button = document.createElement("button");
    button.type = "button";
    button.className = "clip-button";
    button.title = "Copy to clipboard";
    button.setAttribute("aria-label", "Copy to clipboard");
    button.innerHTML = `${COPY_ICON}<span class="tooltiptext" role="status"></span>`;
    buttons.appendChild(button);
    pre.insertBefore(buttons, pre.firstChild);

    const tooltip = button.querySelector(".tooltiptext");
    button.addEventListener("click", async () => {
      const ok = await writeClipboard(commandText(pre.querySelector("code") || pre));
      tooltip.textContent = ok ? "Copied!" : "Copy failed";
      button.classList.toggle("copied", ok);
      button.firstElementChild.outerHTML = ok ? CHECK_ICON : COPY_ICON;
      flash(button, null, () => {
        tooltip.textContent = "";
        button.firstElementChild.outerHTML = COPY_ICON;
      });
    });
  });

  document.querySelectorAll("code.cmd").forEach((code) => {
    code.tabIndex = 0;
    code.setAttribute("role", "button");
    code.title = "Click to copy";
    const copy = async () => {
      if (await writeClipboard(commandText(code))) flash(code, null, () => {});
    };
    code.addEventListener("click", copy);
    code.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        copy();
      }
    });
  });
})();
