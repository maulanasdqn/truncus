import { Link } from "@tanstack/react-router"
import { ChevronRightIcon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { StatusBadge } from "#/components/status-badge.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "#/components/ui/table.tsx"
import { formatDateTime } from "#/libs/format/index.ts"
import type { TSessionMeta } from "#/libs/api/types.ts"

type TProps = {
	sessions: readonly TSessionMeta[]
	onOpen: (id: string) => void
}

export const SessionTable: FC<TProps> = ({
	sessions,
	onOpen,
}): ReactElement => {
	if (sessions.length === 0) {
		return <EmptyState message="No sessions match these filters." />
	}
	return (
		<Table>
			<TableHeader>
				<TableRow>
					<TableHead>Project</TableHead>
					<TableHead>Machine</TableHead>
					<TableHead>Status</TableHead>
					<TableHead>Ended</TableHead>
					<TableHead className="text-right">Chunks</TableHead>
					<TableHead />
				</TableRow>
			</TableHeader>
			<TableBody>
				{sessions.map((session) => (
					<TableRow
						key={session.id}
						className="cursor-pointer"
						onClick={() => onOpen(session.id)}
					>
						<TableCell className="font-medium">{session.project}</TableCell>
						<TableCell className="text-muted-foreground">
							{session.machine}
						</TableCell>
						<TableCell>
							<StatusBadge status={session.status} />
						</TableCell>
						<TableCell className="text-muted-foreground">
							{formatDateTime(session.ended_at)}
						</TableCell>
						<TableCell className="text-right tabular-nums">
							{session.chunk_count}
						</TableCell>
						<TableCell className="text-right">
							<Link
								to="/sessions/$sessionId"
								params={{ sessionId: session.id }}
								aria-label={`Open session ${session.id}`}
								onClick={(event) => event.stopPropagation()}
								className="inline-flex text-muted-foreground hover:text-foreground"
							>
								<ChevronRightIcon className="size-4" />
							</Link>
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	)
}
