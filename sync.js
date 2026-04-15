const fs = require('fs');
const path = require('path');
const axios = require('axios');

const SESSION = process.env.LEETCODE_SESSION;
const CSRF_TOKEN = process.env.LEETCODE_CSRF_TOKEN;

const HEADERS = {
    'Cookie': `LEETCODE_SESSION=${SESSION}; csrftoken=${CSRF_TOKEN};`,
    'X-CSRFToken': CSRF_TOKEN,
    'Content-Type': 'application/json',
    'Referer': 'https://leetcode.com/',
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36'
};

const LANG_CONFIG = {
    'rust':       { folder: 'Rust',       ext: 'rs'   },
    'python':     { folder: 'Python',     ext: 'py'   },
    'python3':    { folder: 'Python',     ext: 'py'   },
    'cpp':        { folder: 'C++',        ext: 'cpp'  },
    'c':          { folder: 'C',          ext: 'c'    },
    'java':       { folder: 'Java',       ext: 'java' },
    'javascript': { folder: 'JavaScript', ext: 'js'   },
    'typescript': { folder: 'TypeScript', ext: 'ts'   },
    'mysql':      { folder: 'SQL',        ext: 'sql'  },
    'pandas':     { folder: 'PYTHONDATA', ext: 'py'   }
};

const sleep = (ms) => new Promise(r => setTimeout(r, ms));
const sanitize = (s) => s.replace(/[^a-zA-Z0-9 ]/g, '').trim().replace(/\s+/g, '_');

async function gql(query, variables = {}, retries = 5) {
    for (let i = 0; i < retries; i++) {
        try {
            const { data } = await axios.post(
                'https://leetcode.com/graphql',
                { query, variables },
                { headers: HEADERS, timeout: 20000 }
            );
            return data?.data || null;
        } catch {
            await sleep(3000 * (i + 1));
        }
    }
    return null;
}

async function fetchAllAccepted() {
    const accepted = new Map();
    let lastKey = null;
    let offset = 0;
    let hasNext = true;

    while (hasNext) {
        const data = await gql(`
            query submissionList($offset: Int!, $limit: Int!, $lastKey: String) {
                submissionList(offset: $offset, limit: $limit, lastKey: $lastKey) {
                    lastKey
                    hasNext
                    submissions {
                        id
                        title
                        titleSlug
                        statusDisplay
                        lang
                        timestamp
                    }
                }
            }
        `, { offset, limit: 20, lastKey });

        const list = data?.submissionList;
        if (!list) { await sleep(5000); continue; }

        for (const s of list.submissions) {
            if (s.statusDisplay === 'Accepted' && !accepted.has(s.titleSlug)) {
                accepted.set(s.titleSlug, s);
            }
        }

        hasNext = list.hasNext;
        lastKey = list.lastKey;
        offset += 20;

        process.stdout.write(`\rFetched ${accepted.size} unique accepted...`);
        await sleep(350);
    }

    console.log(`\nTotal: ${accepted.size}`);
    return accepted;
}

async function fetchCode(submissionId) {
    const data = await gql(`
        query submissionDetails($submissionId: Int!) {
            submissionDetails(submissionId: $submissionId) {
                code
            }
        }
    `, { submissionId: parseInt(submissionId) });
    return data?.submissionDetails?.code || null;
}

async function fetchQuestion(slug) {
    const data = await gql(`
        query questionData($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                title
                difficulty
                content
                topicTags { name }
                hints
                exampleTestcaseList
            }
        }
    `, { titleSlug: slug });
    return data?.question || null;
}

function buildReadme(q, code, lang, ext) {
    const tags = (q.topicTags || []).map(t => t.name).join(', ');
    const hints = (q.hints || []).map((h, i) => `${i + 1}. ${h}`).join('\n');

    return [
        `# ${q.title}`,
        ``,
        `**Difficulty:** ${q.difficulty}`,
        `**Tags:** ${tags}`,
        ``,
        `---`,
        ``,
        `## Problem`,
        ``,
        q.content || '',
        ``,
        hints ? `## Hints\n\n${hints}\n` : '',
        `## Solution`,
        ``,
        `\`\`\`${lang}`,
        code || '',
        `\`\`\``,
    ].filter(l => l !== null).join('\n');
}

async function run() {
    const accepted = await fetchAllAccepted();
    const stats = {};
    let processed = 0;
    const total = accepted.size;

    for (const [slug, sub] of accepted) {
        processed++;
        const conf = LANG_CONFIG[sub.lang] || { folder: sub.lang.toUpperCase(), ext: 'txt' };
        const safeTitle = sanitize(sub.title);
        const probDir = path.join(__dirname, conf.folder, safeTitle);
        const codePath = path.join(probDir, `solution.${conf.ext}`);
        const descPath = path.join(probDir, 'README.md');

        if (!stats[conf.folder]) stats[conf.folder] = [];
        stats[conf.folder].push({ title: sub.title, path: `./${conf.folder}/${safeTitle}` });

        if (fs.existsSync(codePath) && fs.existsSync(descPath)) {
            console.log(`[${processed}/${total}] SKIP  ${sub.title}`);
            continue;
        }

        fs.mkdirSync(probDir, { recursive: true });

        const [code, question] = await Promise.all([
            fs.existsSync(codePath) ? Promise.resolve(null) : fetchCode(sub.id),
            fs.existsSync(descPath) ? Promise.resolve(null) : fetchQuestion(slug)
        ]);

        await sleep(400);

        if (code && !fs.existsSync(codePath)) {
            fs.writeFileSync(codePath, code, 'utf8');
        }

        if (question && !fs.existsSync(descPath)) {
            const existingCode = code || (fs.existsSync(codePath) ? fs.readFileSync(codePath, 'utf8') : '');
            const md = buildReadme(question, existingCode, sub.lang, conf.ext);
            fs.writeFileSync(descPath, md, 'utf8');
        }

        console.log(`[${processed}/${total}] DONE  ${sub.title} (${conf.folder})`);
    }

    let readme = `# LEETCODE ARCHIVE\n\nTotal Solved: ${total}\n\n`;
    for (const lang of Object.keys(stats).sort()) {
        readme += `## ${lang}\n| # | Problem |\n|---|---|\n`;
        stats[lang].forEach((p, i) => {
            readme += `| ${i + 1} | [${p.title}](${p.path}) |\n`;
        });
        readme += '\n';
    }

    fs.writeFileSync('README.md', readme, 'utf8');
    console.log('README.md written.');
}

run().catch(console.error);
