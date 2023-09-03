const {exec} = require("child_process");
const fs = require('fs');


const BUILD_PATH = "./contracts/target/ink/psp22.contract"

const handleBuild = (features) => {

    let cmd = "cd contracts && RUSTFLAGS=-Awarnings cargo contract build --quiet --output-json ";
    if (features && features.length > 0) {
        cmd += "--features " + features;
    }

    console.debug("CMD: ", cmd)

    const exec = require('child_process').exec;
    return new Promise((resolve, reject) => {
        exec(cmd, (error, stdout, stderr) => {
            if (error) {
                console.warn(error);
            }
            resolve({
                data: !error ? JSON.parse(stdout) : stderr,
                success: !Boolean(error)
            });
        });
    });

}

const loadBundle = (filePath) => {
    try {
        return fs.readFileSync(filePath || BUILD_PATH, {encoding: 'base64'});
    } catch (err) {
        return null;
    }
}

exports.handleBuild = handleBuild;

exports.loadBundle = loadBundle;