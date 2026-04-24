const fs = require('fs');
const path = require('path');
const axios = require('axios');

const BLACK_SESSION = process.env.LEETCODE_SESSION;
const BLACK_CSRF = process.env.LEETCODE_CSRF_TOKEN;

const BLACK_HEADERS = {
    'Cookie': `LEETCODE_SESSION=${BLACK_SESSION}; csrftoken=${BLACK_CSRF};`,
    'X-CSRFToken': BLACK_CSRF,
    'Content-Type': 'application/json',
    'Referer': 'https://leetcode.com/',
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36'
};

const BLACK_LANG_CONFIG = {
    'rust':       { folder: 'Rust',       ext: 'rs'   },
    'python':     { folder: 'Python',     ext: 'py'   },
    'python3':    { folder: 'Python',     ext: 'py'   },
    'cpp':        { folder: 'C++',        ext: 'cpp'  },
    'c':          { folder: 'C',          ext: 'c'    },
    'java':       { folder: 'Java',       ext: 'java' },
    'javascript': { folder: 'JavaScript', ext: 'js'   },
    'typescript': { folder: 'TypeScript', ext: 'ts'   },
    'mysql':      { folder: 'SQL',        ext: 'sql'  },
    'pandas':     { folder: 'PYTHONDATA', ext: 'py'   },
    'bash':       { folder: 'BASH',       ext: 'sh'   },
    'golang':     { folder: 'GOLANG',     ext: 'go'   }
};

const BLACK_OVERFLOW_LIMIT = 1000;
const BLACK_USERNAME = 'BBLACK0X80';

const blackSleep = (ms) => new Promise(r => setTimeout(r, ms));
const blackSanitize = (s) => s.replace(/[^a-zA-Z0-9 ]/g, '').trim().replace(/\s+/g, '_');

async function blackGql(blackQuery, blackVars = {}, blackRetries = 5) {
    for (let blackAttempt = 0; blackAttempt < blackRetries; blackAttempt++) {
        try {
            const { data: blackData } = await axios.post(
                'https://leetcode.com/graphql',
                { query: blackQuery, variables: blackVars },
                { headers: BLACK_HEADERS, timeout: 20000 }
            );
            return blackData?.data || null;
        } catch {
            await blackSleep(3000 * (blackAttempt + 1));
        }
    }
    return null;
}

async function blackFetchProfile() {
    const blackData = await blackGql(`
        query blackGetProfile($username: String!) {
            matchedUser(username: $username) {
                profile {
                    ranking
                }
                submitStatsGlobal {
                    acSubmissionNum {
                        difficulty
                        count
                    }
                }
            }
        }
    `, { username: BLACK_USERNAME });

    const blackUser = blackData?.matchedUser;
    if (!blackUser) return null;

    const blackRank = blackUser.profile?.ranking ?? null;
    const blackAcStats = blackUser.submitStatsGlobal?.acSubmissionNum ?? [];
    const blackTotal = blackAcStats.find(s => s.difficulty === 'All')?.count ?? 0;

    return { blackRank, blackTotal };
}

