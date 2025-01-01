import { BaseResponse } from '../types/messages'

export async function sendMessage<TMessage, TResponse extends BaseResponse>(
    message: TMessage
): Promise<TResponse> {
    try {
        const response = await browser.runtime.sendMessage<TMessage, TResponse>(message)
        return response as TResponse
    } catch (error) {
        return {
            success: false,
            error: error instanceof Error ? error.message : 'Unknown error occurred'
        } as TResponse
    }
}

export async function sendTabMessage<TMessage, TResponse extends BaseResponse>(
    tabId: number,
    message: TMessage
): Promise<TResponse> {
    try {
        const response = await browser.tabs.sendMessage<TMessage, TResponse>(tabId, message)
        return response
    } catch (error) {
        return {
            success: false,
            error: error instanceof Error ? error.message : 'Unknown error occurred'
        } as TResponse
    }
}
