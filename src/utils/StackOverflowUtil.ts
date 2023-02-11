import type { Item, Result } from '../types/StackOverflowTypes';
import { unescape } from 'html-escaper';

const URL =
    'https://api.stackexchange.com/2.3/search?order=desc&sort=relevance&site=stackoverflow&intitle=';

export async function search(searchTerm: string): Promise<Item[]> {
    if (searchTerm === '') return [];

    try {
        return (
            (await (await fetch(URL + encodeURI(searchTerm))).json()) as Result
        ).items;
    } catch (err) {
        return [];
    }
}

export function unescapeHTML(text: string): string {
    return unescape(text[0].toUpperCase() + text.slice(1));
}
