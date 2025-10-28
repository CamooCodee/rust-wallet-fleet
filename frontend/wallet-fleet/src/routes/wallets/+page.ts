import type { Load } from "@sveltejs/kit";
import type { ApiResponse } from "../../util/api";
import { apiUrl } from "../../util/util";

interface WalletListResponse extends ApiResponse {
    wallets: {
        pubkey: string,
        sol_lamports: string
    }[]
}

export const load: Load = async ({ fetch }) => {
    const response = await fetch(apiUrl("/wallets/list"));
    const data = await response.json() as WalletListResponse;

    return data;
}