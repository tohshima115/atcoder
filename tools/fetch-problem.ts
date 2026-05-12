import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { homedir } from "node:os";
import { join, basename } from "node:path";
import * as cheerio from "cheerio";
import TurndownService from "turndown";

const COOKIE_JAR = join(homedir(), ".local/share/online-judge-tools/cookie.jar");

function readRevelSession(): string | null {
  if (!existsSync(COOKIE_JAR)) return null;
  const jar = readFileSync(COOKIE_JAR, "utf8");
  const m = jar.match(/Set-Cookie3: REVEL_SESSION="([^"]+)"/);
  return m ? m[1] : null;
}

async function get(url: string): Promise<string> {
  const session = readRevelSession();
  const headers: Record<string, string> = {
    "User-Agent": "Mozilla/5.0 atcoder-md",
  };
  if (session) headers["Cookie"] = `REVEL_SESSION=${session}`;
  const res = await fetch(url, { headers });
  if (!res.ok) throw new Error(`HTTP ${res.status} for ${url}`);
  return res.text();
}

function pickTaskStatement(html: string): string {
  const $ = cheerio.load(html);
  const ja = $("#task-statement span.lang-ja");
  if (ja.length) return ja.html() ?? "";
  const en = $("#task-statement span.lang-en");
  if (en.length) return en.html() ?? "";
  return $("#task-statement").html() ?? "";
}

function pickTitle(html: string): string {
  const $ = cheerio.load(html);
  // span.h2 contains child links (Editorial, etc.) — keep only direct text nodes.
  const $h2 = $("span.h2").first();
  const title = $h2.length
    ? $h2.clone().children().remove().end().text().trim()
    : $("title").text().trim();
  return title.replace(/\s+/g, " ");
}

function htmlToMarkdown(html: string): string {
  const td = new TurndownService({
    headingStyle: "atx",
    codeBlockStyle: "fenced",
    bulletListMarker: "-",
    emDelimiter: "_",
  });
  // AtCoder uses <var> for inline math. Wrap in $...$ for KaTeX/MathJax rendering.
  td.addRule("var", {
    filter: ["var"],
    replacement: (content) => `$${content.replace(/\\_/g, "_")}$`,
  });
  let md = td.turndown(html);
  // Cleanup: AtCoder math uses LaTeX with \( \) \[ \] delimiters.
  // After turndown's backslash escaping, restore math syntax to KaTeX-friendly $...$.
  md = md.replace(/\\\\\(/g, "$").replace(/\\\\\)/g, "$");
  md = md.replace(/\\\\\[/g, "$$").replace(/\\\\\]/g, "$$");
  // Unescape doubled backslashes that came from LaTeX commands (\\leq → \leq).
  md = md.replace(/\\\\([a-zA-Z])/g, "\\$1");
  return md;
}

function autoDetect(): { contest: string; problem?: string } | null {
  // Use APMD_PWD if set (apmd shell function passes the caller's cwd here,
  // because the subshell may have cd'd elsewhere). Fall back to process.cwd().
  const cwd = process.env.APMD_PWD || process.cwd();
  // ts/abcXXX/<letter>/  → contest + problem
  const tsM = cwd.match(/\/ts\/([a-z]+\d+)\/([a-z][a-z0-9]?)(?:\/|$)/);
  if (tsM) return { contest: tsM[1], problem: tsM[2] };
  // rust/abcXXX/  → contest only (problem must be specified)
  const rustM = cwd.match(/\/rust\/([a-z]+\d+)(?:\/|$)/);
  if (rustM) return { contest: rustM[1] };
  return null;
}

function buildUrl(contest: string, problem: string): string {
  return `https://atcoder.jp/contests/${contest}/tasks/${contest}_${problem}`;
}

async function fetchTasksList(contest: string): Promise<string[]> {
  const html = await get(`https://atcoder.jp/contests/${contest}/tasks`);
  const $ = cheerio.load(html);
  const problems = new Set<string>();
  $("table.table tbody tr").each((_, tr) => {
    const link = $(tr).find("td:first-child a").attr("href");
    if (!link) return;
    const m = link.match(/\/tasks\/[a-z]+\d+_([a-z][a-z0-9]?)/);
    if (m) problems.add(m[1]);
  });
  return [...problems];
}

async function fetchOne(url: string, outPath: string): Promise<void> {
  const html = await get(url);
  const title = pickTitle(html);
  const taskHtml = pickTaskStatement(html);
  if (!taskHtml) throw new Error(`No #task-statement at ${url}`);
  const md = htmlToMarkdown(taskHtml);
  const out = `# ${title}\n\nSource: ${url}\n\n${md}\n`;
  writeFileSync(outPath, out);
  console.log(`✓ ${outPath}  (${title})`);
}

function printUsage(): void {
  console.error(`Usage:
  apmd                          auto-detect contest+problem from cwd
  apmd <letter>                 use auto-detected contest, specify problem
  apmd <contest> <letter>       e.g. apmd abc400 a
  apmd <contest> --all          fetch all problems of a contest
  apmd <url>                    explicit URL`);
}

async function main(): Promise<void> {
  const outBaseDir = process.env.APMD_PWD || process.cwd();
  const args = process.argv.slice(2);

  // Mode: URL given directly
  if (args[0]?.startsWith("http")) {
    const url = args[0];
    const letter = url.match(/_([a-z][a-z0-9]?)$/)?.[1] ?? "problem";
    await fetchOne(url, join(outBaseDir, `${letter}.md`));
    return;
  }

  // Mode: <contest> --all
  if (args.length === 2 && args[1] === "--all") {
    const contest = args[0];
    const problems = await fetchTasksList(contest);
    if (problems.length === 0) throw new Error(`No problems found for ${contest}`);
    console.log(`${contest}: ${problems.join(", ")}`);
    for (const p of problems) {
      // If a per-problem subdir exists (TS layout), write problem.md inside it.
      // Otherwise (Rust layout), write <letter>.md at outBaseDir.
      const subdir = join(outBaseDir, p);
      const outPath = existsSync(subdir)
        ? join(subdir, "problem.md")
        : join(outBaseDir, `${p}.md`);
      await fetchOne(buildUrl(contest, p), outPath);
      await new Promise((r) => setTimeout(r, 1500));
    }
    return;
  }

  // Resolve contest + problem
  let contest: string | undefined;
  let problem: string | undefined;
  if (args.length === 0) {
    const det = autoDetect();
    if (!det || !det.problem) {
      printUsage();
      process.exit(1);
    }
    contest = det.contest;
    problem = det.problem;
  } else if (args.length === 1) {
    const det = autoDetect();
    if (!det) { printUsage(); process.exit(1); }
    contest = det.contest;
    problem = args[0];
  } else if (args.length === 2) {
    contest = args[0];
    problem = args[1];
  } else {
    printUsage();
    process.exit(1);
  }

  const url = buildUrl(contest, problem);
  // Output filename: in a single-problem dir (TS), name "problem.md".
  // Otherwise (rust or explicit contest+problem), use "<letter>.md".
  const det = autoDetect();
  const single = det?.problem === problem;
  const outPath = join(outBaseDir, single ? "problem.md" : `${problem}.md`);
  await fetchOne(url, outPath);
}

main().catch((err) => {
  console.error(err.message ?? err);
  process.exit(1);
});
