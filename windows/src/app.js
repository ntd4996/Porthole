"use strict";
const T = window.__TAURI__ || {};
const invoke = (cmd, args) => T.core.invoke(cmd, args);

/* ---------- i18n ---------- */
const I18N = {
  en: {
    refresh: "Refresh ports", ports: "Ports", ignored: "Ignored",
    scanning: "Scanning ports…", noports: "No dev ports running", other: "Other",
    footer: (p, t) => `${p} ports · ${t} tunnels`,
    checkupd: "Check for Updates", quit: "Quit",
    open: "Open in browser", copy: "Copy URL", killp: "Kill process", unignore: "Un-ignore",
    ignoreproc: (n) => `Ignore process ${n}`, ignoreport: (p) => `Ignore port ${p}`,
    hide: (n) => `Ignore ${n}`, killpid: (p) => `Kill PID ${p}?`, kill: "Kill", cancel: "Cancel",
    nothing: "Nothing ignored", rules: "Rules (not running)", addport: "Ignore a port…", add: "Add",
    uptodate: "You are on the latest version.", updating: (v) => `Downloading update ${v}…`,
    updfail: "Update check failed.", copied: "Copied URL",
  },
  vi: {
    refresh: "Làm mới cổng", ports: "Cổng", ignored: "Đã ẩn",
    scanning: "Đang quét cổng…", noports: "Không có cổng dev nào đang chạy", other: "Khác",
    footer: (p, t) => `${p} cổng · ${t} tunnel`,
    checkupd: "Kiểm tra cập nhật", quit: "Thoát",
    open: "Mở trong trình duyệt", copy: "Sao chép URL", killp: "Tắt tiến trình", unignore: "Bỏ ẩn",
    ignoreproc: (n) => `Ẩn tiến trình ${n}`, ignoreport: (p) => `Ẩn cổng ${p}`,
    hide: (n) => `Ẩn ${n}`, killpid: (p) => `Tắt PID ${p}?`, kill: "Tắt", cancel: "Huỷ",
    nothing: "Chưa ẩn gì", rules: "Quy tắc (không chạy)", addport: "Ẩn một cổng…", add: "Thêm",
    uptodate: "Bạn đang dùng phiên bản mới nhất.", updating: (v) => `Đang tải bản cập nhật ${v}…`,
    updfail: "Kiểm tra cập nhật thất bại.", copied: "Đã sao chép URL",
  },
  zh: {
    refresh: "刷新端口", ports: "端口", ignored: "已忽略",
    scanning: "正在扫描端口…", noports: "没有正在运行的开发端口", other: "其他",
    footer: (p, t) => `${p} 个端口 · ${t} 个隧道`,
    checkupd: "检查更新", quit: "退出",
    open: "在浏览器中打开", copy: "复制链接", killp: "结束进程", unignore: "取消忽略",
    ignoreproc: (n) => `忽略进程 ${n}`, ignoreport: (p) => `忽略端口 ${p}`,
    hide: (n) => `忽略 ${n}`, killpid: (p) => `结束 PID ${p}？`, kill: "结束", cancel: "取消",
    nothing: "没有忽略项", rules: "规则（未运行）", addport: "忽略一个端口…", add: "添加",
    uptodate: "已是最新版本。", updating: (v) => `正在下载更新 ${v}…`,
    updfail: "检查更新失败。", copied: "已复制链接",
  },
};
const LANG = (() => {
  const l = (navigator.language || "en").toLowerCase();
  if (l.startsWith("vi")) return "vi";
  if (l.startsWith("zh")) return "zh";
  return "en";
})();
const D = I18N[LANG];

/* ---------- icons ---------- */
const ICON = {
  open: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>',
  copy: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>',
  hide: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/></svg>',
  kill: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>',
  show: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>',
  globe: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>',
};

/* ---------- state ---------- */
const state = { ports: [], rules: { processes: [], ports: [] }, didScan: false, tab: "ports" };
let scanning = false;

