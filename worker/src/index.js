// Porthole site edge Worker.
//
// Sits in front of the GitHub Pages origin for porthole.thenightwatcher.online.
// Two jobs:
//   1. Markdown content negotiation: when an agent sends `Accept: text/markdown`,
//      reply with a Markdown view instead of HTML (token-cheap, machine-readable).
//   2. Advertise discovery resources via a `Link` response header (RFC 8288).
// Everything else passes straight through to GitHub Pages unchanged.

const LINK_HEADER = [
  '</llms.txt>; rel="alternate"; type="text/plain"',
  '</sitemap.xml>; rel="sitemap"; type="application/xml"',
  '</robots.txt>; rel="describedby"',
  '<https://github.com/ntd4996/Porthole>; rel="vcs-git"',
].join(', ');

// Markdown views, keyed by pathname. Returned only when the client negotiates
// `Accept: text/markdown`. Mirrors the human HTML page in machine-readable form.
const MARKDOWN = {
  '/': `# Porthole

> A free, open-source (MIT) macOS menu bar app that shows which dev ports are running, which project owns each one, and which tunnels point where, right from your menu bar.

## What it does

When you run a dozen dev servers across projects, \`lsof -i\` gets old fast. Porthole keeps a live list in your menu bar:

- **Running dev ports** with the process behind each one (\`vite\`, \`next\`, \`prisma\`, \`uvicorn\`).
- **Which project owns the port**, resolved from the process working directory (git root / \`package.json\` / \`go.mod\` / \`pyproject.toml\`), grouped per project.
- **Tunnels pointing at a port**, detected from ngrok, Cloudflare Tunnel, Tailscale, and localtunnel, with the public URL one click away.
- **Quick actions** per port: open \`localhost:PORT\`, copy the URL, or kill the process.
- **Ignore list** to hide noisy system services so you only watch real dev ports.

## Requirements

- macOS 14 (Sonoma) or later. Universal binary (Apple Silicon + Intel).

## Install

- Homebrew: \`brew install --cask ntd4996/tap/porthole\`
- Direct download: notarized DMG from the [latest release](https://github.com/ntd4996/Porthole/releases/latest).

## Links

- Home: https://porthole.thenightwatcher.online/
- Source (Swift 6, SwiftPM, MIT): https://github.com/ntd4996/Porthole
- Latest release: https://github.com/ntd4996/Porthole/releases/latest
- Changelog: https://github.com/ntd4996/Porthole/blob/main/CHANGELOG.md
- Auto-update feed (Sparkle): https://porthole.thenightwatcher.online/appcast.xml
- Agent reading list: https://porthole.thenightwatcher.online/llms.txt
`,
};

function wantsMarkdown(request) {
  const accept = request.headers.get('Accept') || '';
  // Only when markdown is explicitly requested (not a wildcard `*/*` browser).
  return /\btext\/markdown\b/i.test(accept);
}

export default {
  async fetch(request) {
    const url = new URL(request.url);

    if (request.method === 'GET' && wantsMarkdown(request)) {
      const md = MARKDOWN[url.pathname];
      if (md) {
        return new Response(md, {
          status: 200,
          headers: {
            'content-type': 'text/markdown; charset=utf-8',
            'link': LINK_HEADER,
            'vary': 'Accept',
            'cache-control': 'public, max-age=300',
          },
        });
      }
    }

    // Pass through to the GitHub Pages origin. Cloudflare loop-prevention sends
    // this subrequest to the origin, not back through this Worker.
    const originResp = await fetch(request);
    const resp = new Response(originResp.body, originResp);
    resp.headers.set('link', LINK_HEADER);
    resp.headers.append('vary', 'Accept');
    return resp;
  },
};
