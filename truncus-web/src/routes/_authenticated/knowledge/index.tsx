import { createFileRoute } from "@tanstack/react-router"
import { BookOpenIcon, SearchIcon } from "lucide-react"
import { useEffect, useState } from "react"
import type { FC, FormEvent, ReactElement } from "react"
import { match } from "ts-pattern"
import { z } from "zod"

import { SkeletonLines } from "#/components/loading-skeletons.tsx"
import { Button } from "#/components/ui/button.tsx"
import { Card, CardContent } from "#/components/ui/card.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { Input } from "#/components/ui/input.tsx"
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "#/components/ui/select.tsx"
import { ClearKnowledgeDialog } from "./_components/clear-knowledge-dialog.tsx"
import { KnowledgeResults } from "./_components/knowledge-results.tsx"
import { NoteList } from "./_components/note-list.tsx"
import { useKnowledge } from "./_hooks/use-knowledge.ts"

const SYNC_HINT = "truncus vault sync --project <name> <folder>"

const KnowledgePage: FC = (): ReactElement => {
	const search = Route.useSearch()
	const navigate = Route.useNavigate()
	const project = search.project ?? ""
	const query = search.q ?? ""
	const { projectsQuery, notesQuery, searchQuery, clearMutation } =
		useKnowledge(project, query)
	const projects = projectsQuery.data ?? []

	useEffect(() => {
		if (project === "" && projects.length > 0) {
			void navigate({
				search: { project: projects[0].project },
				replace: true,
			})
		}
	}, [project, projects, navigate])

	const [term, setTerm] = useState(query)
	const setProject = (value: string): void => {
		setTerm("")
		void navigate({ search: { project: value }, replace: true })
	}
	const runSearch = (event: FormEvent<HTMLFormElement>): void => {
		event.preventDefault()
		void navigate({
			search: { project, q: term.trim() === "" ? undefined : term.trim() },
			replace: true,
		})
	}

	return (
		<section aria-labelledby="knowledge-heading" className="flex flex-col gap-6">
			<header className="flex flex-col gap-1">
				<h1
					id="knowledge-heading"
					className="text-2xl font-bold uppercase tracking-label"
				>
					Knowledge
				</h1>
				<p className="text-sm font-light text-muted-foreground">
					Reference notes synced from your Obsidian vault. Agents retrieve these
					on demand via knowledge_search — no need to paste them into context.
				</p>
			</header>

			{match(projectsQuery)
				.with({ status: "pending" }, () => <SkeletonLines lines={5} />)
				.with({ status: "error" }, ({ error }) => (
					<p role="alert" className="text-sm text-destructive">
						Failed to load: {error.message}
					</p>
				))
				.otherwise(() =>
					projects.length === 0 ? (
						<EmptyState
							icon={<BookOpenIcon />}
							title="No knowledge synced yet"
							message={`Sync an Obsidian vault folder from the CLI: ${SYNC_HINT}`}
						/>
					) : (
						<div className="flex flex-col gap-6">
							<div className="flex flex-wrap items-center gap-4">
								<Select value={project} onValueChange={setProject}>
									<SelectTrigger className="w-64" aria-label="Project">
										<SelectValue />
									</SelectTrigger>
									<SelectContent>
										{projects.map((item) => (
											<SelectItem key={item.project} value={item.project}>
												{item.project} ({item.note_count})
											</SelectItem>
										))}
									</SelectContent>
								</Select>
								{project !== "" && (
									<ClearKnowledgeDialog
										project={project}
										isClearing={clearMutation.isPending}
										onConfirm={() => clearMutation.mutate(project)}
									/>
								)}
								<code className="ml-auto hidden rounded-none bg-muted px-2 py-1 font-mono text-xs text-muted-foreground sm:block">
									{SYNC_HINT}
								</code>
							</div>

							<form onSubmit={runSearch} className="flex gap-3">
								<div className="relative flex-1">
									<SearchIcon className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
									<Input
										className="h-11 pl-9 text-base"
										placeholder="Ask the knowledge base… e.g. how are orders stored"
										value={term}
										onChange={(event) => setTerm(event.target.value)}
									/>
								</div>
								<Button type="submit" className="h-11" disabled={term.trim() === ""}>
									Search
								</Button>
							</form>

							{query.trim() !== "" && (
								<div className="flex flex-col gap-3">
									<h2 className="text-xs uppercase tracking-label text-muted-foreground">
										Results for “{query}”
									</h2>
									{match(searchQuery)
										.with({ status: "pending" }, () => (
											<SkeletonLines lines={3} />
										))
										.with({ status: "error" }, ({ error }) => (
											<p role="alert" className="text-sm text-destructive">
												Search failed: {error.message}
											</p>
										))
										.otherwise(({ data }) =>
											data ? <KnowledgeResults hits={data} /> : null,
										)}
								</div>
							)}

							<div className="flex flex-col gap-3">
								<h2 className="text-xs uppercase tracking-label text-muted-foreground">
									Notes
								</h2>
								{match(notesQuery)
									.with({ status: "pending" }, () => <SkeletonLines lines={4} />)
									.with({ status: "error" }, ({ error }) => (
										<p role="alert" className="text-sm text-destructive">
											Failed to load: {error.message}
										</p>
									))
									.otherwise(() => (
										<Card>
											<CardContent>
												<NoteList notes={notesQuery.data ?? []} />
											</CardContent>
										</Card>
									))}
							</div>
						</div>
					),
				)}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/knowledge/")({
	validateSearch: z.object({
		project: z.string().optional(),
		q: z.string().optional(),
	}),
	component: KnowledgePage,
})
