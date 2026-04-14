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
    'csharp': { folder: 'C#', ext: 'cs' },
    'go': { folder: 'Go', ext: 'go' },
    'kotlin': { folder: 'Kotlin', ext: 'kt' },
    'swift': { folder: 'Swift', ext: 'swift' },
    'ruby': { folder: 'Ruby', ext: 'rb' },
    'php': { folder: 'PHP', ext: 'php' },
    'mysql': { folder: 'SQL', ext: 'sql' },
    'mssql': { folder: 'SQL', ext: 'sql' },
    'oraclesql': { folder: 'SQL', ext: 'sql' },
    'pandas': { folder: 'Pandas', ext: 'py' },
    'bash': { folder: 'Shell', ext: 'sh' }
};

const sanitize = (s) => s.replace(/[^a-zA-Z0-9 ]/g, "").trim().replace(/\s+/g, "_");

async function fetchSubmissions() {
    let all = [];
    let offset = 0;
    let hasNext = true;

    while (hasNext) {
        try {
            const { data } = await axios.get(`https://leetcode.com/api/submissions/?offset=${offset}&limit=20`, {
                headers: {
                    'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`,
                    'X-CSRFToken': CSRF_TOKEN,
                    'Referer': 'https://leetcode.com/'
                }
            });

            const accepted = (data.submissions_dump || []).filter(s => s.status_display === "Accepted");
            all.push(...accepted);
            hasNext = data.has_next;
            offset += 20;
            await new Promise(r => setTimeout(r, 1200));
        } catch (e) {
            break;
        }
    }
    return all;
}

async function run() {
    const raw = await fetchSubmissions();
    const store = new Map();

    for (const s of raw) {
        const key = `${s.title_slug}_${s.lang}`;
        if (!store.has(key)) store.set(key, s);
    }

    const stats = {};

    for (const [_, s] of store) {
        const conf = LANG_CONFIG[s.lang] || { folder: s.lang.toUpperCase(), ext: 'txt' };
        const dir = path.join(__dirname, conf.folder);
        
        if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

        const filename = `${sanitize(s.title)}.${conf.ext}`;
        const file = path.join(dir, filename);

        if (!fs.existsSync(file)) {
            fs.writeFileSync(file, s.code);
        }

        if (!stats[conf.folder]) stats[conf.folder] = [];
        stats[conf.folder].push({ title: s.title, slug: s.title_slug, file: filename });
    }

    let readme = `# LEETCODE ARCHIVE | BLACK0X80\n\n`;
    readme += `![Solutions](https://img.shields.io/badge/Solutions-${store.size}-00ff88?style=for-the-badge) `;
    readme += `![Hard](https://img.shields.io/badge/Level-Hard_Focused-red?style=for-the-badge)\n\n---\n\n`;
    
    for (const lang in stats) {
        readme += `### ${lang} (${stats[lang].length})\n`;
        readme += `| # | Problem | Source |\n|---|---|---|\n`;
        stats[lang].forEach((p, i) => {
            readme += `| ${i+1} | [${p.title}](https://leetcode.com/problems/${p.slug}/) | [View](./${lang}/${p.file}) |\n`;
        });
        readme += `\n`;
    }

    fs.writeFileSync('README.md', readme);
}

run();
