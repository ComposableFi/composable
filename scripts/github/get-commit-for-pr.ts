import {Octokit} from "octokit";
import * as core from '@actions/core';
import yargs from "yargs";

async function main() {
    const argv = yargs(process.argv.slice(2))
        .usage("Usage: npm run update-release-body [args]")
        .version("1.0.0")
        .options({
            repo: {
                type: "string",
                describe: "newly updated body",
            },
            commit_sha: {
                type: "string",
                describe: "commit hash for the pull request to find",
            },
        })
        .demandOption(["commit_sha", "repo"])
        .help().argv;

    const octokit = new Octokit({
        auth: process.env.GITHUB_TOKEN || undefined,
    });

    const data = await octokit.request(
        "GET /repos/{owner}/{repo}/commits/{commit_sha}/pulls",
        {
            owner: argv.repo.split("/")[0],
            repo: argv.repo.split("/")[1],
            commit_sha: argv.commit_sha,
        }
    )

    if (data.data.length > 0) {
        if (data.data[0].labels.some(label => label.name === "needs-backport")) {
            core.setOutput("cherry_pick", 1)
        }
    }
}

main();
