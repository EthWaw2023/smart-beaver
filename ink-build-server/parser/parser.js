const events = require('events');
const fs = require('fs');
const readline = require('readline');
const START_FEATURE_BLOCK_STRING = '/*%&&';
const END_FEATURE_BLOCK_STRING = '&&%*/';
const MACRO_STRING = '#[cfg(feature';




async function processLineByLine(features) {
    const linesArray = [];
    try {
        const rl = readline.createInterface({
                                                input: fs.createReadStream('contracts/psp22.rs'),
                                                crlfDelay: Infinity
                                            });
        let isSkippingFeature = false;
        let nesting = 0;

        for await (const line of rl) {
            const startRegex = new RegExp(START_FEATURE_BLOCK_STRING, "i");
            const endRegex = new RegExp(END_FEATURE_BLOCK_STRING, "i");
            if (!isSkippingFeature) {
                if (line.includes(START_FEATURE_BLOCK_STRING)) {
                    isSkippingFeature = await shouldSkipFeature(line, features)
                }

                if (!isSkippingFeature) {
                    if(!line.includes(START_FEATURE_BLOCK_STRING)
                    && !line.includes(END_FEATURE_BLOCK_STRING)
                    && !line.includes(MACRO_STRING)){
                        console.log(line)
                        linesArray.push(line);
                    }

                } else {
                    nesting++;
                }

            } else {
                if (line.includes(START_FEATURE_BLOCK_STRING)) {
                    nesting++;
                }
                if (line.includes(END_FEATURE_BLOCK_STRING)) {
                    nesting--;
                    if (nesting === 0) {
                        isSkippingFeature = false;
                    }
                }
            }

        }

        return linesArray;
    } catch (err) {
        console.error(err);
    }
}

async function shouldSkipFeature(line, features) {
    let result = true;
    features.forEach(feature => {
        if(line.includes(feature)) {
            result = false;
        }
    })
    return result;
}
async function parser(standard, features){
    const linesArray = await processLineByLine(features);
    return linesArray.join('\n');
}

exports.parser = parser;
