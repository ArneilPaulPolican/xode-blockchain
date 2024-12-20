import { createClient } from "polkadot-api"
import { withLogsRecorder } from "polkadot-api/logs-provider"
import { getWsProvider } from "polkadot-api/ws-provider/node"
import { start } from 'smoldot'
import * as smoldot from 'smoldot'
import fs from 'fs';
import path from 'path';

const rpc = process.env.RPC;
const wsProvider = getWsProvider(rpc as string)

async function test(): Promise<void> {
  const provider = withLogsRecorder((line) => console.log(line), wsProvider)
  const client = createClient(provider)
  try {
    // Start Smoldot client and specify the WebSocket provider
    const smoldotClient = start();

    // Read chain specification from a file (ensure the path is correct)
    const chainSpecPath = path.join(__dirname, '..', 'chainSpec.json');  // adjust this path
    const chainSpec = fs.readFileSync(chainSpecPath, 'utf8');

    // Add the chain specification to the Smoldot client
    const chain = await smoldotClient.addChain({ chainSpec });
    console.log('Successfully connected to the chain using Smoldot!');

    // Fetch the genesis block hash using Smoldot
    const genesisHash = await (await chain).sendJsonRpc('chain_getBlockHash');
    console.log('Genesis Hash:', genesisHash);

    // Optionally, subscribe to real-time notifications from the chain
    chain.nextJsonRpcResponse().then((response: any) => {
      console.log('Received notification/response:', response);
    });

    // Further interaction with the `client` (Polkadot API)
    // const chainInfo =  client.rpc.system.chain();
    // console.log('Chain Info (Polkadot API):', chainInfo);

  } catch (error) {
    console.log('Error: ', error)
  }
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