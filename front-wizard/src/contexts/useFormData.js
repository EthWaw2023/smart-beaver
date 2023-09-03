import {create} from "zustand";
import {subscribeWithSelector} from "zustand/middleware";

const useFormData = create(subscribeWithSelector((set) => ({
    generatedCode: null,
    setCode: (code) => set(() => ({generatedCode: code})),
})));

export default useFormData;
