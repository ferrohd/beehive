export const findCouponField = (): HTMLInputElement | null => {
    // Common coupon field selectors
    const selectors = [
        'input[name*="coupon"]',
        'input[name*="promo"]',
        'input[id*="coupon"]',
        'input[id*="promo"]',
        'input[placeholder*="coupon" i]',
        'input[placeholder*="promo" i]'
    ]

    for (const selector of selectors) {
        const field = document.querySelector(selector) as HTMLInputElement
        if (field) return field
    }

    return null
}
