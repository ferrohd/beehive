import { useEffect, useState } from 'react'
import { Alert, AlertDescription } from '@/components/ui/alert.tsx'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card.tsx'
import { Badge } from '@/components/ui/badge.tsx'
import { Loader2 } from 'lucide-react'
import { ApplyCouponRequest, ApplyCouponResponse, CouponsFoundRequest, CouponsFoundResponse } from '@/types/messages'

interface Coupon {
    code: string
    description: string
    success_rate: number
    last_tested: string
}

export default function App() {
    const [coupons, setCoupons] = useState<Coupon[]>([])
    const [loading, setLoading] = useState(true)
    const [selectedCoupon, setSelectedCoupon] = useState<string | null>(null)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        const getCoupons = async () => {
            try {
                const [tab] = await browser.tabs.query({ active: true, currentWindow: true })
                if (!tab?.id) throw new Error('Invalid tab')

                let message: CouponsFoundRequest = { type: 'COUPONS_FOUND' }
                const response = await sendTabMessage<CouponsFoundRequest, CouponsFoundResponse>(tab.id, message)

                if (!response.success) {
                    throw new Error(response.error || 'Failed to fetch coupons')
                }

                setCoupons(response.coupons)
            } catch (err) {
                setError('Failed to fetch coupons')
                console.error(err)
            } finally {
                setLoading(false)
            }
        }

        getCoupons()
    }, [])

    const applyCoupon = async (code: string) => {
        setSelectedCoupon(code)

        try {
            const [tab] = await browser.tabs.query({ active: true, currentWindow: true })
            if (!tab?.id) throw new Error('Invalid tab')

            let message: ApplyCouponRequest = { type: 'APPLY_COUPON', code }
            const response = await sendTabMessage<ApplyCouponRequest,ApplyCouponResponse>(tab.id, message)

            if (!response.success) {
                setError(response.error || 'Failed to apply coupon')
                return
            }

            if (response.copied) {
                setError('Could not find coupon field. Code copied to clipboard!')
            }
        } catch (err) {
            setError('Failed to communicate with the page')
            console.error(err)
        }
    }

    if (loading) {
        return (
            <div className="flex items-center justify-center h-48">
                <Loader2 className="h-8 w-8 animate-spin" />
            </div>
        )
    }

    if (coupons.length === 0) {
        return (
            <Alert>
                <AlertDescription>
                    No coupons found for this site.
                </AlertDescription>
            </Alert>
        )
    }

    return (
        <div className="w-96 p-4">
            <CardHeader>
                <CardTitle>Available Coupons</CardTitle>
                <CardDescription>
                    Click a coupon to apply it automatically
                </CardDescription>
            </CardHeader>

            <CardContent>
                {error && (
                    <Alert className="mb-4">
                        <AlertDescription>{error}</AlertDescription>
                    </Alert>
                )}

                <div className="space-y-2">
                    {coupons.map((coupon) => (
                        <Card
                            key={coupon.code}
                            className="cursor-pointer hover:bg-gray-50"
                            onClick={() => applyCoupon(coupon.code)}
                        >
                            <CardContent className="p-4">
                                <div className="flex justify-between items-center">
                                    <div>
                                        <p className="font-mono text-lg">{coupon.code}</p>
                                        <p className="text-sm text-gray-600">{coupon.description}</p>
                                    </div>
                                    <Badge variant="secondary">
                                        {coupon.success_rate}% success
                                    </Badge>
                                </div>
                            </CardContent>
                        </Card>
                    ))}
                </div>
            </CardContent>
        </div>
    )
}
