import { clearToken, getToken } from "#/libs/auth/token.ts"
import type {
	TDeleteResponse,
	TListSessionsParams,
	TSearchParams,
	TSearchResponse,
	TSessionList,
	TSessionMeta,
} from "./types.ts"

const API_BASE = import.meta.env.VITE_API_URL ?? ""

export class ApiError extends Error {
	readonly status: number

	constructor(status: number, message: string) {
		super(message)
		this.name = "ApiError"
		this.status = status
	}
}

const toQuery = (
	params: Record<string, string | number | undefined>,
): string => {
	const search = new URLSearchParams()
	for (const [key, value] of Object.entries(params)) {
		if (value !== undefined && value !== "") search.set(key, String(value))
	}
	const query = search.toString()
	return query ? `?${query}` : ""
}

const request = async <T>(path: string, init?: RequestInit): Promise<T> => {
	const token = getToken()
	const response = await fetch(`${API_BASE}${path}`, {
		...init,
		headers: {
			...(init?.headers ?? {}),
			...(token ? { authorization: `Bearer ${token}` } : {}),
		},
	})
	if (response.status === 401) {
		clearToken()
		throw new ApiError(401, "Unauthorized — your API token was rejected")
	}
	if (!response.ok) {
		const detail = await response.text().catch(() => response.statusText)
		throw new ApiError(response.status, detail || `Request failed (${response.status})`)
	}
	return (await response.json()) as T
}

export const api = {
	listSessions: (params: TListSessionsParams = {}): Promise<TSessionList> =>
		request<TSessionList>(`/v1/sessions${toQuery(params)}`),
	getSession: (id: string): Promise<TSessionMeta> =>
		request<TSessionMeta>(`/v1/sessions/${encodeURIComponent(id)}`),
	search: (params: TSearchParams): Promise<TSearchResponse> =>
		request<TSearchResponse>(`/v1/search${toQuery(params)}`),
	deleteSession: (id: string): Promise<TDeleteResponse> =>
		request<TDeleteResponse>(`/v1/sessions/${encodeURIComponent(id)}`, {
			method: "DELETE",
		}),
	verifyToken: async (token: string): Promise<boolean> => {
		const response = await fetch(`${API_BASE}/v1/sessions?limit=1`, {
			headers: { authorization: `Bearer ${token}` },
		})
		return response.ok
	},
}
