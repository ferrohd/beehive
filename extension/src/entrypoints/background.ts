import { browser } from "wxt/browser";

export default defineBackground({
  persistent: true,
  type: "module",
  main: () => {
    browser.runtime.onInstalled.addListener(() => {
      console.log("[My Honey Clone] Extension installed.");
    });

    /**
     * This example logic:
     * 1. Listens for tab updates to detect potential checkout pages.
     * 2. If a checkout page is detected, sends a message to the content script to check for coupon fields.
     */
    browser.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
      if (changeInfo.status === "complete" && tab.url) {
        const isCheckoutPage = /checkout|cart|payment/i.test(tab.url);
        if (isCheckoutPage) {
          browser.tabs.sendMessage(tabId, { type: "CHECK_FOR_COUPON" });
        }
      }
    })
}});
