var fs = require("fs");

function readFiles(dirname) {
  const LICENSE = "// SPDX-License-Identifier: MIT\n";
  fs.readdir(dirname, function (err, filenames) {
    if (err) {
      console.error(err);
      return;
    }
    filenames.forEach(function (filename) {
      fs.readFile(dirname + filename, "utf-8", function (err, content) {
        if (err) {
          console.error(err);
          return;
        }
        if (!content.startsWith(LICENSE)) {
          const newContent = LICENSE + content;
          fs.writeFile(dirname + filename, newContent, "utf8", function (err) {
            if (err) {
              console.error(err);
              return;
            }
          });
        }
      });
    });
  });
}

readFiles("./contracts-merged/");
