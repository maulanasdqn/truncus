import { createFileRoute, Link } from "@tanstack/react-router"
import { ArrowLeftIcon } from "lucide-react"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { SkeletonLines, SkeletonStatGrid } from "#/components/loading-skeletons.tsx"
import { SessionDetail } from "./_components/session-detail.tsx"
import { useSession } from "./_hooks/use-session.ts"

const SessionDetailPage: FC = (): ReactElement => {
	const { sessionId } = Route.useParams()
	const { query, deleteMutation } = useSession(sessionId)
	return (
		<section className="flex flex-col gap-6">
			<Link
				to="/sessions"
				className="inline-flex w-fit items-center gap-2 text-xs uppercase tracking-label text-muted-foreground hover:text-foreground"
			>
				<ArrowLeftIcon className="size-4" />
				Sessions
			</Link>
			{match(query)
				.with({ status: "pending" }, () => (
					<div className="flex flex-col gap-6">
						<SkeletonStatGrid cards={4} columns={4} />
						<SkeletonLines lines={6} />
					</div>
				))
				.with({ status: "error" }, ({ error }) => (
					<p role="alert" className="text-sm text-destructive">
						Failed to load: {error.message}
					</p>
				))
				.otherwise(({ data }) =>
					data ? (
						<SessionDetail
							session={data}
							isDeleting={deleteMutation.isPending}
							onDelete={() => deleteMutation.mutate()}
						/>
					) : null,
				)}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/sessions/$sessionId/")({
	component: SessionDetailPage,
})
