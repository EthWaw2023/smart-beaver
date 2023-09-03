import { create } from "zustand";

export const useStandardSelect = create((set) => ({
  selectedTab: "psp22",
  setSelection: (selection) => set({ selectedTab: selection }),
}));
