var express = require('express');
const {parser} = require("../parser/parser");
const {handleBuild, loadBundle} = require("../builder/builderWraper");
var router = express.Router();


router.get('/', function (req, res, next) {
    res.json({});
});

// Endpoint to encode data to base64
router.post('/generate', async (req, res) => {
    const {features, standard} = req.body;

    if (!features || !standard) {
        return res.status(400).json({error: 'Both features and standard are required fields.'});
    }


    const code = await parser(standard, features);
    const encodedData = Buffer.from(code).toString('base64');

    res.json({encodedData});
});

router.post('/build', async (req, res) => {
    const {features, standard} = req.body;
    if (!features || !standard) {
        return res.status(400).json({error: 'Both features and standard are required fields.'});
    }

    const cmdResult = await handleBuild(features);

    res.json({
        result: cmdResult,
        contractBundle: loadBundle(cmdResult.data?.metadata_result?.dest_bundle)
    });
})

module.exports = router;
