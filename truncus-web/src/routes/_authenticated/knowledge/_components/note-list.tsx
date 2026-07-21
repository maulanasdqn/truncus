import { ChevronRightIcon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { EmptyState } from "#/components/ui/empty-state.tsx"
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "#/components/ui/table.tsx"
import { formatRelative } from "#/libs/format/index.ts"
import type { TNoteMeta } from "#/libs/api/types.ts"

type TProps = {
	notes: readonly TNoteMeta[]
	onOpen: (path: string) => void
}

export const NoteList: FC<TProps> = ({ notes, onOpen }): ReactElement => {
	if (notes.length === 0) {
		return <EmptyState message="No notes synced for this project yet." />
	}
	return (
		<Table>
			<TableHeader>
				<TableRow>
					<TableHead>Path</TableHead>
					<TableHead>Title</TableHead>
					<TableHead className="text-right">Chunks</TableHead>
					<TableHead>Updated</TableHead>
					<TableHead />
				</TableRow>
			</TableHeader>
			<TableBody>
				{notes.map((note) => (
					<TableRow
						key={note.path}
						className="cursor-pointer"
						onClick={() => onOpen(note.path)}
					>
						<TableCell className="font-mono text-xs">{note.path}</TableCell>
						<TableCell className="font-medium">{note.title}</TableCell>
						<TableCell className="text-right tabular-nums">
							{note.chunk_count}
						</TableCell>
						<TableCell className="text-muted-foreground">
							{formatRelative(note.updated_at)}
						</TableCell>
						<TableCell className="text-right">
							<ChevronRightIcon className="inline size-4 text-muted-foreground" />
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	)
}
