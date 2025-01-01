import { Coupon, fetchCoupons } from "@/api/coupon";
import { ApplyCouponRequest, ApplyCouponResponse, BaseRequest, BaseResponse, CouponsFoundResponse } from "@/types/messages";
import { detectCheckoutPage } from "@/utils/pageDetection";

export default defineContentScript({
  matches: ["<all_urls>"],
  matchAboutBlank: false,
  main: () => {
    let cachedCoupons: Coupon[] = []
    let fetchError = false

    // Listen for messages from popup to apply coupons
    browser.runtime.onMessage.addListener(async (message: any, sender): Promise<BaseResponse> => {
      let baseMessage = message as BaseRequest;
      switch (baseMessage.type) {
        case 'APPLY_COUPON':
          return handleApplyCouponRequest((message as ApplyCouponRequest).code)
        case 'COUPONS_FOUND':
          return handleCouponRequest(cachedCoupons, fetchError)
        default:
          return {
            success: false,
            error: 'Invalid message type'
          }
      }
    })

    // Fetch coupons for the current domain
    if (!detectCheckoutPage()) return
    const currentDomain = window.location.hostname
    fetchCoupons(currentDomain)
      .then(coupons => {
        cachedCoupons = coupons
      })
      .catch(error => {
        fetchError = true
        console.error('Failed to fetch coupons:', error)
      })
  },
});

async function handleCouponRequest(coupons: Coupon[], error: boolean): Promise<CouponsFoundResponse> {
  return {
    success: !error,
    coupons
  }
}

async function handleApplyCouponRequest(code: string): Promise<ApplyCouponResponse> {
  const couponField = findCouponField()

  if (couponField) {
    try {
      couponField.value = code
      couponField.dispatchEvent(new Event('input', { bubbles: true }))
      return {
        success: true,
        applied: true,
        copied: false
      }
    } catch (error) {
      return {
        success: false,
        error: 'Failed to apply coupon',
        applied: false,
        copied: false
      }
    }
  } else {
    try {
      await navigator.clipboard.writeText(code)
      return {
        success: true,
        applied: false,
        copied: true
      }
    } catch (error) {
      return {
        success: false,
        error: 'Failed to copy to clipboard',
        applied: false,
        copied: false
      }
    }
  }
}
