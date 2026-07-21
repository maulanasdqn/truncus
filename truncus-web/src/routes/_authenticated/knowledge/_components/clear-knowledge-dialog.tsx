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
	isClearing: boolean
	onConfirm: () => void
}

export const ClearKnowledgeDialog: FC<TProps> = ({
	project,
	isClearing,
	onConfirm,
}): ReactElement => (
	<Dialog>
		<DialogTrigger asChild>
			<Button variant="outline" size="sm" loading={isClearing} loadingText="Clearing…">
				<Trash2Icon className="size-4" />
				Clear
			</Button>
		</DialogTrigger>
		<DialogContent>
			<DialogHeader>
				<DialogTitle className="uppercase tracking-label">
					Clear knowledge base
				</DialogTitle>
				<DialogDescription>
					This removes all synced notes and embeddings for {project}. Re-run
					truncus vault sync to restore them.
				</DialogDescription>
			</DialogHeader>
			<DialogFooter>
				<DialogClose asChild>
					<Button variant="outline">Cancel</Button>
				</DialogClose>
				<DialogClose asChild>
					<Button variant="destructive" onClick={onConfirm}>
						Clear knowledge
					</Button>
				</DialogClose>
			</DialogFooter>
		</DialogContent>
	</Dialog>
)
