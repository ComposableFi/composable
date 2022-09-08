const fs = require('fs');
const path = require('path');

// Import public/manifest.json file and check if it has changed. 
// If it has not changed, throw an error.
const manifestPath = path.resolve(__dirname, '../public/manifest.json');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const FgRed = "\x1b[31m";
const Reset = "\x1b[0m";

console.log("Installation complete.\n Checking for changes in manifest.json...");
if (manifest.name === 'PWA App') {
    console.log(FgRed, "The manifest.json file has not changed. Please update the public/manifest.json file before deploying the project.", Reset);
}

console.log("Please edit _document.tsx to update PWA values before deployment.");
