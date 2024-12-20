"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const polkadot_api_1 = require("polkadot-api");
const logs_provider_1 = require("polkadot-api/logs-provider");
const node_1 = require("polkadot-api/ws-provider/node");
const smoldot_1 = require("smoldot");
const fs_1 = __importDefault(require("fs"));
const path_1 = __importDefault(require("path"));
const rpc = process.env.RPC;
const wsProvider = (0, node_1.getWsProvider)(rpc);
function test() {
    return __awaiter(this, void 0, void 0, function* () {
        const provider = (0, logs_provider_1.withLogsRecorder)((line) => console.log(line), wsProvider);
        const client = (0, polkadot_api_1.createClient)(provider);
        try {
            // Start Smoldot client and specify the WebSocket provider
            const smoldotClient = (0, smoldot_1.start)();
            // Read chain specification from a file (ensure the path is correct)
            const chainSpecPath = path_1.default.join(__dirname, '..', 'chainSpec.json'); // adjust this path
            const chainSpec = fs_1.default.readFileSync(chainSpecPath, 'utf8');
            // Add the chain specification to the Smoldot client
            const chain = yield smoldotClient.addChain({ chainSpec });
            console.log('Successfully connected to the chain using Smoldot!');
            // Fetch the genesis block hash using Smoldot
            const genesisHash = yield (yield chain).sendJsonRpc('chain_getBlockHash');
            console.log('Genesis Hash:', genesisHash);
            // Optionally, subscribe to real-time notifications from the chain
            chain.nextJsonRpcResponse().then((response) => {
                console.log('Received notification/response:', response);
            });
            // Further interaction with the `client` (Polkadot API)
            // const chainInfo =  client.rpc.system.chain();
            // console.log('Chain Info (Polkadot API):', chainInfo);
        }
        catch (error) {
            console.log('Error: ', error);
        }
    });
}
test().catch((error) => {
    console.error('Error executing test:', error);
});
// async function local() {
//   const client = start();
//   const chainSpec = 'ws://127.0.0.1:9999';
//   try {
//     const chain = await client.addChain({ chainSpec });
//     console.log('Connected to Xode!');
//     // Example: Continuously listen for messages from the chain
//     while (true) {
//       const notification = await chain.nextJsonRpcResponse();
//       console.log('Received notification:', notification);
//       const genesisHash = await chain.sendJsonRpc('chain_getBlockHash');
//       console.log('Genesis Hash:', genesisHash);
//     }
//   } catch (error) {
//       console.error('Error connecting to Xode:', error);
//   }
// }
// local().catch((error) => {
//   console.error('Error connecting to Xode:', error);
// });
// async function main() {
//   // Initialize Smoldot provider
//   const client = start();
//   // 2. Add a connection to Xode (replace with the actual Xode endpoint)
//   const chainSpec = 'ws://rpcnodea01.xode.net/n7yoxCmcIrCF6VziCcDmYTwL8R03a/rpc';
//   const chain = client.addChain({ 
//     chainSpec
//   });
//   // 3. Poll for messages (notifications)
//   while (true) {
//     const notification =  (await chain).nextJsonRpcResponse();
//     console.log('Received notification:', notification);
//   }
//   // 4. Send an RPC call (example: fetch the genesis hash)
//   const genesisHash = (await chain).sendJsonRpc('chain_getBlockHash');
//   console.log('Genesis Hash:', genesisHash);
// }
// main().catch((error) => {
//   console.error('Error connecting to Xode:', error);
// });
