const fs = require('fs');
const path = require('path');
const axios = require('axios');

const SESSION = process.env.LEETCODE_SESSION;
const CSRF_TOKEN = process.env.LEETCODE_CSRF_TOKEN;

const LANG_CONFIG = {
    'python': { folder: 'Python', ext: 'py' },
    'python3': { folder: 'Python', ext: 'py' },
    'cpp': { folder: 'C++', ext: 'cpp' },
    'c': { folder: 'C', ext: 'c' },
    'java': { folder: 'Java', ext: 'java' },
    'javascript': { folder: 'JavaScript', ext: 'js' },
    'typescript': { folder: 'TypeScript', ext: 'ts' },
    'csharp': { folder: 'C#', ext: 'cs' },
    'go': { folder: 'Go', ext: 'go' },
    'rust': { folder: 'Rust', ext: 'rs' },
    'kotlin': { folder: 'Kotlin', ext: 'kt' },
    'swift': { folder: 'Swift', ext: 'swift' },
    'ruby': { folder: 'Ruby', ext: 'rb' },
    'php': { folder: 'PHP', ext: 'php' },
    'mysql': { folder: 'SQL', ext: 'sql' },
    'mssql': { folder: 'SQL', ext: 'sql' },
    'oraclesql': { folder: 'SQL', ext: 'sql' },
    'bash': { folder: 'Shell', ext: 'sh' }
};

const sanitize = (str) => str.replace(/[^a-zA-Z0-9\-_ ]/g, "").trim().replace(/\s+/g, "_");

async function getSubmissions() {
    let allSubmissions = [];
    let offset = 0;
    let hasNext = true;

    while (hasNext) {
        try {
            const response = await axios.get(`https://leetcode.com/api/submissions/?offset=${offset}&limit=20`, {
                headers: {
                    'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`,
                    'X-CSRFToken': CSRF_TOKEN,
                    'Referer': 'https://leetcode.com/'
                }
            });

            const data = response.data;
            const accepted = (data.submissions_dump || []).filter(s => s.status_display === "Accepted");
            allSubmissions.push(...accepted);
            
            hasNext = data.has_next;
            offset += 20;
            await new Promise(r => setTimeout(r, 1500));
        } catch (e) {
            break;
        }
    }
    return allSubmissions;
}

async function main() {
    const submissions = await getSubmissions();
    const uniqueSolutions = new Map();

    for (const s of submissions) {
        const id = `${s.title_slug}_${s.lang}`;
        if (!uniqueSolutions.has(id)) uniqueSolutions.set(id, s);
    }

    const stats = {};

    for (const [key, s] of uniqueSolutions) {
        const config = LANG_CONFIG[s.lang] || { folder: s.lang.toUpperCase(), ext: 'txt' };
        const dir = path.join(__dirname, config.folder);
        
        if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });

        const filename = `${sanitize(s.title)}.${config.ext}`;
        const filePath = path.join(dir, filename);

        if (!fs.existsSync(filePath)) {
            fs.writeFileSync(filePath, s.code);
        }

        if (!stats[config.folder]) stats[config.folder] = [];
        stats[config.folder].push({ title: s.title, slug: s.title_slug, file: filename });
    }

    let readme = `# LeetCode V-Vault\n\n`;
    readme += `![Solved](https://img.shields.io/badge/Solved-${uniqueSolutions.size}-00ff88?style=for-the-badge) `;
    readme += `![Languages](https://img.shields.io/badge/Languages-${Object.keys(stats).length}-00ccff?style=for-the-badge)\n\n---\n`;

    for (const lang in stats) {
        readme += `\n### ${lang}\n`;
        readme += `| # | Problem | Solution |\n|---|---|---|\n`;
        stats[lang].forEach((prob, i) => {
            readme += `| ${i + 1} | [${prob.title}](https://leetcode.com/problems/${prob.slug}/) | [View](./${lang}/${prob.file}) |\n`;
        });
    }

    fs.writeFileSync('README.md', readme);
}

main();
