import { Trash2Icon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { Button } from "#/components/ui/button.tsx"
import {
	Dialog,
	DialogClose,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from "#/components/ui/dialog.tsx"

type TProps = {
	project: string
	isDeleting: boolean
	onConfirm: () => void
}

export const DeleteSessionDialog: FC<TProps> = ({
	project,
	isDeleting,
	onConfirm,
}): ReactElement => (
	<Dialog>
		<DialogTrigger asChild>
			<Button variant="outline" size="sm" loading={isDeleting} loadingText="Deleting…">
				<Trash2Icon className="size-4" />
				Delete
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle className="uppercase tracking-label">
					Delete session
				</DialogTitle>
				<DialogDescription>
					This permanently removes this {project} session from D1, R2 and
					Vectorize. This cannot be undone.
				</DialogDescription>
			</DialogHeader>
			<DialogFooter>
				<DialogClose asChild>
					<Button variant="outline">Cancel</Button>
				</DialogClose>
				<DialogClose asChild>
					<Button variant="destructive" onClick={onConfirm}>
						Delete session
					</Button>
				</DialogClose>
			</DialogFooter>
		</DialogContent>
	</Dialog>
)
