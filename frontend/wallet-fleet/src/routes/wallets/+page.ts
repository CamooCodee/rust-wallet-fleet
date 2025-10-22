import type { Load } from "@sveltejs/kit";
import config from "../../util/config";
import type { ApiResponse } from "../../util/api";
import { apiUrl } from "../../util/util";

interface WalletListResponse extends ApiResponse {
    pubkeys: string[]
}

export const load: Load = async ({ fetch }) => {
    console.log("load")
    const response = await fetch(apiUrl("/wallets/list"));
    const data = await response.json() as WalletListResponse;

    return data;
}