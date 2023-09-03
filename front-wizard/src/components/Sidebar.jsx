import AccessBox from "./AccessControl";
import FeaturesBoxPSP22 from "./FeaturesPSP22";
import FeaturesBoxPSP37 from "./FeaturesPSP37";
import InfoBox from "./InfoBox.jsx";
import SettingsBox from "./Settings";
import { useInkathon } from "@scio-labs/use-inkathon";
import { useCallback, useEffect } from "react";
import UpgradeabilityBox from "./Upgradeability";
import { detectInstalledWallet } from "../utils/generalUtils.js";
import { useStandardSelect } from "../contexts/useStandardSelect";
import useFormData from "../contexts/useFormData.js";
import { sendGenerateRequest } from "../api/BackendAPIHandler.js";

function SideBar({ settingsForm }) {
  const { connect, activeChain, isConnected } = useInkathon();
  const buttonChoosen = useStandardSelect((s) => s.selectedTab);
  const { setCode } = useFormData();

  useEffect(() => {
    console.log("connect", connect);
    console.log("activeChain", activeChain);
    console.log("isConnected", isConnected);
    console.log("Window wallet detected: ", detectInstalledWallet());
  }, []);

  const callCodeGenerator = useCallback(() => {
    console.log("Form", settingsForm.values);
    sendGenerateRequest("psp22", [
      "mintable",
      "burnable",
      "pausable",
      "capped",
      "metadata",
    ])
      .then((responseData) => {
        const encodedData = responseData.encodedData;
        const decodedData = atob(encodedData);
        // console.log(decodedData);
        setCode(decodedData);
      })
      .catch((error) => {
        // Handle errors
      });
  }, []);

  return (
    <div className="flex flex-col justify-start text-white items-center text-center">
      <div>
        <SettingsBox formik={settingsForm} />
      </div>
      <div>
        {buttonChoosen === "psp22" ? (
          <FeaturesBoxPSP22 formik={settingsForm} />
        ) : (
          (() => {
            return <FeaturesBoxPSP37 formik={settingsForm} />;
          })()
        )}
      </div>
      <div>
        <AccessBox />
      </div>
      <div>
        <UpgradeabilityBox />
      </div>
      <div>
        <InfoBox />
      </div>
      <div>
        <button
          className="text-black hover:text-white bg-white rounded-2xl hover:bg-[#238880] h-10 px-8 py-4 flex items-center justify-center"
          onClick={callCodeGenerator}
        >
          Submit
        </button>
      </div>
    </div>
  );
}

export default SideBar;
