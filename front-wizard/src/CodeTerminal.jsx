import { useCallback, useEffect, useRef } from "react";
import useFormData from "./contexts/useFormData.js";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import ClipboardJS from "clipboard";

import Highlight from "react-highlight";
import { sendGenerateRequest } from "./api/BackendAPIHandler.js";

const CodeTerminal = () => {
  const formData = useFormData();

  const handleCopyToClipboard = () => {
    // Copy text to clipboard
    const clipboard = new ClipboardJS(".copy-button", {
      text: () => formData.generatedCode,
    });

    clipboard.on("success", () => {
      // Show a success toast when copied to clipboard
      toast.success("You bea(ve)r it in your clipboard now! 🦫 ", {
        position: "top-right",
        autoClose: 2000, // Adjust the duration as needed
      });

      // Destroy the clipboard instance to prevent memory leaks
      clipboard.destroy();
    });

    clipboard.on("error", () => {
      // Handle error if copying fails
      toast.error("Failed to copy to clipboard 🫥 ", {
        position: "top-right",
        autoClose: 2000, // Adjust the duration as needed
      });

      // Destroy the clipboard instance to prevent memory leaks
      clipboard.destroy();
    });

    // Trigger the clipboard copy action
    clipboard.onClick({ trigger: document.querySelector(".copy-button") });
  };

  useEffect(() => {
    sendGenerateRequest("psp22", [])
      .then((responseData) => {
        const encodedData = responseData.encodedData;
        const decodedData = atob(encodedData);
        formData.setCode(decodedData);
      })
      .catch((error) => {
        // Handle errors
        console.warn(error);
      });
  }, []);

  const lastScrollY = useRef(window.scrollY);
  const holdCopyButton = useRef(null);

  const handleScroll = useCallback(() => {
    const currentScrollY = window.scrollY;
    const buttonPosition = holdCopyButton.current;

    if (buttonPosition) {
      if (lastScrollY.current < currentScrollY && currentScrollY > 300) {
        if (!buttonPosition.classList.contains("copyButtonClass")) {
          buttonPosition.classList.remove("relative");
          buttonPosition.classList.add("copyButtonClass");
        }
      } else {
        if (
          buttonPosition.classList.contains("copyButtonClass") &&
          currentScrollY < 300
        ) {
          buttonPosition.classList.remove("copyButtonClass");
        }
      }
    }

    lastScrollY.current = currentScrollY;
  }, []);

  useEffect(() => {
    window.addEventListener("scroll", handleScroll);

    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, [handleScroll]);

  return (
    <>
      <div className="w-[1000px] componentTranslate">
        <div>
          <button
            className="text-black border bg-white font-bold border-white px-4 py-2 relative float-right rounded-2xl hover:border-transparent mr-8 mt-8 hover:bg-[#238880] copy-button transformButton"
            onClick={handleCopyToClipboard}
            ref={holdCopyButton}
          >
            Copy
          </button>
        </div>
        <div className="max-h-[90vh] overflow-auto w-[1000px]">
          <Highlight className="rust bg-[#0D4948] w-[800px]">
            {formData.generatedCode}
          </Highlight>
        </div>
        {/*<ToastContainer theme="dark" />*/}
      </div>
    </>
  );
};

export default CodeTerminal;
