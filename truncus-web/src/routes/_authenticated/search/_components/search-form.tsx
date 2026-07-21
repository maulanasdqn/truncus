import { SearchIcon } from "lucide-react"
import { useState } from "react"
import type { FC, FormEvent, ReactElement } from "react"

import { Button } from "#/components/ui/button.tsx"
import { Input } from "#/components/ui/input.tsx"
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "#/components/ui/select.tsx"

type TProps = {
	initialQuery: string
	initialKind: string
	onSearch: (query: string, kind: string) => void
}

export const SearchForm: FC<TProps> = ({
	initialQuery,
	initialKind,
	onSearch,
}): ReactElement => {
	const [query, setQuery] = useState(initialQuery)
	const [kind, setKind] = useState(initialKind)
	const handleSubmit = (event: FormEvent<HTMLFormElement>): void => {
		event.preventDefault()
		onSearch(query.trim(), kind)
	}
	return (
		<form onSubmit={handleSubmit} className="flex flex-col gap-3 sm:flex-row">
			<div className="relative flex-1">
				<SearchIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
				<Input
					autoFocus
					className="h-11 pl-9 text-base"
					placeholder="Search your memory… e.g. how did I fix the login bug"
					value={query}
					onChange={(event) => setQuery(event.target.value)}
				/>
			</div>
			<Select value={kind} onValueChange={setKind}>
				<SelectTrigger className="h-11 w-full sm:w-40" aria-label="Result kind">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					<SelectItem value="all">All kinds</SelectItem>
					<SelectItem value="summary">Summaries</SelectItem>
					<SelectItem value="chunk">Chunks</SelectItem>
				</SelectContent>
			</Select>
			<Button type="submit" className="h-11" disabled={query.trim() === ""}>
				Search
			</Button>
		</form>
	)
}
