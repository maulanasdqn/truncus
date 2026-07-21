const TOKEN_KEY = "truncus.token"

export const getToken = (): string | null => {
	try {
		return globalThis.localStorage?.getItem(TOKEN_KEY) ?? null
	} catch {
		return null
	}
}

export const setToken = (token: string): void => {
	globalThis.localStorage?.setItem(TOKEN_KEY, token)
}

export const clearToken = (): void => {
	globalThis.localStorage?.removeItem(TOKEN_KEY)
}

export const hasToken = (): boolean => getToken() !== null
