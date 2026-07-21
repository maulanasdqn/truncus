import { SearchIcon } from "lucide-react"
import type { FC, ReactElement } from "react"

import { Input } from "#/components/ui/input.tsx"
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "#/components/ui/select.tsx"

type TProps = {
	projects: readonly string[]
	project: string
	query: string
	count: number
	onProjectChange: (value: string) => void
	onQueryChange: (value: string) => void
}

export const SessionFilters: FC<TProps> = ({
	projects,
	project,
	query,
	count,
	onProjectChange,
	onQueryChange,
}): ReactElement => (
	<div className="flex flex-col gap-4 sm:flex-row sm:items-center">
		<div className="relative flex-1">
			<SearchIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
			<Input
				type="search"
				placeholder="Filter by project, machine, summary…"
				className="pl-9"
				value={query}
				onChange={(event) => onQueryChange(event.target.value)}
			/>
		</div>
		<Select value={project} onValueChange={onProjectChange}>
			<SelectTrigger className="w-full sm:w-56" aria-label="Filter by project">
				<SelectValue />
			</SelectTrigger>
			<SelectContent>
				<SelectItem value="all">All projects</SelectItem>
				{projects.map((name) => (
					<SelectItem key={name} value={name}>
						{name}
					</SelectItem>
				))}
			</SelectContent>
		</Select>
		<p className="shrink-0 text-xs uppercase tracking-label text-muted-foreground">
			{count} {count === 1 ? "session" : "sessions"}
		</p>
	</div>
)
