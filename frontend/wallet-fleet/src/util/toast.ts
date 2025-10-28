import type { ApiResponse } from "./api";

export interface Toast {
    success: boolean;
    message: string;
}

export function toast(toast: Toast) {
    window.dispatchEvent(new CustomEvent("toast:add", { detail: toast as any }))
}

export function toastRes(response: Response, data: ApiResponse) {
    toast({ message: data.message, success: response.status < 300 })
}

export function toastResFailure(response: Response, data: ApiResponse) {
    if (response.status < 300) return;

    toast({ message: data.message, success: false })
}