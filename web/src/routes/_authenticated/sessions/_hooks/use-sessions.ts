import { useQuery } from "@tanstack/react-query"
import type { UseQueryResult } from "@tanstack/react-query"
import { useMemo } from "react"

import { api } from "#/libs/api/client.ts"
import type { TSessionMeta } from "#/libs/api/types.ts"

type TFilters = {
	project: string
	query: string
}

type TUseSessionsReturn = {
	listQuery: UseQueryResult<TSessionMeta[], Error>
	projects: readonly string[]
	filtered: readonly TSessionMeta[]
}

const matchesQuery = (session: TSessionMeta, query: string): boolean => {
	if (query === "") return true
	const haystack = [
		session.project,
		session.machine,
		session.id,
		session.summary ?? "",
	]
		.join(" ")
		.toLowerCase()
	return haystack.includes(query)
}

export const useSessions = (filters: TFilters): TUseSessionsReturn => {
	const listQuery = useQuery({
		queryKey: ["sessions", "all"],
		queryFn: async (): Promise<TSessionMeta[]> => {
			const { sessions } = await api.listSessions({ limit: 100 })
			return [...sessions].sort((a, b) => b.ended_at - a.ended_at)
		},
	})
	const sessions = listQuery.data ?? []
	const projects = useMemo(
		() => [...new Set(sessions.map((session) => session.project))].sort(),
		[sessions],
	)
	const filtered = useMemo(() => {
		const query = filters.query.trim().toLowerCase()
		return sessions.filter((session) => {
			if (filters.project !== "all" && session.project !== filters.project) {
				return false
			}
			return matchesQuery(session, query)
		})
	}, [sessions, filters.project, filters.query])
	return { listQuery, projects, filtered }
}
