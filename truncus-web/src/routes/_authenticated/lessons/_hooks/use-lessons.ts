import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import type { UseMutationResult, UseQueryResult } from "@tanstack/react-query"
import { useMemo } from "react"
import { toast } from "sonner"

import { api } from "#/libs/api/client.ts"
import type { TDeleteResponse, TLesson } from "#/libs/api/types.ts"

type TUseLessonsReturn = {
	listQuery: UseQueryResult<TLesson[], Error>
	lessons: readonly TLesson[]
	projects: readonly string[]
	deleteMutation: UseMutationResult<TDeleteResponse, Error, string>
	reflectMutation: UseMutationResult<TDeleteResponse, Error, void>
	pendingDeleteId: string | null
}

export const useLessons = (project: string): TUseLessonsReturn => {
	const queryClient = useQueryClient()
	const listQuery = useQuery({
		queryKey: ["lessons"],
		queryFn: async (): Promise<TLesson[]> => {
			const { lessons } = await api.listLessons({ limit: 200 })
			return lessons
		},
	})
	const invalidate = (): void => {
		void queryClient.invalidateQueries({ queryKey: ["lessons"] })
	}
	const deleteMutation = useMutation({
		mutationFn: (id: string) => api.deleteLesson(id),
		onSuccess: () => {
			toast.success("Lesson deleted")
			invalidate()
		},
		onError: (error) => toast.error(error.message),
	})
	const reflectMutation = useMutation({
		mutationFn: () => api.reflect({ limit: 25 }),
		onSuccess: () =>
			toast.success("Reflecting over recent sessions — refresh in a moment"),
		onError: (error) => toast.error(error.message),
	})

	const all = listQuery.data ?? []
	const projects = useMemo(
		() => [...new Set(all.map((lesson) => lesson.project))].sort(),
		[all],
	)
	const lessons = useMemo(
		() =>
			project === "all"
				? all
				: all.filter((lesson) => lesson.project === project),
		[all, project],
	)

	return {
		listQuery,
		lessons,
		projects,
		deleteMutation,
		reflectMutation,
		pendingDeleteId: deleteMutation.isPending
			? (deleteMutation.variables ?? null)
			: null,
	}
}
