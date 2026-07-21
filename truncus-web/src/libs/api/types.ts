export type TSessionStatus = "pending" | "processing" | "ready" | "failed"

export type TSessionMeta = {
	id: string
	project: string
	machine: string
	started_at: number
	ended_at: number
	status: string
	summary: string | null
	error: string | null
	chunk_count: number
}

export type TSessionList = {
	sessions: TSessionMeta[]
}

export type TSearchHit = {
	session_id: string
	kind: string
	score: number
	text: string
	project: string
	ended_at: number
}

export type TSearchResponse = {
	hits: TSearchHit[]
}

export type TDeleteResponse = {
	id: string
	status: string
}

export type TListSessionsParams = {
	project?: string
	limit?: number
}

export type TSearchParams = {
	q: string
	project?: string
	kind?: string
	limit?: number
}
