import { useQuery } from "@tanstack/react-query"
import type { UseQueryResult } from "@tanstack/react-query"

import { api } from "#/libs/api/client.ts"
import type { TSessionMeta } from "#/libs/api/types.ts"

export type TProjectCount = {
	project: string
	count: number
}

export type TOverviewStats = {
	total: number
	projects: number
	chunks: number
	ready: number
	processing: number
	pending: number
	failed: number
	lastActivity: number | null
	topProjects: readonly TProjectCount[]
	recent: readonly TSessionMeta[]
}

const STATUS_KEYS = ["ready", "processing", "pending", "failed"] as const

const buildStats = (sessions: readonly TSessionMeta[]): TOverviewStats => {
	const byProject = new Map<string, number>()
	const counts = { ready: 0, processing: 0, pending: 0, failed: 0 }
	let chunks = 0
	let lastActivity: number | null = null
	for (const session of sessions) {
		byProject.set(session.project, (byProject.get(session.project) ?? 0) + 1)
		chunks += Math.max(0, session.chunk_count)
		if ((STATUS_KEYS as readonly string[]).includes(session.status)) {
			counts[session.status as (typeof STATUS_KEYS)[number]] += 1
		}
		if (lastActivity === null || session.ended_at > lastActivity) {
			lastActivity = session.ended_at
		}
	}
	const topProjects = [...byProject.entries()]
		.map(([project, count]) => ({ project, count }))
		.sort((a, b) => b.count - a.count)
		.slice(0, 6)
	const recent = [...sessions]
		.sort((a, b) => b.ended_at - a.ended_at)
		.slice(0, 5)
	return {
		total: sessions.length,
		projects: byProject.size,
		chunks,
		...counts,
		lastActivity,
		topProjects,
		recent,
	}
}

type TUseOverviewReturn = {
	query: UseQueryResult<TOverviewStats, Error>
}

export const useOverview = (): TUseOverviewReturn => ({
	query: useQuery({
		queryKey: ["overview"],
		queryFn: async (): Promise<TOverviewStats> => {
			const { sessions } = await api.listSessions({ limit: 100 })
			return buildStats(sessions)
		},
	}),
})
