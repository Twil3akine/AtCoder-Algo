// ==UserScript==
// @name         AtCoder Local Runner
// @namespace    https://github.com/Twil3akine/AtCoder-Algo
// @version      0.1.0
// @description  AtCoderの問題ページで、現在のコードをローカル実行してサンプルを検証します。
// @match        https://atcoder.jp/contests/*/tasks/*
// @grant        unsafeWindow
// @grant        GM_xmlhttpRequest
// @connect      127.0.0.1
// @run-at       document-idle
// ==/UserScript==

(function () {
  "use strict";

  const RUNNER_URL = "http://127.0.0.1:4000";
  const PROFILE = "atcoder";
  if (document.getElementById("ac-local-runner")) return;

  const runOnlyButton = document.createElement("button");
  runOnlyButton.type = "button";
  runOnlyButton.id = "ac-lr-run-only";
  runOnlyButton.className = "btn btn-default";
  runOnlyButton.textContent = "実行";

  const runSamplesButton = document.createElement("button");
  runSamplesButton.type = "button";
  runSamplesButton.id = "ac-lr-run-samples";
  runSamplesButton.className = "btn btn-success";
  runSamplesButton.textContent = "実行して提出";

  const root = document.createElement("section");
  root.id = "ac-local-runner";
  root.innerHTML = `
    <div class="ac-lr-header">
      <span id="ac-lr-health" class="ac-lr-health pending">runnerを確認中...</span>
    </div>
    <div id="ac-lr-message" class="ac-lr-message" hidden></div>
    <div id="ac-lr-sample-results" class="ac-lr-results"></div>
    <details class="ac-lr-custom">
      <summary>カスタムテスト</summary>
      <label for="ac-lr-custom-input">標準入力</label>
      <textarea id="ac-lr-custom-input" spellcheck="false"></textarea>
      <button type="button" id="ac-lr-run-custom" class="btn btn-primary">実行</button>
      <div id="ac-lr-custom-result" class="ac-lr-results"></div>
    </details>
  `;

  installStyles();
  const submitButton = document.querySelector("#submit");
  const mount = submitButton?.closest(".form-group") || document.querySelector(".form-code-submit");
  if (!mount || !submitButton) return;
  submitButton.insertAdjacentElement("afterend", runOnlyButton);
  runOnlyButton.insertAdjacentElement("afterend", runSamplesButton);
  mount.insertAdjacentElement("afterend", root);

  const healthLabel = root.querySelector("#ac-lr-health");
  const message = root.querySelector("#ac-lr-message");
  const sampleResults = root.querySelector("#ac-lr-sample-results");
  const customInput = root.querySelector("#ac-lr-custom-input");
  const customResult = root.querySelector("#ac-lr-custom-result");

  runOnlyButton.addEventListener("click", () => runSamples(false));
  runSamplesButton.addEventListener("click", () => runSamples(true));
  root.querySelector("#ac-lr-run-custom").addEventListener("click", runCustomTest);

  const samples = readSamples();
  installSampleButtons(samples);
  checkHealth();

  async function checkHealth() {
    healthLabel.className = "ac-lr-health pending";
    healthLabel.textContent = "runnerを確認中...";
    try {
      const health = await requestJson("GET", "/health");
      if (!Array.isArray(health.profiles) || !health.profiles.includes(PROFILE)) {
        throw new Error("AtCoder profileがありません");
      }
      const version = health.versions?.rust?.atcoder;
      healthLabel.className = "ac-lr-health ok";
      healthLabel.textContent = version ? `接続済み (Rust ${version})` : "接続済み";
      return true;
    } catch (error) {
      healthLabel.className = "ac-lr-health error";
      healthLabel.textContent = "runner未接続";
      showMessage(
        `Local runnerへ接続できません。AtCoder-Algoディレクトリへ移動してrunnerを起動してください。\n${error.message}`,
        "error",
      );
      return false;
    }
  }

  async function runSamples(submitAfterAccepted) {
    sampleResults.replaceChildren();
    hideMessage();
    if (samples.length === 0) {
      showMessage("このページからサンプル入出力を取得できませんでした。", "error");
      return;
    }
    if (!(await checkHealth())) return;

    const language = selectedLanguage();
    if (!language) return;
    const sourceCode = getSourceCode();
    if (!sourceCode.trim()) {
      showMessage("ソースコードが空です。", "error");
      return;
    }

    setBusy(true);
    let allAccepted = true;
    try {
      const results = await runBatch(
        language.compilerName,
        sourceCode,
        samples.map((sample) => sample.input),
      );
      for (let index = 0; index < samples.length; index += 1) {
        const sample = samples[index];
        const result = results[index] || results[0];
        if (!result) throw new Error("runner returned no results");
        const verdict = sampleVerdict(sample, result);
        sampleResults.append(renderSampleResult(index + 1, sample, result, verdict));
        if (verdict !== "AC") allAccepted = false;
        if (result.status === "compileError" || result.status === "internalError") break;
      }

      if (allAccepted) {
        if (submitAfterAccepted) submitCurrentCode();
      }
    } catch (error) {
      showMessage(`実行リクエストに失敗しました。\n${error.message}`, "error");
    } finally {
      setBusy(false);
    }
  }

  async function runCustomTest() {
    customResult.replaceChildren();
    hideMessage();
    if (!(await checkHealth())) return;

    const language = selectedLanguage();
    if (!language) return;
    const sourceCode = getSourceCode();
    if (!sourceCode.trim()) {
      showMessage("ソースコードが空です。", "error");
      return;
    }

    setBusy(true);
    try {
      const result = await runSource(language.compilerName, sourceCode, customInput.value);
      customResult.append(renderCustomResult(result));
    } catch (error) {
      showMessage(`実行リクエストに失敗しました。\n${error.message}`, "error");
    } finally {
      setBusy(false);
    }
  }

  async function runOneSample(sample, number, button) {
    hideMessage();
    if (!(await checkHealth())) return;
    const language = selectedLanguage();
    if (!language) return;
    const sourceCode = getSourceCode();
    if (!sourceCode.trim()) {
      showMessage("ソースコードが空です。", "error");
      return;
    }

    button.disabled = true;
    try {
      const result = await runSource(language.compilerName, sourceCode, sample.input);
      const verdict = sampleVerdict(sample, result);
      sampleResults.prepend(renderSampleResult(number, sample, result, verdict));
    } catch (error) {
      showMessage(`実行リクエストに失敗しました。\n${error.message}`, "error");
    } finally {
      button.disabled = false;
    }
  }

  function runSource(compilerName, sourceCode, stdin) {
    return requestJson("POST", "/", {
      mode: "run",
      profile: PROFILE,
      compilerName,
      sourceCode,
      stdin,
    });
  }

  function runBatch(compilerName, sourceCode, stdins) {
    return requestJson("POST", "/", {
      mode: "batch",
      profile: PROFILE,
      compilerName,
      sourceCode,
      stdins,
    });
  }

  function selectedLanguage() {
    const select =
      document.querySelector("#select-lang select.current") ||
      document.querySelector('select[name="language_id"]');
    const label = select?.selectedOptions?.[0]?.textContent?.trim() || "";
    const value = select?.value || "";

    if (/rust/i.test(label)) return { compilerName: "rust", label };
    if (/pypy/i.test(label)) return { compilerName: "pypy", label };
    if (/python/i.test(label)) return { compilerName: "python", label };

    showMessage(
      `選択中の言語にはlocal runnerが対応していません。Rust、CPython、PyPyを選択してください。\nLanguage ID: ${value || "不明"}`,
      "error",
    );
    return null;
  }

  function getSourceCode() {
    if (typeof unsafeWindow?.getSourceCode === "function") {
      return String(unsafeWindow.getSourceCode());
    }

    const plain = document.querySelector("#plain-textarea, .plain-textarea");
    if (typeof unsafeWindow?.ace !== "undefined" && document.querySelector("#editor")) {
      const toggle = document.querySelector(".btn-toggle-editor");
      if (!toggle?.classList.contains("active")) {
        return unsafeWindow.ace.edit(document.querySelector("#editor")).getValue();
      }
    }
    if (plain) return plain.value;

    const textarea = document.querySelector('textarea[name="sourceCode"], textarea[name="source"]');
    return textarea?.value || "";
  }

  function submitCurrentCode() {
    const submit = document.querySelector("#submit");
    if (!submit) {
      showMessage("AtCoderの提出ボタンが見つかりませんでした。", "error");
      return;
    }
    submit.click();
  }

  function readSamples() {
    const selectors = [
      ["#task-statement pre.source-code-for-copy", ".part"],
      ["#task-statement .lang > *:first-child .div-btn-copy + pre", ".part"],
      ["#task-statement .div-btn-copy + pre", ".part"],
      ["#task-statement > .part section > pre", ".part"],
      ["#task-statement > .part:not(.io-style) > h3 + section > pre", ".part"],
      ["#task-statement pre", ".part"],
    ];

    for (const [selector, containerSelector] of selectors) {
      const blocks = [...document.querySelectorAll(selector)].filter(
        (pre) => !pre.closest(".io-style") && !pre.querySelector("var"),
      );
      if (blocks.length < 2 || blocks.length % 2 !== 0) continue;

      const parsed = [];
      for (let index = 0; index < blocks.length; index += 2) {
        const input = blocks[index];
        const output = blocks[index + 1];
        const container = input.closest(containerSelector) || input.parentElement;
        parsed.push({
          input: readPre(input),
          expected: readPre(output),
          anchor: container?.querySelector(".btn-copy, .div-btn-copy, h1, h2, h3, h4, h5, h6"),
        });
      }
      return parsed;
    }
    return [];
  }

  function installSampleButtons(testCases) {
    testCases.forEach((sample, index) => {
      if (!sample.anchor || sample.anchor.parentElement?.querySelector(".ac-lr-run-one")) return;
      const button = document.createElement("button");
      button.type = "button";
      button.className = "btn btn-default btn-xs ac-lr-run-one";
      button.textContent = "Local Run";
      button.addEventListener("click", () => runOneSample(sample, index + 1, button));
      sample.anchor.insertAdjacentElement("afterend", button);
    });
  }

  function readPre(pre) {
    let value = "";
    const visit = (node) => {
      if (node.nodeType === Node.TEXT_NODE) value += node.nodeValue;
      else if (node.nodeName === "BR") value += "\n";
      else node.childNodes.forEach(visit);
    };
    pre.childNodes.forEach(visit);
    return value.replace(/\u00a0/g, " ").replace(/\r\n?/g, "\n");
  }

  function sampleVerdict(sample, result) {
    if (result.status !== "ok") return statusVerdict(result.status);
    return normalizeOutput(result.stdout || "") === normalizeOutput(sample.expected) ? "AC" : "WA";
  }

  function renderSampleResult(number, sample, result, verdict) {
    const card = resultCard(`Sample ${number}`, verdict);
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
    appendOutput(card, "stdout", result.stdout || "(empty)");
    appendOutput(card, "stderr", result.stderr || "(empty)");
    return card;
  }

  function resultCard(title, verdict) {
    const card = document.createElement("article");
    card.className = "ac-lr-result-card";
    const heading = document.createElement("div");
    heading.className = "ac-lr-result-heading";
    const name = document.createElement("strong");
    name.textContent = title;
    const badge = document.createElement("span");
    badge.className = `ac-lr-verdict ${verdict.toLowerCase()}`;
    badge.textContent = verdict;
    heading.append(name, badge);
    card.append(heading);
    return card;
  }

  function appendOutput(card, title, value) {
    const details = document.createElement("details");
    details.open = true;
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

  function normalizeOutput(value) {
    return value
      .replace(/\r\n?/g, "\n")
      .split("\n")
      .map((line) => line.replace(/[ \t]+$/g, ""))
      .join("\n")
      .replace(/\n+$/g, "");
  }

  function showMessage(text, type) {
    message.hidden = false;
    message.className = `ac-lr-message ${type}`;
    message.textContent = text;
  }

  function hideMessage() {
    message.hidden = true;
  }

  function setBusy(busy) {
    runOnlyButton.disabled = busy;
    runSamplesButton.disabled = busy;
    for (const button of root.querySelectorAll("button")) button.disabled = busy;
    for (const button of document.querySelectorAll(".ac-lr-run-one")) button.disabled = busy;
  }

  function requestJson(method, path, body) {
    const url = `${RUNNER_URL}${path}`;
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

  function installStyles() {
    const style = document.createElement("style");
    style.textContent = `
      #ac-local-runner { margin: 20px 0; padding: 15px; border: 1px solid #ccc; border-radius: 4px; background: #fff; }
      .ac-lr-header, .ac-lr-result-heading { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
      .ac-lr-header { justify-content: space-between; margin-bottom: 12px; font-size: 16px; }
      #ac-lr-run-only, #ac-lr-run-samples { margin-left: 5px; }
      .ac-lr-health { border-radius: 999px; padding: 3px 8px; font-size: 12px; }
      .ac-lr-health.ok { color: #176b2c; background: #dff4e4; }
      .ac-lr-health.error { color: #9c1b1b; background: #fde2e2; }
      .ac-lr-health.pending { color: #765b00; background: #fff2bd; }
      .ac-lr-message { margin: 10px 0; padding: 9px; border-radius: 4px; white-space: pre-wrap; }
      .ac-lr-message.error { color: #8d1717; background: #fde2e2; }
      .ac-lr-message.ok { color: #176b2c; background: #dff4e4; }
      .ac-lr-results { display: grid; gap: 8px; margin-top: 10px; }
      .ac-lr-result-card { padding: 10px; border: 1px solid #ddd; border-radius: 4px; }
      .ac-lr-result-heading { justify-content: space-between; }
      .ac-lr-verdict { border-radius: 4px; padding: 3px 8px; font-weight: bold; }
      .ac-lr-verdict.ac, .ac-lr-verdict.ok { color: #176b2c; background: #dff4e4; }
      .ac-lr-verdict.wa, .ac-lr-verdict.ce, .ac-lr-verdict.re, .ac-lr-verdict.tle, .ac-lr-verdict.error { color: #9c1b1b; background: #fde2e2; }
      .ac-lr-result-card pre { overflow: auto; max-height: 300px; padding: 8px; background: #f7f7f7; white-space: pre-wrap; }
      .ac-lr-custom { margin-top: 12px; }
      .ac-lr-custom > summary { cursor: pointer; font-weight: bold; margin-bottom: 8px; }
      #ac-lr-custom-input { box-sizing: border-box; display: block; width: 100%; min-height: 110px; margin: 5px 0 8px; resize: vertical; font: 13px/1.45 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
      .ac-lr-run-one { margin-left: 8px; vertical-align: middle; }
    `;
    document.head.append(style);
  }
})();
