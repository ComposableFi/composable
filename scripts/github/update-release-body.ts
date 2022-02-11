import { Octokit } from "octokit";
import yargs from "yargs";
import * as fs from "fs";

async function main() {
    const argv = yargs(process.argv.slice(2))
        .usage("Usage: npm run update-release-body [args]")
        .version("1.0.0")
        .options({
            id: {
                type: "string",
                describe: "id of the release to update",
            },
            body: {
                type: "string",
                describe: "newly updated body",
            },
            repo: {
                type: "string",
                describe: "repo name e.g org/repo",
            },
        })
        .demandOption(["id", "body", "repo"])
        .help().argv;

    const octokit = new Octokit({
        auth: process.env.GITHUB_TOKEN || undefined,
    });

    const body = fs.readFileSync(argv.body).toString()

    await octokit.request("PATCH /repos/{owner}/{repo}/releases/{release_id}", {
        owner: argv.repo.split("/")[0],
        repo: argv.repo.split("/")[1],
        release_id: argv.id,
        body,
    });
}

main();
