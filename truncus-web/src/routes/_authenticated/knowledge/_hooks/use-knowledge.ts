import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import type { UseMutationResult, UseQueryResult } from "@tanstack/react-query"
import { toast } from "sonner"

import { api } from "#/libs/api/client.ts"
import type {
	TNoteMeta,
	TNoteProject,
	TNotesRemoved,
	TSearchHit,
} from "#/libs/api/types.ts"

type TUseKnowledgeReturn = {
	projectsQuery: UseQueryResult<TNoteProject[], Error>
	notesQuery: UseQueryResult<TNoteMeta[], Error>
	searchQuery: UseQueryResult<TSearchHit[], Error>
	clearMutation: UseMutationResult<TNotesRemoved, Error, string>
}

export const useKnowledge = (
	project: string,
	query: string,
): TUseKnowledgeReturn => {
	const queryClient = useQueryClient()
	const projectsQuery = useQuery({
		queryKey: ["knowledge", "projects"],
		queryFn: async (): Promise<TNoteProject[]> =>
			(await api.noteProjects()).projects,
	})
	const notesQuery = useQuery({
		queryKey: ["knowledge", "notes", project],
		queryFn: async (): Promise<TNoteMeta[]> =>
			(await api.listNotes(project)).notes,
		enabled: project !== "",
	})
	const searchQuery = useQuery({
		queryKey: ["knowledge", "search", project, query],
		queryFn: async (): Promise<TSearchHit[]> =>
			(
				await api.knowledgeSearch({
					q: query,
					project: project || undefined,
					limit: 12,
				})
			).hits,
		enabled: query.trim().length > 0 && project !== "",
	})
	const clearMutation = useMutation({
		mutationFn: (target: string) => api.clearNotes(target),
		onSuccess: () => {
			toast.success("Knowledge base cleared")
			void queryClient.invalidateQueries({ queryKey: ["knowledge"] })
		},
		onError: (error) => toast.error(error.message),
	})
	return { projectsQuery, notesQuery, searchQuery, clearMutation }
}
