export function capitalize_first(str: string): string {
    if (str.length === 0) {
        return str;
    }
    return str[0].toUpperCase() + str.substring(1);
}

export function capitalize_all(str: string, separator?: string): string {
    return str.split(separator ?? " ").map(capitalize_first).join(separator ?? " ");
}
