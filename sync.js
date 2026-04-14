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
    let hasMore = true;

    while (hasMore) {
        try {
            const { data } = await axios.get(`https://leetcode.com/api/submissions/?offset=${offset}&limit=50`, {
                headers: { 
                    'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`, 
                    'X-CSRFToken': CSRF_TOKEN, 
                    'Referer': 'https://leetcode.com/' 
                }
            });

            if (!data.submissions_dump || data.submissions_dump.length === 0) {
                hasMore = false;
                break;
            }

            all.push(...data.submissions_dump.filter(s => s.status_display === "Accepted"));
            offset += 50;
            await new Promise(r => setTimeout(r, 1500));
        } catch (e) {
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
        return data.data.question.content || "Description unavailable.";
    } catch {
        return "Description fetch failed.";
    }
}

async function run() {
    const raw = await fetchSubmissions();
    const store = new Map();
    
    for (const s of raw) {
        if (!store.has(s.title_slug)) store.set(s.title_slug, s);
    }

    const stats = {};
    const entries = Array.from(store.values());

    for (const s of entries) {
        const conf = LANG_CONFIG[s.lang] || { folder: s.lang.toUpperCase(), ext: 'txt' };
        const probDir = path.join(__dirname, conf.folder, sanitize(s.title));
        
        if (!fs.existsSync(probDir)) fs.mkdirSync(probDir, { recursive: true });

        const codePath = path.join(probDir, `solution.${conf.ext}`);
        const descPath = path.join(probDir, `README.md`);

        if (!fs.existsSync(codePath)) fs.writeFileSync(codePath, s.code);
        
        if (!fs.existsSync(descPath)) {
            const description = await getProblemDescription(s.title_slug);
            fs.writeFileSync(descPath, `# ${s.title}\n\n${description}`);
            await new Promise(r => setTimeout(r, 1000));
        }

        if (!stats[conf.folder]) stats[conf.folder] = [];
        stats[conf.folder].push({ title: s.title, path: `./${conf.folder}/${sanitize(s.title)}` });
    }

    let mainReadme = `# LEETCODE ARCHIVE\n\nTotal Solved: ${store.size}\n\n`;
    for (const lang in stats) {
        mainReadme += `## ${lang}\n| # | Problem |\n|---|---|\n`;
        stats[lang].forEach((p, i) => {
            mainReadme += `| ${i+1} | [${p.title}](${p.path}) |\n`;
        });
        mainReadme += `\n`;
    }
    fs.writeFileSync('README.md', mainReadme);
}

run();
