import { createFileRoute } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { SkeletonStatGrid } from "#/components/loading-skeletons.tsx"
import { OverviewStats } from "./_components/overview-stats.tsx"
import { useOverview } from "./_hooks/use-overview.ts"

const OverviewPage: FC = (): ReactElement => {
	const { query } = useOverview()
	return (
		<section aria-labelledby="overview-heading" className="flex flex-col gap-6">
			<header className="flex flex-col gap-1">
				<h1
					id="overview-heading"
					className="text-2xl font-bold uppercase tracking-label"
				>
					Overview
				</h1>
				<p className="text-sm font-light text-muted-foreground">
					Everything Truncus remembers, at a glance.
				</p>
			</header>
			{match(query)
				.with({ status: "pending" }, () => (
					<SkeletonStatGrid cards={4} columns={4} />
				))
				.with({ status: "error" }, ({ error }) => (
					<p role="alert" className="text-sm text-destructive">
						Failed to load: {error.message}
					</p>
				))
				.otherwise(({ data }) =>
					data ? <OverviewStats stats={data} /> : null,
				)}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/overview/")({
	component: OverviewPage,
})
