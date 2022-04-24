// run like `npm run cors -- set` or `npm run cors -- get`
const { execSync } = require("child_process");
const path = require("path");

const bucket = `dakom-jigsaw-puzzle`;
const configPath = path.resolve(`./storage-cors.json`);
const action = process.argv[2];

if (action === "set") {
    execSync(`gsutil cors set ${configPath} gs://${bucket}`, {
        stdio: [0, 1, 2],
    });
} else if(action === "get") {
    execSync(`gsutil cors get gs://${bucket}`, { stdio: [0, 1, 2] });
} else {
    console.error("unknown action!");
}
