import { Link } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { StatusBadge } from "#/components/status-badge.tsx"
import { Card, CardContent, CardHeader, CardTitle } from "#/components/ui/card.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { formatRelative } from "#/libs/format/index.ts"
import type { TOverviewStats, TProjectCount } from "../_hooks/use-overview.ts"

const StatCard: FC<{
	label: string
	value: string
	note: string
}> = ({ label, value, note }): ReactElement => (
	<Card className="h-full">
		<CardContent>
			<dl className="flex flex-col gap-1">
				<dt className="text-xs uppercase tracking-label text-muted-foreground">
					{label}
				</dt>
				<dd className="text-4xl font-bold">{value}</dd>
				<dd className="text-xs font-light text-muted-foreground">{note}</dd>
			</dl>
		</CardContent>
	</Card>
)

const ProjectRow: FC<{ item: TProjectCount; max: number }> = ({
	item,
	max,
}): ReactElement => (
	<li>
		<Link
			to="/sessions"
			search={{ project: item.project }}
			className="group flex flex-col gap-1"
		>
			<div className="flex items-center justify-between gap-3 text-sm">
				<span className="truncate font-medium group-hover:underline">
					{item.project}
				</span>
				<span className="shrink-0 text-muted-foreground">{item.count}</span>
			</div>
			<div className="h-1.5 w-full bg-muted">
				<div
					className="h-full bg-primary"
					style={{ width: `${Math.max(6, (item.count / max) * 100)}%` }}
				/>
			</div>
		</Link>
	</li>
)

type TProps = {
	stats: TOverviewStats
}

export const OverviewStats: FC<TProps> = ({ stats }): ReactElement => {
	const maxProject = stats.topProjects[0]?.count ?? 1
	return (
		<div className="flex flex-col gap-6">
			<ul className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<li>
					<StatCard
						label="Sessions"
						value={String(stats.total)}
						note={`${stats.ready} ready · ${stats.failed} failed`}
					/>
				</li>
				<li>
					<StatCard
						label="Projects"
						value={String(stats.projects)}
						note="Distinct codebases remembered"
					/>
				</li>
				<li>
					<StatCard
						label="Memory chunks"
						value={String(stats.chunks)}
						note="Embedded and searchable"
					/>
				</li>
				<li>
					<StatCard
						label="Last activity"
						value={match(stats.lastActivity)
							.with(null, () => "—")
							.otherwise((ms) => formatRelative(ms))}
						note={`${stats.processing + stats.pending} in the pipeline`}
					/>
				</li>
			</ul>
			<div className="grid gap-6 lg:grid-cols-2">
				<Card>
					<CardHeader>
						<CardTitle className="text-sm uppercase tracking-label">
							Top projects
						</CardTitle>
					</CardHeader>
					<CardContent>
						{match(stats.topProjects.length)
							.with(0, () => (
								<EmptyState message="No sessions captured yet." />
							))
							.otherwise(() => (
								<ul className="flex flex-col gap-4">
									{stats.topProjects.map((item) => (
										<ProjectRow
											key={item.project}
											item={item}
											max={maxProject}
										/>
									))}
								</ul>
							))}
					</CardContent>
				</Card>
				<Card>
					<CardHeader>
						<CardTitle className="text-sm uppercase tracking-label">
							Recent sessions
						</CardTitle>
					</CardHeader>
					<CardContent>
						{match(stats.recent.length)
							.with(0, () => (
								<EmptyState message="No sessions captured yet." />
							))
							.otherwise(() => (
								<ul className="flex flex-col divide-y divide-border">
									{stats.recent.map((session) => (
										<li key={session.id}>
											<Link
												to="/sessions/$sessionId"
												params={{ sessionId: session.id }}
												className="flex items-center justify-between gap-3 py-3 first:pt-0 last:pb-0"
											>
												<div className="flex min-w-0 flex-col gap-1">
													<span className="truncate text-sm font-medium">
														{session.project}
													</span>
													<span className="truncate text-xs text-muted-foreground">
														{session.machine} ·{" "}
														{formatRelative(session.ended_at)}
													</span>
												</div>
												<StatusBadge status={session.status} />
											</Link>
										</li>
									))}
								</ul>
							))}
					</CardContent>
				</Card>
			</div>
		</div>
	)
}
