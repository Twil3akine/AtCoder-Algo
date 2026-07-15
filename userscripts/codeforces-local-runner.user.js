// ==UserScript==
// @name         Codeforces Local Runner
// @namespace    https://github.com/Twil3akine/AtCoder-Algo
// @version      0.1.0
// @description  Codeforcesの問題ページでサンプルとカスタムテストをローカル実行します。
// @match        https://codeforces.com/problemset/problem/*/*
// @match        https://codeforces.com/contest/*/problem/*
// @match        https://codeforces.com/gym/*/problem/*
// @match        https://codeforces.com/group/*/contest/*/problem/*
// @grant        GM_xmlhttpRequest
// @connect      127.0.0.1
// @run-at       document-idle
// ==/UserScript==

(function () {
  "use strict";

  const RUNNER_URL = "http://127.0.0.1:4000";
  const STORAGE_PREFIX = "atcoder-algo:codeforces";
  const LANGUAGES = {
    rust: {
      label: "Rust",
      compilerName: "rust",
      template: "fn main() {\n    \n}\n",
    },
    python: {
      label: "Python",
      compilerName: "python",
      template: "def main():\n    pass\n\n\nif __name__ == \"__main__\":\n    main()\n",
    },
    pypy: {
      label: "PyPy",
      compilerName: "pypy",
      template: "def main():\n    pass\n\n\nif __name__ == \"__main__\":\n    main()\n",
    },
  };

  const problem = parseProblemLocation(location.pathname);
  const statement = document.querySelector(".problem-statement");
  if (!problem || !statement) return;

  const storageBase = `${STORAGE_PREFIX}:${problem.contestId}:${problem.index}`;
  const languageKey = `${storageBase}:language`;
  let selectedLanguage = localStorage.getItem(languageKey) || "rust";
  if (!LANGUAGES[selectedLanguage]) selectedLanguage = "rust";

  const root = document.createElement("section");
  root.id = "cf-local-runner";
  root.innerHTML = `
    <div class="cf-lr-header">
      <strong>Local Code Runner</strong>
      <span id="cf-lr-health" class="cf-lr-health pending">Checking runner...</span>
    </div>
    <div class="cf-lr-toolbar">
      <label>Language
        <select id="cf-lr-language">
          <option value="rust">Rust</option>
          <option value="python">Python</option>
          <option value="pypy">PyPy</option>
        </select>
      </label>
      <button type="button" id="cf-lr-run-samples" class="cf-lr-primary">Run Samples</button>
      <button type="button" id="cf-lr-copy">Copy Code</button>
      <a id="cf-lr-submit" class="cf-lr-button" target="_blank" rel="noopener noreferrer">Submit</a>
    </div>
    <textarea id="cf-lr-code" class="cf-lr-code" spellcheck="false" aria-label="Source code"></textarea>
    <div id="cf-lr-message" class="cf-lr-message" hidden></div>
    <div id="cf-lr-sample-results" class="cf-lr-results"></div>
    <details class="cf-lr-custom">
      <summary>Custom Test</summary>
      <label for="cf-lr-custom-input">Custom Input</label>
      <textarea id="cf-lr-custom-input" class="cf-lr-input" spellcheck="false"></textarea>
      <button type="button" id="cf-lr-run-custom" class="cf-lr-primary">Run Custom Test</button>
      <div id="cf-lr-custom-result" class="cf-lr-results"></div>
    </details>
  `;
  installStyles();
  statement.insertAdjacentElement("afterend", root);

  const languageSelect = root.querySelector("#cf-lr-language");
  const codeEditor = root.querySelector("#cf-lr-code");
  const healthLabel = root.querySelector("#cf-lr-health");
  const message = root.querySelector("#cf-lr-message");
  const sampleResults = root.querySelector("#cf-lr-sample-results");
  const customInput = root.querySelector("#cf-lr-custom-input");
  const customResult = root.querySelector("#cf-lr-custom-result");

  languageSelect.value = selectedLanguage;
  codeEditor.value = loadCode(selectedLanguage);
  configureSubmitLink(root.querySelector("#cf-lr-submit"), problem);

  languageSelect.addEventListener("change", () => {
    saveCode(selectedLanguage, codeEditor.value);
    selectedLanguage = languageSelect.value;
    localStorage.setItem(languageKey, selectedLanguage);
    codeEditor.value = loadCode(selectedLanguage);
  });
  codeEditor.addEventListener("input", () => saveCode(selectedLanguage, codeEditor.value));
  root.querySelector("#cf-lr-copy").addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(codeEditor.value);
      showMessage("Code copied to clipboard.", "ok");
    } catch (error) {
      showMessage(`Copy failed: ${error.message}`, "error");
    }
  });
  root.querySelector("#cf-lr-run-samples").addEventListener("click", runSamples);
  root.querySelector("#cf-lr-run-custom").addEventListener("click", runCustomTest);

  checkHealth();

  function codeKey(language) {
    return `${storageBase}:${language}:code`;
  }

  function loadCode(language) {
    return localStorage.getItem(codeKey(language)) ?? LANGUAGES[language].template;
  }

  function saveCode(language, code) {
    localStorage.setItem(codeKey(language), code);
  }

  async function checkHealth() {
    healthLabel.className = "cf-lr-health pending";
    healthLabel.textContent = "Checking runner...";
    try {
      const health = await requestJson("GET", "/health");
      const profiles = Array.isArray(health.profiles) ? health.profiles.join(", ") : "unknown";
      healthLabel.className = "cf-lr-health ok";
      healthLabel.textContent = `Runner online (${profiles})`;
      return true;
    } catch (_) {
      healthLabel.className = "cf-lr-health error";
      healthLabel.textContent = "Runner offline";
      showRunnerUnavailable();
      return false;
    }
  }

  async function runSamples() {
    const samples = readSamples();
    sampleResults.replaceChildren();
    message.hidden = true;
    if (samples.length === 0) {
      showMessage("Samples could not be found on this page.", "error");
      return;
    }
    if (!(await checkHealth())) return;

    setBusy(true);
    try {
      for (let index = 0; index < samples.length; index += 1) {
        const sample = samples[index];
        const result = await runSource(sample.input);
        sampleResults.append(renderSampleResult(index + 1, sample, result));
        if (result.status === "compileError" || result.status === "internalError") break;
      }
    } catch (_) {
      showRunnerUnavailable();
    } finally {
      setBusy(false);
    }
  }

  async function runCustomTest() {
    customResult.replaceChildren();
    message.hidden = true;
    if (!(await checkHealth())) return;
    setBusy(true);
    try {
      const result = await runSource(customInput.value);
      customResult.append(renderCustomResult(result));
    } catch (_) {
      showRunnerUnavailable();
    } finally {
      setBusy(false);
    }
  }

  function runSource(stdin) {
    const language = LANGUAGES[selectedLanguage];
    saveCode(selectedLanguage, codeEditor.value);
    return requestJson("POST", "/", {
      mode: "run",
      profile: "codeforces",
      compilerName: language.compilerName,
      sourceCode: codeEditor.value,
      stdin,
    });
  }

  function renderSampleResult(number, sample, result) {
    const normalizedExpected = normalizeOutput(sample.expected);
    const normalizedActual = normalizeOutput(result.stdout || "");
    let verdict;
    if (result.status === "ok") {
      verdict = normalizedExpected === normalizedActual ? "AC" : "WA";
    } else {
      verdict = statusVerdict(result.status);
    }

    const card = resultCard(`Sample ${number}`, verdict);
    appendMeta(card, result);
    if (verdict === "WA") {
      appendOutput(card, "Expected", sample.expected);
      appendOutput(card, "Actual", result.stdout || "");
    } else if (["CE", "RE", "ERROR"].includes(verdict)) {
      appendOutput(card, "stderr", result.stderr || "(empty)");
    }
    return card;
  }

  function renderCustomResult(result) {
    const verdict = result.status === "ok" ? "OK" : statusVerdict(result.status);
    const card = resultCard("Custom Test", verdict);
    appendMeta(card, result);
    appendOutput(card, "stdout", result.stdout || "(empty)");
    appendOutput(card, "stderr", result.stderr || "(empty)");
    return card;
  }

  function resultCard(title, verdict) {
    const card = document.createElement("article");
    card.className = "cf-lr-result-card";
    const heading = document.createElement("div");
    heading.className = "cf-lr-result-heading";
    const name = document.createElement("strong");
    name.textContent = title;
    const badge = document.createElement("span");
    badge.className = `cf-lr-verdict ${verdict.toLowerCase()}`;
    badge.textContent = verdict;
    heading.append(name, badge);
    card.append(heading);
    return card;
  }

  function appendMeta(card, result) {
    const meta = document.createElement("div");
    meta.className = "cf-lr-meta";
    const time = result.time == null ? "-" : `${result.time} ms`;
    const exitCode = result.exitCode == null ? "-" : String(result.exitCode);
    meta.textContent = `Time: ${time} / Exit code: ${exitCode}`;
    card.append(meta);
  }

  function appendOutput(card, title, value) {
    const details = document.createElement("details");
    details.open = ["stdout", "stderr", "Expected", "Actual"].includes(title);
    const summary = document.createElement("summary");
    summary.textContent = title;
    const pre = document.createElement("pre");
    pre.textContent = value;
    details.append(summary, pre);
    card.append(details);
  }

  function statusVerdict(status) {
    return {
      ok: "AC",
      compileError: "CE",
      runtimeError: "RE",
      timeLimitExceeded: "TLE",
      internalError: "ERROR",
    }[status] || "ERROR";
  }

  function showRunnerUnavailable() {
    showMessage(
      "Local runner is not running. AtCoder-Algo ディレクトリに入ると direnv により自動起動します。",
      "error",
    );
  }

  function showMessage(text, type) {
    message.hidden = false;
    message.className = `cf-lr-message ${type}`;
    message.textContent = text;
  }

  function setBusy(busy) {
    for (const button of root.querySelectorAll("button")) button.disabled = busy;
  }

  function readSamples() {
    const inputs = [...document.querySelectorAll(".sample-test .input pre")].map(readPre);
    const outputs = [...document.querySelectorAll(".sample-test .output pre")].map(readPre);
    return inputs.slice(0, Math.min(inputs.length, outputs.length)).map((input, index) => ({
      input,
      expected: outputs[index],
    }));
  }

  function readPre(pre) {
    const lines = pre.querySelectorAll(".test-example-line");
    if (lines.length > 0) {
      return [...lines].map((line) => line.textContent.replace(/\u00a0/g, " ")).join("\n") + "\n";
    }
    let value = "";
    const visit = (node) => {
      if (node.nodeType === Node.TEXT_NODE) value += node.nodeValue;
      else if (node.nodeName === "BR") value += "\n";
      else node.childNodes.forEach(visit);
    };
    pre.childNodes.forEach(visit);
    return value.replace(/\u00a0/g, " ").replace(/\r\n?/g, "\n");
  }

  function normalizeOutput(value) {
    return value
      .replace(/\r\n?/g, "\n")
      .split("\n")
      .map((line) => line.replace(/[ \t]+$/g, ""))
      .join("\n")
      .replace(/\n+$/g, "");
  }

  function configureSubmitLink(link, problemInfo) {
    const existing = [...document.querySelectorAll('a[href*="/submit"]')].find((anchor) =>
      /submit/.test(anchor.pathname),
    );
    link.href = existing?.href || `https://codeforces.com/contest/${problemInfo.contestId}/submit`;
  }

  function requestJson(method, path, body) {
    const url = `${RUNNER_URL}${path}`;
    if (typeof GM_xmlhttpRequest === "function") {
      return new Promise((resolve, reject) => {
        GM_xmlhttpRequest({
          method,
          url,
          timeout: method === "GET" ? 3000 : 75000,
          headers: body ? { "Content-Type": "application/json" } : undefined,
          data: body ? JSON.stringify(body) : undefined,
          onload(response) {
            if (response.status < 200 || response.status >= 300) {
              reject(new Error(`HTTP ${response.status}`));
              return;
            }
            try {
              resolve(JSON.parse(response.responseText));
            } catch (error) {
              reject(error);
            }
          },
          onerror: () => reject(new Error("runner connection failed")),
          ontimeout: () => reject(new Error("runner request timed out")),
        });
      });
    }
    return fetch(url, {
      method,
      headers: body ? { "Content-Type": "application/json" } : undefined,
      body: body ? JSON.stringify(body) : undefined,
    }).then((response) => {
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      return response.json();
    });
  }

  function parseProblemLocation(pathname) {
    const patterns = [
      /\/problemset\/problem\/(\d+)\/([^/]+)/,
      /\/(?:contest|gym)\/(\d+)\/problem\/([^/]+)/,
      /\/group\/[^/]+\/contest\/(\d+)\/problem\/([^/]+)/,
    ];
    for (const pattern of patterns) {
      const match = pathname.match(pattern);
      if (match) return { contestId: match[1], index: match[2] };
    }
    return null;
  }

  function installStyles() {
    const style = document.createElement("style");
    style.textContent = `
      #cf-local-runner { margin: 1.25rem 0; padding: 1rem; border: 1px solid #b9b9b9; border-radius: 6px; background: #fff; color: #222; font-family: Arial, sans-serif; }
      .cf-lr-header, .cf-lr-toolbar, .cf-lr-result-heading { display: flex; align-items: center; gap: .75rem; flex-wrap: wrap; }
      .cf-lr-header { justify-content: space-between; margin-bottom: .75rem; font-size: 1.1rem; }
      .cf-lr-toolbar { margin-bottom: .75rem; }
      .cf-lr-toolbar label { display: flex; align-items: center; gap: .4rem; }
      .cf-lr-button, #cf-local-runner button { border: 1px solid #999; border-radius: 4px; padding: .45rem .75rem; background: #f5f5f5; color: #222; cursor: pointer; text-decoration: none; font-size: 13px; }
      #cf-local-runner button:disabled { opacity: .55; cursor: wait; }
      #cf-local-runner .cf-lr-primary { background: #1976d2; border-color: #1976d2; color: #fff; }
      .cf-lr-code, .cf-lr-input { box-sizing: border-box; width: 100%; resize: vertical; border: 1px solid #aaa; border-radius: 4px; padding: .7rem; font: 13px/1.45 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; tab-size: 4; }
      .cf-lr-code { min-height: 340px; }
      .cf-lr-input { min-height: 120px; margin: .35rem 0 .65rem; }
      .cf-lr-health { border-radius: 999px; padding: .2rem .55rem; font-size: 12px; }
      .cf-lr-health.ok { color: #176b2c; background: #dff4e4; }
      .cf-lr-health.error { color: #9c1b1b; background: #fde2e2; }
      .cf-lr-health.pending { color: #765b00; background: #fff2bd; }
      .cf-lr-message { margin: .75rem 0; padding: .65rem; border-radius: 4px; white-space: pre-wrap; }
      .cf-lr-message.error { background: #fde2e2; color: #8d1717; }
      .cf-lr-message.ok { background: #dff4e4; color: #176b2c; }
      .cf-lr-results { display: grid; gap: .65rem; margin-top: .75rem; }
      .cf-lr-result-card { border: 1px solid #ddd; border-radius: 4px; padding: .7rem; }
      .cf-lr-result-heading { justify-content: space-between; }
      .cf-lr-verdict { border-radius: 4px; padding: .2rem .55rem; font-weight: bold; }
      .cf-lr-verdict.ac, .cf-lr-verdict.ok { background: #dff4e4; color: #176b2c; }
      .cf-lr-verdict.wa, .cf-lr-verdict.ce, .cf-lr-verdict.re, .cf-lr-verdict.tle, .cf-lr-verdict.error { background: #fde2e2; color: #9c1b1b; }
      .cf-lr-meta { margin: .4rem 0; color: #666; font-size: 12px; }
      .cf-lr-result-card pre { overflow: auto; max-height: 300px; padding: .6rem; background: #f7f7f7; white-space: pre-wrap; }
      .cf-lr-custom { margin-top: 1rem; }
      .cf-lr-custom > summary { cursor: pointer; font-weight: bold; margin-bottom: .6rem; }
    `;
    document.head.append(style);
  }
})();