const esc = (s) => String(s).replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]));

/* ---------- scanning ---------- */
async function refresh() {
  if (scanning) return;
  scanning = true;
  try {
    const res = await invoke("scan_ports");
    state.ports = res.ports || [];
    state.rules = res.rules || { processes: [], ports: [] };
    state.didScan = true;
  } catch (e) {
    console.error("scan failed", e);
  } finally {
    scanning = false;
    render();
  }
}

/* ---------- rendering ---------- */
function groupByProject(list) {
  const named = new Map();
  const other = [];
  for (const p of list) {
    if (p.project) {
      if (!named.has(p.project.name)) named.set(p.project.name, { kind: p.project.kind, ports: [] });
      named.get(p.project.name).ports.push(p);
    } else other.push(p);
  }
  const groups = [...named.keys()].sort().map((name) => ({ title: name, kind: named.get(name).kind, ports: named.get(name).ports }));
  if (other.length) groups.push({ title: "Other", kind: null, ports: other });
  return groups;
}

function tunnelPill(t) {
  const detail = t.public_url || `:${t.target_port}`;
  const inner = `${ICON.globe}<span>${esc(t.provider)}</span><span class="u">${esc(detail)}</span>`;
  if (t.public_url) return `<button class="pill ${esc(t.provider)}" data-act="openurl" data-url="${esc(t.public_url)}">${inner}</button>`;
  return `<span class="pill ${esc(t.provider)}">${inner}</span>`;
}

function portRow(p, mode) {
  const acts = mode === "ignored"
    ? `<button class="icon" data-act="open" data-port="${p.port}" title="${esc(D.open)}">${ICON.open}</button>
       <button class="icon" data-act="copy" data-port="${p.port}" title="${esc(D.copy)}">${ICON.copy}</button>
       <button class="icon" data-act="unignore" data-port="${p.port}" data-cmd="${esc(p.command)}" data-name="${esc(p.display_name)}" title="${esc(D.unignore)}">${ICON.show}</button>`
    : `<button class="icon" data-act="open" data-port="${p.port}" title="${esc(D.open)}">${ICON.open}</button>
       <button class="icon" data-act="copy" data-port="${p.port}" title="${esc(D.copy)}">${ICON.copy}</button>
       <button class="icon" data-act="ignore-proc" data-cmd="${esc(p.command)}" title="${esc(D.hide(p.command))}">${ICON.hide}</button>
       <button class="icon danger" data-act="kill" data-pid="${p.pid}" title="${esc(D.killp)}">${ICON.kill}</button>`;
  const pills = (p.tunnels || []).map(tunnelPill).join("");
  return `<div class="row" data-port="${p.port}" data-pid="${p.pid}" data-cmd="${esc(p.command)}" data-name="${esc(p.display_name)}" data-mode="${mode}">
      <div class="row-top"><span class="port">:${p.port}</span><span class="pname">${esc(p.display_name)}</span><span class="acts">${acts}</span></div>
      ${pills}
    </div>`;
}

function projectCard(g, mode) {
  const badge = g.kind && g.kind !== "unknown" ? `<span class="badge">${esc(g.kind)}</span>` : "";
  const title = g.title === "Other" ? D.other : g.title;
  return `<div class="card"><div class="card-h"><span class="name">${esc(title)}</span>${badge}</div>${g.ports.map((p) => portRow(p, mode)).join("")}</div>`;
}

function renderPorts() {
  const el = document.getElementById("panel-ports");
  const visible = state.ports.filter((p) => !p.ignored);
  if (visible.length) el.innerHTML = groupByProject(visible).map((g) => projectCard(g, "normal")).join("");
  else if (!state.didScan) el.innerHTML = `<div class="state"><div class="spinner"></div>${esc(D.scanning)}</div>`;
  else el.innerHTML = `<div class="state">${esc(D.noports)}</div>`;
}

