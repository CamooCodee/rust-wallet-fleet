import config from "./config";

export function shortenPubkey(pubkey: string, charsToShow: number = 5) {
    const start = pubkey.slice(0, charsToShow);
    const end = pubkey.slice(pubkey.length - charsToShow, pubkey.length);

    return `${start}...${end}`;
}

export function apiUrl(endpoint: string) {
    return `${config.apiUrl}${endpoint}`;
}

export function postApi(endpoint: string, data: any) {
    return fetch(apiUrl(endpoint), {
        method: "POST",
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(data)
    })
}