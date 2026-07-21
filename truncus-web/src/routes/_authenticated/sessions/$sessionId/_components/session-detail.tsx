import { CopyIcon } from "lucide-react"
import type { FC, ReactElement } from "react"
import { toast } from "sonner"
import { match, P } from "ts-pattern"

import { StatusBadge } from "#/components/status-badge.tsx"
import { Card, CardContent, CardHeader, CardTitle } from "#/components/ui/card.tsx"
import { formatDateTime, formatDuration } from "#/libs/format/index.ts"
import type { TSessionMeta } from "#/libs/api/types.ts"
import { DeleteSessionDialog } from "./delete-session-dialog.tsx"

type TProps = {
	session: TSessionMeta
	isDeleting: boolean
	onDelete: () => void
}

const MetaItem: FC<{ label: string; value: string }> = ({
	label,
	value,
}): ReactElement => (
	<div className="flex flex-col gap-1">
		<dt className="text-xs uppercase tracking-label text-muted-foreground">
			{label}
		</dt>
		<dd className="text-sm font-medium break-words">{value}</dd>
	</div>
)

export const SessionDetail: FC<TProps> = ({
	session,
	isDeleting,
	onDelete,
}): ReactElement => {
	const copyId = (): void => {
		void navigator.clipboard
			?.writeText(session.id)
			.then(() => toast.success("Session ID copied"))
	}
	return (
		<div className="flex flex-col gap-6">
			<div className="flex flex-wrap items-start justify-between gap-4">
				<div className="flex min-w-0 flex-col gap-2">
					<div className="flex flex-wrap items-center gap-3">
						<h1 className="text-2xl font-bold uppercase tracking-label">
							{session.project}
						</h1>
						<StatusBadge status={session.status} />
					</div>
					<button
						type="button"
						onClick={copyId}
						className="inline-flex items-center gap-1.5 text-xs font-light text-muted-foreground hover:text-foreground"
					>
						<span className="font-mono break-all">{session.id}</span>
						<CopyIcon className="size-3 shrink-0" />
					</button>
				</div>
				<DeleteSessionDialog
					project={session.project}
					isDeleting={isDeleting}
					onConfirm={onDelete}
				/>
			</div>

			<Card>
				<CardContent>
					<dl className="grid grid-cols-2 gap-6 sm:grid-cols-4">
						<MetaItem label="Machine" value={session.machine} />
						<MetaItem
							label="Started"
							value={formatDateTime(session.started_at)}
						/>
						<MetaItem label="Ended" value={formatDateTime(session.ended_at)} />
						<MetaItem
							label="Duration"
							value={formatDuration(session.started_at, session.ended_at)}
						/>
						<MetaItem label="Chunks" value={String(session.chunk_count)} />
					</dl>
				</CardContent>
			</Card>

			{session.error && session.error !== "" ? (
				<Card className="border-destructive/40">
					<CardHeader>
						<CardTitle className="text-sm uppercase tracking-label text-destructive">
							Pipeline error
						</CardTitle>
					</CardHeader>
					<CardContent>
						<p className="font-mono text-sm text-destructive break-words">
							{session.error}
						</p>
					</CardContent>
				</Card>
			) : null}

			<Card>
				<CardHeader>
					<CardTitle className="text-sm uppercase tracking-label">
						Summary
					</CardTitle>
				</CardHeader>
				<CardContent>
					{match(session.summary)
						.with(P.nullish, () => (
							<p className="text-sm text-muted-foreground">
								No summary yet — this session is still processing.
							</p>
						))
						.with("", () => (
							<p className="text-sm text-muted-foreground">
								No summary available.
							</p>
						))
						.otherwise((summary) => (
							<p className="text-sm leading-relaxed whitespace-pre-wrap">
								{summary}
							</p>
						))}
				</CardContent>
			</Card>
		</div>
	)
}