function renderIgnored() {
  const el = document.getElementById("panel-ignored");
  const ignored = state.ports.filter((p) => p.ignored);
  const ci = (a, b) => a.toLowerCase() === b.toLowerCase();
  const uncoveredProcs = state.rules.processes
    .filter((n) => !ignored.some((p) => ci(n, p.command) || ci(n, p.display_name)))
    .sort();
  const uncoveredPorts = state.rules.ports.filter((pt) => !ignored.some((p) => p.port === pt)).sort((a, b) => a - b);

  let html = "";
  if (ignored.length) html += groupByProject(ignored).map((g) => projectCard(g, "ignored")).join("");
  if (uncoveredProcs.length || uncoveredPorts.length) {
    const rows = uncoveredProcs.map((n) => `<div class="rrow"><span class="rlabel">${esc(n)}</span><button class="icon" data-act="unignore-proc" data-name="${esc(n)}" title="${esc(D.unignore)}">${ICON.show}</button></div>`).join("")
      + uncoveredPorts.map((pt) => `<div class="rrow"><span class="rlabel mono">:${pt}</span><button class="icon" data-act="unignore-port" data-port="${pt}" title="${esc(D.unignore)}">${ICON.show}</button></div>`).join("");
    html += `<div class="card rules"><div class="card-h"><span class="name">${esc(D.rules)}</span></div>${rows}</div>`;
  }
  if (!ignored.length && !uncoveredProcs.length && !uncoveredPorts.length) html += `<div class="state">${esc(D.nothing)}</div>`;
  html += `<div class="addrow"><input id="addport" type="text" inputmode="numeric" placeholder="${esc(D.addport)}"/><button id="addbtn">${esc(D.add)}</button></div>`;
  el.innerHTML = html;

  const input = document.getElementById("addport");
  const doAdd = async () => { const v = parseInt(input.value, 10); if (!isNaN(v)) { await invoke("ignore_port", { port: v }); input.value = ""; refresh(); } };
  document.getElementById("addbtn").onclick = doAdd;
  input.onkeydown = (e) => { if (e.key === "Enter") doAdd(); };
}

function render() {
  renderPorts();
  renderIgnored();
  const visible = state.ports.filter((p) => !p.ignored);
  const tunnels = visible.reduce((n, p) => n + (p.tunnels ? p.tunnels.length : 0), 0);
  document.getElementById("count").textContent = D.footer(visible.length, tunnels);
  document.getElementById("panel-ports").hidden = state.tab !== "ports";
  document.getElementById("panel-ignored").hidden = state.tab !== "ignored";
}

/* ---------- actions ---------- */
async function copyUrl(port) {
  const url = `http://localhost:${port}`;
  try { await (T.clipboardManager ? T.clipboardManager.writeText(url) : navigator.clipboard.writeText(url)); toast(D.copied); }
  catch (e) { console.error(e); }
}

document.getElementById("app").addEventListener("click", async (e) => {
  const b = e.target.closest("[data-act]");
  if (!b) return;
  const act = b.dataset.act;
  if (act === "open") invoke("open_url", { url: `http://localhost:${b.dataset.port}` });
  else if (act === "openurl") invoke("open_url", { url: b.dataset.url });
  else if (act === "copy") copyUrl(b.dataset.port);
  else if (act === "ignore-proc") { await invoke("ignore_process", { name: b.dataset.cmd }); refresh(); }
  else if (act === "kill") confirmKill(parseInt(b.dataset.pid, 10));
  else if (act === "unignore") { await invoke("unignore_matching", { port: parseInt(b.dataset.port, 10), command: b.dataset.cmd, displayName: b.dataset.name }); refresh(); }
  else if (act === "unignore-proc") { await invoke("unignore_process", { name: b.dataset.name }); refresh(); }
  else if (act === "unignore-port") { await invoke("unignore_port", { port: parseInt(b.dataset.port, 10) }); refresh(); }
});

