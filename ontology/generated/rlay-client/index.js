
    const { Client } = require('@rlay/rlay-client-lib');
    const map = new Map();

    const getClient = (config) => {
      const stringConfig = JSON.stringify(config);
      if (map.has(stringConfig)) {
        return map.get(stringConfig);
      } else {
        const client = new Client(config);
        const schemaCIDs = {"urlLabel":"0x019580031b20428df13c43218f450d449c62438a7a491ab1c0eb87ab16391ebe31ec8592e03d","alternativeUrlLabel":"0x019580031b207601ac7214fb3a19e6c8581a3de0dac73d7443b52e81e66fd8075c1e67d9a6f1","Sha256ChecksumLabel":"0x019580031b20eae6866f060b8b1dc8421c6df43441f1a23c4e93015a67bba010131cf22bf039","urlAnnotationProperty":"0x019480031b20987583d302c492cc335bb35dee7620757d15e6e551893514d51d4cef39b5c795","alternativeUrl":"0x019480031b2098e8057cbc8f27a31e2767c53bfa92539556c5e5bc42dd8125d9ad5e36a1c10e","sha256Checksum":"0x019480031b20666c8b50fbc7d89ea779e24a176600edefec08d40d15b155af6f32005f4072a5"};
        const schema = [{"key":"urlAnnotationProperty","assertion":{"type":"DataProperty","annotations":["0x019580031b20428df13c43218f450d449c62438a7a491ab1c0eb87ab16391ebe31ec8592e03d"]}},{"key":"sha256Checksum","assertion":{"type":"DataProperty","annotations":["0x019580031b20eae6866f060b8b1dc8421c6df43441f1a23c4e93015a67bba010131cf22bf039"]}},{"key":"alternativeUrl","assertion":{"type":"DataProperty","annotations":["0x019580031b207601ac7214fb3a19e6c8581a3de0dac73d7443b52e81e66fd8075c1e67d9a6f1"]}}];

        client.initSchema(schemaCIDs, schema);
        client.initClient();

        map.set(stringConfig, client);
        return getClient(config);
      }
    }

    const t = getClient({});
    t.getClient = getClient;

    module.exports = t;