function blackUpdateReadme(blackRank, blackTotal) {
    const blackReadmePath = path.join(__dirname, 'README.md');
    if (!fs.existsSync(blackReadmePath)) {
        console.log('README.md not found, skipping update.');
        return;
    }

    let blackContent = fs.readFileSync(blackReadmePath, 'utf8');
    const blackOriginal = blackContent;

    const blackRankStr   = blackRank.toLocaleString('en-US');
    const blackTotalStr  = blackTotal.toLocaleString('en-US');
    const blackRankEnc   = blackRankStr.replace(/,/g, '%2C');
    const blackTotalEnc  = blackTotalStr.replace(/,/g, '%2C');

    // badge: Global_Rank-%239%2C645-
    blackContent = blackContent.replace(
        /(Global_Rank-%23)[\d%2C]+(-)/g,
        `$1${blackRankEnc}$2`
    );

    // badge: Solved-1%2C363%2F  or  Solved-1%2C363/
    blackContent = blackContent.replace(
        /(Solved-)[\d%2C]+(%2F|\/)/g,
        `$1${blackTotalEnc}$2`
    );

    // typing svg: lines=1%2C363+Problems+Solved+%7C+Global+Rank+%239%2C645
    blackContent = blackContent.replace(
        /(lines=)[\d%2C]+(\+Problems[^;]*%23)[\d%2C]+/g,
        `$1${blackTotalEnc}$2${blackRankEnc}`
    );

    // stats table: | Global Rank | **#9,645** |
    blackContent = blackContent.replace(
        /(\| Global Rank \| )\*\*#[\d,]+\*\*/g,
        `$1**#${blackRankStr}**`
    );

    // stats table: | Problems Solved | **1,363 / 3,907** |
    blackContent = blackContent.replace(
        /(\| Problems Solved \| )\*\*[\d,]+ \/ [\d,]+\*\*/g,
        `$1**${blackTotalStr} / 3,907**`
    );

    // global standing table: | **#9,645** | Top ...
    blackContent = blackContent.replace(
        /(\| \*\*#)[\d,]+(\*\* \| Top)/g,
        `$1${blackRankStr}$2`
    );

    // footer: #9,645 worldwide
    blackContent = blackContent.replace(
        /(#)[\d,]+( worldwide)/g,
        `$1${blackRankStr}$2`
    );

    if (blackContent === blackOriginal) {
        console.log('README.md: nothing changed (rank/solved patterns not found).');
    } else {
        fs.writeFileSync(blackReadmePath, blackContent, 'utf8');
        console.log(`README.md updated -> Rank #${blackRankStr} | Solved ${blackTotalStr}`);
    }
}

async function blackFetchAllAccepted() {
    const blackAccepted = new Map();
    let blackLastKey = null;
    let blackOffset = 0;
    let blackHasNext = true;

    while (blackHasNext) {
        const blackData = await blackGql(`
            query blackSubmissionList($offset: Int!, $limit: Int!, $lastKey: String) {
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
        `, { offset: blackOffset, limit: 20, lastKey: blackLastKey });

        const blackList = blackData?.submissionList;
        if (!blackList) { await blackSleep(5000); continue; }

        const blackSubmissions = blackList.submissions ?? [];

        for (const blackSub of blackSubmissions) {
            if (blackSub.statusDisplay === 'Accepted' && !blackAccepted.has(blackSub.titleSlug)) {
                blackAccepted.set(blackSub.titleSlug, blackSub);
            }
        }

        blackHasNext = blackList.hasNext ?? false;
        blackLastKey = blackList.lastKey ?? null;
        blackOffset += 20;

        process.stdout.write(`\rFetched ${blackAccepted.size} unique accepted...`);
        await blackSleep(350);
    }

    console.log(`\nTotal: ${blackAccepted.size}`);
    return blackAccepted;
}

async function blackFetchCode(blackSubmissionId) {
    const blackData = await blackGql(`
        query blackSubmissionDetails($submissionId: Int!) {
            submissionDetails(submissionId: $submissionId) {
                code
            }
        }
    `, { submissionId: parseInt(blackSubmissionId) });
    return blackData?.submissionDetails?.code || null;
}

async function blackFetchQuestion(blackSlug) {
    const blackData = await blackGql(`
        query blackQuestionData($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                title
                difficulty
                content
                topicTags { name }
                hints
                exampleTestcaseList
            }
        }
    `, { titleSlug: blackSlug });
    return blackData?.question || null;
}

function blackBuildReadme(blackQuestion, blackCode, blackLang) {
    const blackTags = (blackQuestion.topicTags || []).map(t => t.name).join(', ');
    const blackHints = (blackQuestion.hints || []).map((h, i) => `${i + 1}. ${h}`).join('\n');

    return [
        `# ${blackQuestion.title}`,
        ``,
        `**Difficulty:** ${blackQuestion.difficulty}`,
        `**Tags:** ${blackTags}`,
        ``,
        `---`,
        ``,
        `## Problem`,
        ``,
        blackQuestion.content || '',
        ``,
        blackHints ? `## Hints\n\n${blackHints}\n` : '',
        `## Solution`,
        ``,
        `\`\`\`${blackLang}`,
        blackCode || '',
        `\`\`\``,
    ].filter(l => l !== null).join('\n');
}

function blackGetFolder(blackBaseFolder, blackCount) {
    if (blackCount <= BLACK_OVERFLOW_LIMIT) return blackBaseFolder;
    const blackPart = Math.ceil(blackCount / BLACK_OVERFLOW_LIMIT);
    return `${blackBaseFolder}_${blackPart}`;
}

function blackVerifyFile(blackFilePath) {
    if (!fs.existsSync(blackFilePath)) return false;
    return fs.readFileSync(blackFilePath, 'utf8').trim().length > 0;
}

function blackIsDuplicate(blackSlug, blackActiveFolder) {
    const blackSafeTitle = blackSanitize(blackSlug.replace(/-/g, ' '));
    const blackProbDir = path.join(__dirname, blackActiveFolder, blackSafeTitle);
    return Object.values(BLACK_LANG_CONFIG).some(c =>
        blackVerifyFile(path.join(blackProbDir, `solution.${c.ext}`))
    );
}

async function blackRun() {
    const blackProfile = await blackFetchProfile();
    if (blackProfile) {
        console.log(`Profile -> Rank #${blackProfile.blackRank} | Solved ${blackProfile.blackTotal}`);
        blackUpdateReadme(blackProfile.blackRank, blackProfile.blackTotal);
    } else {
        console.log('Could not fetch profile, README not updated.');
    }

    const blackAccepted = await blackFetchAllAccepted();
    const blackLangCounters = {};
    let blackProcessed = 0;
    const blackTotalProblems = blackAccepted.size;

    for (const [blackSlug, blackSub] of blackAccepted) {
        blackProcessed++;
        const blackConf = BLACK_LANG_CONFIG[blackSub.lang] || { folder: blackSub.lang.toUpperCase(), ext: 'txt' };
        const blackBaseFolder = blackConf.folder;

        if (!blackLangCounters[blackBaseFolder]) blackLangCounters[blackBaseFolder] = 0;
        blackLangCounters[blackBaseFolder]++;

        const blackActiveFolder = blackGetFolder(blackBaseFolder, blackLangCounters[blackBaseFolder]);
        const blackSafeTitle = blackSanitize(blackSub.title);
        const blackProbDir = path.join(__dirname, blackActiveFolder, blackSafeTitle);
        const blackCodePath = path.join(blackProbDir, `solution.${blackConf.ext}`);
        const blackDescPath = path.join(blackProbDir, 'README.md');

        const blackCodeExists = blackVerifyFile(blackCodePath);
        const blackDescExists = blackVerifyFile(blackDescPath);

        if (blackCodeExists && blackDescExists) {
            console.log(`[${blackProcessed}/${blackTotalProblems}] SKIP  ${blackSub.title}`);
            continue;
        }

        if (blackIsDuplicate(blackSlug, blackActiveFolder) && blackCodeExists) {
            console.log(`[${blackProcessed}/${blackTotalProblems}] DUPE  ${blackSub.title}`);
            continue;
        }

        fs.mkdirSync(blackProbDir, { recursive: true });

        const [blackCode, blackQuestion] = await Promise.all([
            blackCodeExists ? Promise.resolve(null) : blackFetchCode(blackSub.id),
            blackDescExists ? Promise.resolve(null) : blackFetchQuestion(blackSlug)
        ]);

        await blackSleep(400);

        if (blackCode && !blackVerifyFile(blackCodePath)) {
            fs.writeFileSync(blackCodePath, blackCode, 'utf8');
        }

        if (blackQuestion && !blackVerifyFile(blackDescPath)) {
            const blackExistingCode = blackCode ||
                (blackVerifyFile(blackCodePath) ? fs.readFileSync(blackCodePath, 'utf8') : '');
            const blackMd = blackBuildReadme(blackQuestion, blackExistingCode, blackSub.lang);
            fs.writeFileSync(blackDescPath, blackMd, 'utf8');
        }

        const blackWriteOk = blackVerifyFile(blackCodePath) && blackVerifyFile(blackDescPath);
        console.log(
            `[${blackProcessed}/${blackTotalProblems}] ${blackWriteOk ? 'DONE' : 'WARN'}  ${blackSub.title} (${blackActiveFolder})`
        );
    }

    console.log('Sync complete.');
}

blackRun().catch(console.error);