/* right-click context menu on a row */
document.getElementById("app").addEventListener("contextmenu", (e) => {
  const row = e.target.closest(".row");
  if (!row) return;
  e.preventDefault();
  closeMenu();
  const mode = row.dataset.mode;
  const items = mode === "ignored"
    ? [[D.unignore, () => invoke("unignore_matching", { port: +row.dataset.port, command: row.dataset.cmd, displayName: row.dataset.name }).then(refresh)]]
    : [
        [D.ignoreproc(row.dataset.cmd), () => invoke("ignore_process", { name: row.dataset.cmd }).then(refresh)],
        [D.ignoreport(row.dataset.port), () => invoke("ignore_port", { port: +row.dataset.port }).then(refresh)],
      ];
  const m = document.createElement("div");
  m.className = "ctxmenu";
  m.style.left = Math.min(e.clientX, window.innerWidth - 170) + "px";
  m.style.top = Math.min(e.clientY, window.innerHeight - 20 - items.length * 34) + "px";
  items.forEach(([label, fn]) => { const bt = document.createElement("button"); bt.textContent = label; bt.onclick = () => { closeMenu(); fn(); }; m.appendChild(bt); });
  document.body.appendChild(m);
});
function closeMenu() { document.querySelectorAll(".ctxmenu").forEach((n) => n.remove()); }
document.addEventListener("click", closeMenu);

function confirmKill(pid) {
  const bg = document.createElement("div");
  bg.className = "dialog-bg";
  bg.innerHTML = `<div class="dialog"><p>${esc(D.killpid(pid))}</p><div class="btns"><button data-x="cancel">${esc(D.cancel)}</button><button class="danger" data-x="kill">${esc(D.kill)}</button></div></div>`;
  bg.addEventListener("click", async (e) => {
    if (e.target === bg || e.target.dataset.x === "cancel") bg.remove();
    else if (e.target.dataset.x === "kill") { bg.remove(); await invoke("kill", { pid }); refresh(); }
  });
  document.body.appendChild(bg);
}

let toastTimer;
function toast(msg) {
  const el = document.getElementById("toast");
  el.textContent = msg; el.hidden = false;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => { el.hidden = true; }, 2200);
}

/* ---------- updater ---------- */
async function checkUpdate(silent) {
  if (!T.updater) { if (!silent) toast(D.updfail); return; }
  try {
    const update = await T.updater.check();
    if (update) {
      toast(D.updating(update.version));
      await update.downloadAndInstall();
      if (T.process) await T.process.relaunch();
    } else if (!silent) toast(D.uptodate);
  } catch (e) { console.error(e); if (!silent) toast(D.updfail); }
}

/* ---------- chrome wiring ---------- */
document.getElementById("refresh").title = D.refresh;
document.getElementById("tab-ports").textContent = D.ports;
document.getElementById("tab-ignored").textContent = D.ignored;
document.getElementById("check-updates").textContent = D.checkupd;
document.getElementById("quit").textContent = D.quit;

document.getElementById("refresh").onclick = refresh;
document.querySelectorAll(".tab").forEach((t) => (t.onclick = () => {
  state.tab = t.dataset.tab;
  document.querySelectorAll(".tab").forEach((x) => x.classList.toggle("on", x === t));
  render();
}));
document.getElementById("check-updates").onclick = () => checkUpdate(false);
document.getElementById("quit").onclick = () => { if (T.process) T.process.exit(0); };

if (T.event) {
  T.event.listen("porthole://refresh", refresh);
  T.event.listen("porthole://check-updates", () => checkUpdate(false));
}

/* auto-refresh while visible */
setInterval(() => { if (!document.hidden) refresh(); }, 4000);
document.addEventListener("visibilitychange", () => { if (!document.hidden) refresh(); });

render();
refresh();
checkUpdate(true);
