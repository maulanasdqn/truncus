import { createFileRoute } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"
import { z } from "zod"

import { SkeletonTable } from "#/components/loading-skeletons.tsx"
import { Card, CardContent } from "#/components/ui/card.tsx"
import { SessionFilters } from "./_components/session-filters.tsx"
import { SessionTable } from "./_components/session-table.tsx"
import { useSessions } from "./_hooks/use-sessions.ts"

const SessionsPage: FC = (): ReactElement => {
	const search = Route.useSearch()
	const navigate = Route.useNavigate()
	const project = search.project ?? "all"
	const query = search.q ?? ""
	const { listQuery, projects, filtered } = useSessions({ project, query })

	const setProject = (value: string): void => {
		void navigate({
			search: (prev) => ({
				...prev,
				project: value === "all" ? undefined : value,
			}),
			replace: true,
		})
	}
	const setQuery = (value: string): void => {
		void navigate({
			search: (prev) => ({ ...prev, q: value === "" ? undefined : value }),
			replace: true,
		})
	}
	const openSession = (id: string): void => {
		void navigate({ to: "/sessions/$sessionId", params: { sessionId: id } })
	}

	return (
		<section aria-labelledby="sessions-heading" className="flex flex-col gap-6">
			<header className="flex flex-col gap-1">
				<h1
					id="sessions-heading"
					className="text-2xl font-bold uppercase tracking-label"
				>
					Sessions
				</h1>
				<p className="text-sm font-light text-muted-foreground">
					Every captured Claude Code session.
				</p>
			</header>
			{match(listQuery)
				.with({ status: "pending" }, () => (
					<SkeletonTable rows={6} columns={6} />
				))
				.with({ status: "error" }, ({ error }) => (
					<p role="alert" className="text-sm text-destructive">
						Failed to load: {error.message}
					</p>
				))
				.otherwise(() => (
					<div className="flex flex-col gap-4">
						<SessionFilters
							projects={projects}
							project={project}
							query={query}
							count={filtered.length}
							onProjectChange={setProject}
							onQueryChange={setQuery}
						/>
						<Card>
							<CardContent>
								<SessionTable sessions={filtered} onOpen={openSession} />
							</CardContent>
						</Card>
					</div>
				))}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/sessions/")({
	validateSearch: z.object({
		project: z.string().optional(),
		q: z.string().optional(),
	}),
	component: SessionsPage,
})
