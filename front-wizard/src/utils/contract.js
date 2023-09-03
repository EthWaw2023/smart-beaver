import { CodePromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
// import contractJsonImport from "/public/contract/flipper.json";

export const deployContract = async (
  provider,
  signerAddr,
  signer,
  bundleData,
  paramsList
) => {
  console.log("deploy: ", provider);
  console.log("signerAddr", signerAddr);
  console.log("signer", signer);
  console.log("bundleData", bundleData);

  //Not neede for now
  const contractWasm = await fetch("contract/flipper.wasm")
    .then((value1) => {
      return value1.arrayBuffer();
    })
    .then((value2) => {
      // console.log(new Uint8Array(value2))
      return new Uint8Array(value2);
    });
  console.log("contractWasm", contractWasm);

  const api = await ApiPromise.create({
    provider: provider,
    initWasm: true,
    signer: signer,
  });

  const parsedBundle = JSON.parse(bundleData);

  console.log("api", api);

  const code = new CodePromise(api, bundleData, parsedBundle.source.wasm);

  // maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
  const gasLimit = import.meta.env.VITE_GAS_LIMIT;
  // a limit to how much Balance to be used to pay for the storage created by the instantiation
  // if null is passed, unlimited balance can be used
  const storageDepositLimit = null;
  // used to derive contract address,
  // use null to prevent duplicate contracts
  const salt = new Uint8Array();
  // balance to transfer to the contract account, formerly known as "endowment".
  // use only with payable constructors, will fail otherwise.
  const value = api.registry.createType("Balance", 1000);
  const initValue = 1;

  const tx = code.tx.new({ gasLimit }, ...paramsList);

  let address;

  // const unsub = await tx.signAndSend(signerAddr, ({ contract, status }) => {
  //   console.log(contract);
  //   console.log(status);
  //   if (status.isInBlock || status.isFinalized) {
  //     address = contract?.address.toString();
  //     unsub();
  //   }
  // });

  const unsub = await tx.signAndSend(
    signerAddr,
    { signer: signer },
    async (result) => {
      if (result.isInBlock || result.isFinalized) {
        const events = {};

        console.warn("result", result);
        console.warn("result json", JSON.stringify(result));

        result.events.forEach((record) => {
          const { event } = record;
          const key = `${event.section}:${event.method}`;
          if (!events[key]) {
            events[key] = 1;
          } else {
            events[key]++;
          }
        });

        console.log("Events: ", events);

        unsub();
      }
    },
  );

  console.log("tmp", JSON.stringify(unsub));

  console.log("address", address);

  console.log(code);
};
