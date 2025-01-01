export interface Coupon {
    code: string
    description: string
    success_rate: number
    last_tested: string
}

export const fetchCoupons = async (domain: string): Promise<Coupon[]> => {
    // In a real implementation, this would call your coupon API
    // For demo purposes, returning mock data
    return [
        {
            code: "SAVE20",
            description: "20% off your order",
            success_rate: 85,
            last_tested: "2024-12-28"
        },
        {
            code: "FREESHIP",
            description: "Free shipping on orders over $50",
            success_rate: 92,
            last_tested: "2024-12-29"
        }
    ]
}
