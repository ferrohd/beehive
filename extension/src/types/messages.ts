export interface Coupon {
    code: string
    description: string
    success_rate: number
    last_tested: string
}

export interface BaseRequest {
    type: string
}

export interface BaseResponse {
    success: boolean
    error?: string
}

// CouponFound
export interface CouponsFoundRequest extends BaseRequest {
    type: 'COUPONS_FOUND'
}

export interface CouponsFoundResponse extends BaseResponse {
    coupons: Coupon[]
}

// ApplyCoupon
export interface ApplyCouponRequest extends BaseRequest {
    type: 'APPLY_COUPON'
    code: string
}
export interface ApplyCouponResponse extends BaseResponse {
    applied: boolean
    copied: boolean
}
