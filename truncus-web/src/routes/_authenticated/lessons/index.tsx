import { createFileRoute } from "@tanstack/react-router"
import { SparklesIcon } from "lucide-react"
import { useEffect, useState } from "react"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"
import { z } from "zod"

import { SkeletonLines } from "#/components/loading-skeletons.tsx"
import { Button } from "#/components/ui/button.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { Pagination } from "#/components/ui/pagination.tsx"
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "#/components/ui/select.tsx"
import { LessonCard } from "./_components/lesson-card.tsx"
import { useLessons } from "./_hooks/use-lessons.ts"

const PAGE_SIZE = 10

const LessonsPage: FC = (): ReactElement => {
	const search = Route.useSearch()
	const navigate = Route.useNavigate()
	const project = search.project ?? "all"
	const {
		listQuery,
		lessons,
		projects,
		deleteMutation,
		reflectMutation,
		pendingDeleteId,
	} = useLessons(project)

	const [page, setPage] = useState(1)
	useEffect(() => {
		setPage(1)
	}, [project])
	const pageCount = Math.max(1, Math.ceil(lessons.length / PAGE_SIZE))
	const currentPage = Math.min(page, pageCount)
	const pageItems = lessons.slice(
		(currentPage - 1) * PAGE_SIZE,
		currentPage * PAGE_SIZE,
	)

	const setProject = (value: string): void => {
		void navigate({
			search: value === "all" ? {} : { project: value },
			replace: true,
		})
	}

	return (
		<section aria-labelledby="lessons-heading" className="flex flex-col gap-6">
			<header className="flex flex-wrap items-end justify-between gap-4">
				<div className="flex flex-col gap-1">
					<h1
						id="lessons-heading"
						className="text-2xl font-bold uppercase tracking-label"
					>
						Lessons
					</h1>
					<p className="text-sm font-light text-muted-foreground">
						Durable learnings Truncus distilled from your sessions — reinforced
						as they recur, and fed back into new sessions.
					</p>
				</div>
				<Button
					variant="outline"
					loading={reflectMutation.isPending}
					loadingText="Reflecting…"
					onClick={() => reflectMutation.mutate()}
				>
					<SparklesIcon className="size-4" />
					Reflect recent
				</Button>
			</header>

			<div className="flex items-center gap-4">
				<Select value={project} onValueChange={setProject}>
					<SelectTrigger className="w-56" aria-label="Filter by project">
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
				<p className="text-xs uppercase tracking-label text-muted-foreground">
					{lessons.length} {lessons.length === 1 ? "lesson" : "lessons"}
				</p>
			</div>

			{match(listQuery)
				.with({ status: "pending" }, () => <SkeletonLines lines={6} />)
				.with({ status: "error" }, ({ error }) => (
					<p role="alert" className="text-sm text-destructive">
						Failed to load: {error.message}
					</p>
				))
				.otherwise(() =>
					lessons.length === 0 ? (
						<EmptyState
							icon={<SparklesIcon />}
							title="No lessons yet"
							message="Reflect over recent sessions to distill your first lessons — or they'll appear automatically as new sessions are captured."
						/>
					) : (
						<div className="flex flex-col gap-4">
							<ul className="flex flex-col gap-4">
								{pageItems.map((lesson) => (
									<li key={lesson.id}>
										<LessonCard
											lesson={lesson}
											isDeleting={pendingDeleteId === lesson.id}
											onDelete={(id) => deleteMutation.mutate(id)}
										/>
									</li>
								))}
							</ul>
							<Pagination
								page={currentPage}
								pageCount={pageCount}
								onPageChange={setPage}
							/>
						</div>
					),
				)}
		</section>
	)
}

export const Route = createFileRoute("/_authenticated/lessons/")({
	validateSearch: z.object({
		project: z.string().optional(),
	}),
	component: LessonsPage,
})
