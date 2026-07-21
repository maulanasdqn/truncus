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

export type TLesson = {
	id: string
	project: string
	category: string
	title: string
	insight: string
	evidence: string
	confidence: number
	times_seen: number
	created_at: number
	updated_at: number
}

export type TLessonList = {
	lessons: TLesson[]
}

export type TNoteMeta = {
	path: string
	title: string
	content_hash: string
	chunk_count: number
	updated_at: number
}

export type TNoteList = {
	notes: TNoteMeta[]
}

export type TNoteProject = {
	project: string
	note_count: number
}

export type TNoteProjectList = {
	projects: TNoteProject[]
}

export type TNotesRemoved = {
	removed: number
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
