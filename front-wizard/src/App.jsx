import "./App.css";
import CodeTerminal from "./CodeTerminal.jsx";
import Buttons from "./components/Buttons.jsx";
import SideBar from "./components/Sidebar";
import logo from "./assets/logo.png";
import { alephzeroTestnet, UseInkathonProvider } from "@scio-labs/use-inkathon";
import {useFormik} from "formik";

function App() {

  const settingsForm = useFormik({
    initialValues: {
      name: "",
      symbol: "",
      decimals: "",
      metadata: false,
      mintable: false,
      burnable: false,
      wrapper: false,
      flashMint: false,
      pausable: false,
      capped: false,
    }
  });


  return (
    <>
      <UseInkathonProvider
        appName={"SmartBeaver"}
        defaultChain={alephzeroTestnet}
        connectOnInit={true}
      >
        <div className="">
          <div className="flex flex-row justify-between items-center mx-10 mt-6 mb-10">
            <img src={logo} alt="Logo" className="w-56" />
            <button className="hover:text-black text-white hover:bg-white rounded-2xl bg-[#238880] h-10 px-8 py-4 flex items-center justify-center">
              Log in
            </button>
          </div>
          <div className="w-[85%] mx-auto border border-white border-opacity-30 p-10 rounded-2xl shadowClass">
            <Buttons settingsForm={settingsForm}/>
            <div className="flex flex-row justify-between">
              <SideBar settingsForm={settingsForm} />
              <CodeTerminal />
            </div>
          </div>
        </div>
      </UseInkathonProvider>
    </>
  );
}

export default App;
