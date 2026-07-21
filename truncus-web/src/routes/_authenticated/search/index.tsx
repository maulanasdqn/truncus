import { createFileRoute } from "@tanstack/react-router"
import { SearchIcon } from "lucide-react"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"
import { z } from "zod"

import { SkeletonLines } from "#/components/loading-skeletons.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { SearchForm } from "./_components/search-form.tsx"
import { SearchResults } from "./_components/search-results.tsx"
import { useSearch } from "./_hooks/use-search.ts"

const SearchPage: FC = (): ReactElement => {
	const search = Route.useSearch()
	const navigate = Route.useNavigate()
	const q = search.q ?? ""
	const kind = search.kind ?? "all"
	const { query } = useSearch({ q, kind: kind === "all" ? undefined : kind })

	const runSearch = (nextQuery: string, nextKind: string): void => {
		void navigate({
			search: {
				q: nextQuery === "" ? undefined : nextQuery,
				kind:
					nextKind === "summary" || nextKind === "chunk"
						? nextKind
						: undefined,
			},
			replace: true,
		})
	}

	return (
		<section aria-labelledby="search-heading" className="flex flex-col gap-6">
			<header className="flex flex-col gap-1">
				<h1
					id="search-heading"
					className="text-2xl font-bold uppercase tracking-label"
				>
					Search
				</h1>
				<p className="text-sm font-light text-muted-foreground">
					Semantic search across every summary and chunk.
				</p>
			</header>
			<SearchForm
				key={`${q}|${kind}`}
				initialQuery={q}
				initialKind={kind}
				onSearch={runSearch}
			/>
			{match({ q: q.trim(), query })
				.with({ q: "" }, () => (
					<EmptyState
						icon={<SearchIcon />}
						title="Search your memory"
						message="Ask in natural language — results are ranked by meaning, not keywords."
					/>
				))
				.with({ query: { status: "pending" } }, () => (
					<SkeletonLines lines={5} />
				))
				.with({ query: { status: "error" } }, ({ query: { error } }) => (
					<p role="alert" className="text-sm text-destructive">
						Search failed: {error.message}
					</p>
				))
				.otherwise(({ query: { data } }) =>
					data ? <SearchResults hits={data} /> : null,
				)}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/search/")({
	validateSearch: z.object({
		q: z.string().optional(),
		kind: z.enum(["summary", "chunk"]).optional(),
	}),
	component: SearchPage,
})
