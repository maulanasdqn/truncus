import { useQuery } from "@tanstack/react-query"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { SkeletonLines } from "#/components/loading-skeletons.tsx"
import { Markdown } from "#/components/markdown.tsx"
import {
	Dialog,
	DialogContent,
	DialogHeader,
	DialogTitle,
} from "#/components/ui/dialog.tsx"
import { api } from "#/libs/api/client.ts"

type TProps = {
	project: string
	path: string | null
	onClose: () => void
}

export const NoteViewer: FC<TProps> = ({
	project,
	path,
	onClose,
}): ReactElement => {
	const query = useQuery({
		queryKey: ["knowledge", "content", project, path],
		queryFn: () => api.noteContent(project, path ?? ""),
		enabled: path !== null,
	})
	return (
		<Dialog
			open={path !== null}
			onOpenChange={(open) => {
				if (!open) onClose()
			}}
		>
			<DialogContent className="max-h-[85vh] gap-4 overflow-y-auto sm:max-w-3xl">
				<DialogHeader>
					<DialogTitle className="font-mono text-sm break-all">
						{path}
					</DialogTitle>
				</DialogHeader>
				{match(query)
					.with({ status: "pending" }, () => <SkeletonLines lines={8} />)
					.with({ status: "error" }, ({ error }) => (
						<p role="alert" className="text-sm text-destructive">
							Failed to load: {error.message}
						</p>
					))
					.otherwise(({ data }) =>
						data ? <Markdown content={data.content} /> : null,
					)}
			</DialogContent>
		</Dialog>
	)
}
