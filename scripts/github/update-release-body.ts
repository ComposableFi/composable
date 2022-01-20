import {Octokit} from "octokit";
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
            changes: {
                type: "string",
                describe: "newly updated body",
            },
            repo: {
                type: "string",
                describe: "repo for the release",
            },
            metadata: {
                type: "string",
                describe: "runtime metadata in md format",
            },
        })
        .demandOption(["id", "body", "repo", "metadata"])
        .help().argv;
    const metadata = fs.readFileSync(argv.metadata).toString();

    const body = `${metadata}\n\n${argv.body}`
    const octokit = new Octokit({
        auth: process.env.GITHUB_TOKEN || undefined,
    });

    await octokit.request("PATCH /repos/{owner}/{repo}/releases/{release_id}", {
        owner: argv.repo.split("/")[0],
        repo: argv.repo.split("/")[1],
        release_id: argv.id,
        body,
    });
}

main();
