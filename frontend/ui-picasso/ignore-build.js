const childProcess = require("child_process");
const path = require("path");
const ABORT = 0;
const CONTINUE = 1;
const BASE_PATH = "frontend";
const PREFIX = `picasso-`;
const PROTECTED_BRANCHES = ["main"];

function abort(message) {
  message && console.error(message);
  process.exit(ABORT);
}
function resume() {
  process.exit(CONTINUE);
}

function getCurrentGitBranch() {
  return childProcess
    .execSync("git rev-parse --abbrev-ref HEAD")
    .toString()
    .trim();
}

function getCurrentGitTag() {
  return childProcess
    .execSync("git describe --tags --exact-match")
    .toString()
    .trim();
}

const PROJECT = process.argv[2] || path.basename(path.resolve());

function main() {
  if (!PROJECT) abort(`No project specified.`);
  const fileNameList = childProcess
    .execSync("git diff --name-only HEAD~10")
    .toString()
    .trim()
    .split("\n");
  const hasProjectSpecificChanges = fileNameList.some((file) =>
    file.startsWith(`${BASE_PATH}/${PROJECT}`)
  );

  !hasProjectSpecificChanges &&
    abort(`No changes detected specific to ${BASE_PATH}/${PROJECT}`);
  const currentBranch = getCurrentGitBranch();
  try {
    const currentTag = getCurrentGitTag();
    if (PROTECTED_BRANCHES.includes(currentBranch)) {
      if (!currentTag.startsWith(PREFIX))
        abort(
          `Branch ${currentBranch} is protected, it requires a specific tag starting with ${PREFIX} to build`
        );
      resume();
    } else {
      abort();
    }
  } catch (e) {
    abort();
  }
}

main();
