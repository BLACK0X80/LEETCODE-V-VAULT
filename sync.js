const fs = require('fs');
const path = require('path');
const axios = require('axios');

const SESSION = process.env.LEETCODE_SESSION;
const CSRF_TOKEN = process.env.LEETCODE_CSRF_TOKEN;

const LANG_CONFIG = {
    'rust': { folder: 'Rust', ext: 'rs' },
    'python': { folder: 'Python', ext: 'py' },
    'python3': { folder: 'Python', ext: 'py' },
    'cpp': { folder: 'C++', ext: 'cpp' },
    'c': { folder: 'C', ext: 'c' },
    'java': { folder: 'Java', ext: 'java' },
    'javascript': { folder: 'JavaScript', ext: 'js' },
    'typescript': { folder: 'TypeScript', ext: 'ts' },
    'mysql': { folder: 'SQL', ext: 'sql' },
    'pandas': { folder: 'Pandas', ext: 'py' }
};

const sanitize = (s) => s.replace(/[^a-zA-Z0-9 ]/g, "").trim().replace(/\s+/g, "_");

async function fetchSubmissions() {
    let all = [];
    let offset = 0;
    while (true) {
        try {
            const { data } = await axios.get(`https://leetcode.com/api/submissions/?offset=${offset}&limit=100`, {
                headers: { 'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`, 'X-CSRFToken': CSRF_TOKEN, 'Referer': 'https://leetcode.com/' }
            });
            if (!data.submissions_dump || data.submissions_dump.length === 0) break;
            all.push(...data.submissions_dump.filter(s => s.status_display === "Accepted"));
            offset += 100;
            await new Promise(r => setTimeout(r, 1000));
        } catch {
            await new Promise(r => setTimeout(r, 5000));
        }
    }
    return all;
}

async function getProblemDescription(slug) {
    try {
        const { data } = await axios.post('https://leetcode.com/graphql', {
            query: `query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { content } }`,
            variables: { titleSlug: slug }
        }, {
            headers: { 'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`, 'X-CSRFToken': CSRF_TOKEN }
        });
        return data.data.question.content || "";
    } catch { return ""; }
}

async function run() {
    Object.values(LANG_CONFIG).forEach(conf => {
        if (fs.existsSync(conf.folder)) fs.rmSync(conf.folder, { recursive: true, force: true });
    });

    const raw = await fetchSubmissions();
    const store = new Map();
    for (const s of raw) {
        if (!store.has(s.title_slug)) store.set(s.title_slug, s);
    }

    const stats = {};
    for (const s of Array.from(store.values())) {
        const conf = LANG_CONFIG[s.lang] || { folder: s.lang.toUpperCase(), ext: 'txt' };
        const probDir = path.join(__dirname, conf.folder, sanitize(s.title));
        if (!fs.existsSync(probDir)) fs.mkdirSync(probDir, { recursive: true });

        fs.writeFileSync(path.join(probDir, `solution.${conf.ext}`), s.code);
        const desc = await getProblemDescription(s.title_slug);
        if (desc) fs.writeFileSync(path.join(probDir, `README.md`), `# ${s.title}\n\n${desc}`);
        await new Promise(r => setTimeout(r, 150));

        if (!stats[conf.folder]) stats[conf.folder] = [];
        stats[conf.folder].push({ title: s.title, path: `./${conf.folder}/${sanitize(s.title)}` });
    }

    let mainReadme = `# LEETCODE ARCHIVE\n\nTotal Solved: ${store.size}\n\n`;
    for (const lang in stats) {
        mainReadme += `## ${lang}\n| # | Problem |\n|---|---|\n`;
        stats[lang].forEach((p, i) => { mainReadme += `| ${i+1} | [${p.title}](${p.path}) |\n`; });
        mainReadme += `\n`;
    }
    fs.writeFileSync('README.md', mainReadme);
}
run();
