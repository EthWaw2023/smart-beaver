import fs from "fs";

import readline from "readline";

let lineCount = 0;

// Replace all occurrences of "lorem" with "aaa" in a text file line by line
function replaceLoremWithAaaInFile(filePath) {
  const outputFilePath = "output.txt"; // Create a new file for the modified content

  const readStream = fs.createReadStream(filePath, "utf8");
  const writeStream = fs.createWriteStream(outputFilePath, "utf8");

  const rl = readline.createInterface({
    input: readStream,
    output: writeStream,
    terminal: false,
  });

  rl.on("line", (line) => {
    // console.log(line);
    let replacedLine = line.replace("Lorem", "aaa");
    replacedLine = "2137";
    lineCount += 1;
    writeStream.write(replacedLine + "\n");
  });

  rl.on("close", () => {
    console.log(
      `Replaced "lorem" with "aaa" in ${filePath} and saved to ${outputFilePath}`,
    );
    console.log("Lines iteraged: ", lineCount);
    console.timeEnd("test");
  });
}

// Replace in your text file by providing its path
const filePath = "./test.txt";
console.time("test");
replaceLoremWithAaaInFile(filePath);
