export const detectCheckoutPage = (): boolean => {
    const url = window.location.href.toLowerCase()
    const keywords = ['checkout', 'cart', 'basket', 'order', 'payment']

    // Check URL
    if (keywords.some(keyword => url.includes(keyword))) return true

    // Check page content
    const pageText = document.body.innerText.toLowerCase()
    const contentKeywords = [
        'shipping address',
        'billing address',
        'payment method',
        'promo code',
        'coupon code'
    ]

    return contentKeywords.some(keyword => pageText.includes(keyword))
}
