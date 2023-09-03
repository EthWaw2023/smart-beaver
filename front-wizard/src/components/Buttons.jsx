import WalletConnectButton from "./WalletConnectButton.jsx";
import { useStandardSelect } from "../contexts/useStandardSelect.js";
import { useState } from "react";
import { sendBuildRequest } from "../api/BackendAPIHandler.js";
import { toast, ToastContainer } from "react-toastify";
import { Modal } from "flowbite-react";
import fileDownload from "js-file-download";
import { useInkathon } from "@scio-labs/use-inkathon";
import { deployContract } from "../utils/contract.js";

function Buttons({ settingsForm }) {
  const { setSelection } = useStandardSelect();

  const { activeAccount, activeSigner, provider } = useInkathon();

  const [isBuilding, setBuilding] = useState(false);
  const [contractBuild, setContractBuild] = useState(null);
  const [selectedTab, setSelectedTab] = useState("psp22");
  const isDisabled = true;

  const handleTabSelect = (event) => {
    const tID = event.target.id;
    console.log("Target ID: ", tID);

    setSelectedTab(tID);
    setSelection(tID);
  };

  const handleRequestBuild = async () => {
    setBuilding(true);
    sendBuildRequest("psp22", [
      "mintable",
      "burnable",
      "pausable",
      "capped",
      "metadata",
    ]).then((res) => {
      console.log(res);
      setBuilding(false);
      setContractBuild(atob(res.contractBundle));
      toast.success(
        "Your contract has been built successfully. You can now download full bundle. 🥳",
      );
    });
  };

  const handleContractDeploy = () => {
    if (contractBuild) {
      const formValues = settingsForm.values;
      console.log("Deployment data: ", formValues);

      /*
                   total_supply: Balance,
                  cap: u128,
                  name: Option<String>,
                  symbol: Option<String>,
                  decimals: u8,
                   */

      deployContract(
        provider,
        activeAccount.address,
        activeSigner,
        contractBuild,
        [formValues?.decimals, 100, formValues?.name, formValues?.symbol, 18],
      ).then(() => console.log("Deployment done"));
    }
  };

  const handleBundleDownload = () => {
    if (contractBuild) {
      fileDownload(contractBuild, "psp22.contract", "application/json");
    }
  };

  return (
    <>
      <Modal
        onClose={() => setBuilding(false)}
        show={isBuilding ? "default" : undefined}
      >
        <Modal.Header>Contract building in progress. Please wait</Modal.Header>
        <Modal.Body>
          <div className="space-y-6" style={{ textAlign: "center" }}>
            <div className="lds-dual-ring"></div>
          </div>
        </Modal.Body>
      </Modal>

      <div className="flex flex-row justify-between border border-white  border-opacity-60 mb-10 p-6 rounded-3xl">
        <div className="flex flex-row gap-3">
          <button
            onClick={handleTabSelect}
            id={"psp22"}
            className={`text-black py-2 px-4 rounded-2xl ${
              selectedTab === "psp22"
                ? "bg-[#238880]"
                : "bg-white hover:bg-[#238880]"
            }`}
          >
            PSP22
          </button>
          <button
            onClick={handleTabSelect}
            id={"psp37"}
            className={`text-black py-2 px-4 rounded-2xl ${
              selectedTab === "psp37"
                ? "bg-[#238880]"
                : "bg-white hover:bg-[#238880]"
            }`}
          >
            PSP37
          </button>
          <button
            onClick={handleTabSelect}
            id={"psp34"}
            className={`text-black bg-white py-2 px-4 border rounded-2xl ${
              isDisabled
                ? "opacity-20 cursor-not-allowed"
                : "hover:bg-[#238880]"
            }`}
            disabled={isDisabled}
          >
            PSP34
          </button>
        </div>
        <div>
          <button
            onClick={handleRequestBuild}
            className={
              "text-black bg-white py-2 px-4 rounded-2xl hover:bg-[#238880]"
            }
          >
            Compile code
          </button>
          {contractBuild && (
            <>
              <button
                style={{ marginLeft: "10px" }}
                onClick={handleBundleDownload}
                className={"text-black bg-white py-2 px-4 border rounded-2xl "}
              >
                Download compiled contract bundle
              </button>
              <button
                onClick={handleContractDeploy}
                style={{ marginLeft: "10px" }}
                className={"text-black bg-white py-2 px-4 border rounded-2xl "}
              >
                Deploy contract 🚀
              </button>
            </>
          )}
        </div>
        <div>
          <WalletConnectButton />
        </div>
        <ToastContainer theme="dark" />
      </div>
    </>
  );
}

export default Buttons;
